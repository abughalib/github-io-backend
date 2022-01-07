pub mod database;
pub mod schema;
pub mod models;

use actix_web::{
  middleware, web, App, HttpRequest, Result, Responder,
  http::header, HttpResponse, HttpServer, get
};
use actix_cors::Cors;
use models::QueryMessage;

#[macro_use]
extern crate diesel;
extern crate serde;

#[get("/")]
async fn index(_req: HttpRequest)->HttpResponse{
  HttpResponse::Ok().body("Nothing to see here!")
}

async fn message(info: web::Json<models::Message>)->HttpResponse{

  let result = database::insert_message_from_web(info.0).await;

  match result{
    Ok(_)=>{
      HttpResponse::Ok().body("Message Sent")
    },
    Err(_)=>{
      // Later to Log Errors
      HttpResponse::BadRequest().body("Unknow Error!")
    }
  }
}

async fn _get_message_using_email(info: web::Json<models::QueryUsingEmail>)-> Result<impl Responder>{
  let msgs: Vec<QueryMessage> = database::get_message_using_email(&info.email).await;
  Ok(web::Json(msgs))
}

#[actix_web::main]
async fn main()->std::io::Result<()>{
  std::env::set_var("RUST_LOG", "actix_web=info");
  env_logger::init();

  let port: u16 = 8088;
  
  HttpServer::new(|| {
    let cors = Cors::default()
    .allow_any_origin()
    .allowed_methods(vec!["GET", "POST"])
    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
    .allowed_header(header::CONTENT_TYPE)
    .max_age(3600);

    App::new()
      .wrap(middleware::Logger::default())
      .wrap(cors)
      .data(web::JsonConfig::default().limit(4096))
      .service(index)
      .service(web::resource("/message")
      .route(web::post().to(message)))
  })
  .bind(("0.0.0.0", port))?
  .run()
  .await
}

#[cfg(test)]
mod tests{
  use crate::models::NewMessage;

use super::*;
  use actix_web::dev::Service;
  use actix_web::{http, test, App};

  #[actix_rt::test]
  async fn test_index()->Result<(), actix_web::Error>{
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
  async fn test_database(){
    let new_msg = NewMessage{
      messenger_name: String::from("dafklasdjlfk"),
      messenger_email: String::from("dj2131sdfa@gmail.com"),
      message_description: String::from(
        "Testing..."
      )
    };
    let id_index: i32 = database::post_message(new_msg).unwrap() as i32;
    use std::time;
    use std::thread;

    let sleep_duration = time::Duration::from_millis(500);
    println!("Sleeping Thread for Database");
    thread::sleep(sleep_duration);
    assert!(database::delete_message(id_index).is_ok())
  }

  #[actix_rt::test]
  async fn test_message()->Result<(), actix_web::Error>{

    use std::time;
    use std::thread;
    let sleep_duration = time::Duration::from_millis(500);
    thread::sleep(sleep_duration);
    println!("Sleeping Thread for Database");

    let mut app = test::init_service(
      App::new()
      .service(web::resource("/message")
      .route(web::post().to(message)))
    ).await;

    let req = test::TestRequest::post()
      .uri("/message")
      .set_json(&models::Message{
        name: "Abu".to_owned(),
        email: "abugh@protonmail.com".to_owned(),
        description: "msg".to_owned(),
      })
      .to_request();

    let resp = app.call(req).await.unwrap();

    assert_eq!(resp.status(), http::StatusCode::OK);

    let resp_body = match resp.response().body().as_ref(){
      Some(actix_web::body::Body::Bytes(bytes)) => bytes,
      _ => panic!("Response Error"),
    };

    //assert_eq!(resp_body, r##"{"name":"Abu Ghalib","email":"abugh@protonmail.com","message":"msg"}"##);
    assert_eq!(resp_body, r##"Message Sent"##);
    println!("{:?}", resp_body);

    let msgs: Vec<QueryMessage> = database::get_message_using_email(
      &String::from("abugh@protonmail.com")
    ).await;

    for idx in &msgs{
      assert!(database::delete_message(idx.id).is_ok());
      thread::sleep(sleep_duration)
    }

    Ok(())
  }
}