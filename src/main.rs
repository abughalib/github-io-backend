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

async fn message_mongoasync(info: web::Json<models::Message>)->HttpResponse{
  let collections = database::establish_connection_mongodb().await
    .expect("Failed to Connect to Mongodb");

  let result = collections.insert_one(info.0, None).await;

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

  let pattern = std::env::args().nth(1)
    .expect("mention database argument as sqlite or mongodb");

  let port: u16 = 8088;
  
  HttpServer::new(move || {
    let cors = Cors::default()
    .allow_any_origin()
    .allowed_methods(vec!["GET", "POST"])
    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
    .allowed_header(header::CONTENT_TYPE)
    .max_age(3600);

    if pattern == "mongodb" {
      App::new()
      .wrap(middleware::Logger::default())
      .wrap(cors)
      .app_data(web::JsonConfig::default().limit(4096))
      .service(index)
      .service(web::resource("/message")
      .route(web::post().to(message_mongoasync)))
    }else {
      App::new()
      .wrap(middleware::Logger::default())
      .wrap(cors)
      .app_data(web::JsonConfig::default().limit(4096))
      .service(index)
      .service(web::resource("/message")
      .route(web::post().to(message)))
    }
  })
  .bind(("0.0.0.0", port))?
  .run()
  .await
}

#[cfg(test)]
mod tests{
  use crate::models::NewMessage;

use super::*;
  use actix_web::{http, test, App};

  #[actix_rt::test]
  async fn test_index()->Result<(), actix_web::Error>{
    let app = test::init_service(
      App::new().service(index),
    )
    .await;

    let req = test::TestRequest::get().to_request();
    let res = test::call_service(&app, req).await;
    assert_eq!(res.status(), http::StatusCode::OK);

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

    let app = test::init_service(
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

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), http::StatusCode::OK);

    //assert_eq!(resp_body, r##"{"name":"Abu Ghalib","email":"abugh@protonmail.com","message":"msg"}"##);

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