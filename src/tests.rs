#[cfg(test)]
mod tests{
  use crate::models::NewMessage;

  use crate::{database, routes, models};
  use actix_web::{http, test, App, web};

  #[actix_rt::test]
  async fn test_index()->Result<(), actix_web::Error>{
    let app = test::init_service(
      App::new().service(routes::index),
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
      .route(web::post().to(routes::message)))
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

    let msgs: Vec<models::QueryMessage> = database::get_message_using_email(
      &String::from("abugh@protonmail.com")
    ).await;

    for idx in &msgs{
      assert!(database::delete_message(idx.id).is_ok());
      thread::sleep(sleep_duration)
    }

    Ok(())
  }
}
