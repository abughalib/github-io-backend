use actix_web::{
  error, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer, get
};
use futures::StreamExt;
use json::JsonValue;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Info{
  name: String,
  email: String,
  message: String,
}

#[get("/")]
async fn index(_req: HttpRequest)->HttpResponse{
  HttpResponse::Ok().body("Nothing to see here!")
}

async fn message(info: web::Json<Info>)->HttpResponse{
  // println!("model: {:?}", &info);
  HttpResponse::Ok().json(info.0)
}

async fn extract_item(info: web::Json<Info>, req: HttpRequest)->HttpResponse{
  println!("Request: {:?}", req);
  println!("Model: {:?}", info);

  HttpResponse::Ok().json(info.0)
}

const MAX_SIZE: usize = 262_144;

async fn message_parse(mut payload: web::Payload)->Result<HttpResponse, Error>{
  let mut body = web::BytesMut::new();
  while let Some(chunk) = payload.next().await{
    let chunk = chunk?;
    if (body.len() + chunk.len()) > MAX_SIZE{
      return Err(error::ErrorBadRequest("OverFlow payload size"));
    }
    body.extend_from_slice(&chunk);
  }

  let obj = serde_json::from_slice::<Info>(&body)?;
  Ok(HttpResponse::Ok().json(obj))
}

async fn message_parse_json(body: web::Bytes)->Result<HttpResponse, Error>{
  let resp = json::parse(std::str::from_utf8(&body).unwrap());
  let injson: JsonValue = match resp{
    Ok(v)=>v,
    Err(e) =>json::object!{"err" => e.to_string()},
  };
  Ok(HttpResponse::Ok()
    .content_type("application/json")
    .body(injson.dump()))
}

#[actix_web::main]
async fn main()->std::io::Result<()>{
  std::env::set_var("RUST_LOG", "actix_web=info");
  env_logger::init();
  
  HttpServer::new(|| {
    App::new()
      .wrap(middleware::Logger::default())
      .data(web::JsonConfig::default().limit(4096))
      .service(index)
      .service(web::resource("/message").route(web::post().to(message)))
      .service(
        web::resource("/message/extract")
          .data(web::JsonConfig::default().limit(1024))
          .route(web::post().to(extract_item)),
      )
      .service(web::resource("/message/parse").route(web::post().to(message_parse)))
      .service(web::resource("/message/parse_json").route(web::post().to(message_parse_json)))
  })
  .bind("127.0.0.1:8088")?
  .run()
  .await
}

#[cfg(test)]
mod tests{
  use super::*;
  use actix_web::dev::Service;
  use actix_web::{http, test, App};

  #[actix_rt::test]
  async fn test_index()->Result<(), Error>{
    let mut app = test::init_service(
      App::new().service(index),
    )
    .await;

    let req = test::TestRequest::get().to_request();
    let res = app.call(req).await.unwrap();

    assert_eq!(res.status(), http::StatusCode::OK);
    
    let res_body = match res.response().body().as_ref(){
      Some(actix_web::body::Body::Bytes(bytes))=>bytes,
      _=>panic!("Response Error"),
    };

    assert_eq!(res_body, "Nothing to see here!");

    Ok(())
    
  }

  #[actix_rt::test]
  async fn test_message()->Result<(), Error>{
    let mut app = test::init_service(
      App::new()
      .service(web::resource("/message").route(web::post().to(message)))
    ).await;

    let req = test::TestRequest::post()
      .uri("/message")
      .set_json(&Info{
        name: "Abu Ghalib".to_owned(),
        email: "abugh@protonmail.com".to_owned(),
        message: "msg".to_owned(),
      })
      .to_request();

      let resp = app.call(req).await.unwrap();

      assert_eq!(resp.status(), http::StatusCode::OK);

      let resp_body = match resp.response().body().as_ref(){
        Some(actix_web::body::Body::Bytes(bytes)) => bytes,
        _ => panic!("Response Error"),
      };

      assert_eq!(resp_body, r##"{"name":"Abu Ghalib","email":"abugh@protonmail.com","message":"msg"}"##);

      println!("{:?}", resp_body);

    Ok(())
  }

}