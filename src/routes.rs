use crate::{database, models};
use actix_web::{get, web, HttpRequest, HttpResponse, Responder, Result};

#[get("/")]
pub async fn index(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().body("Nothing to see here!")
}

pub async fn message(info: web::Json<models::Message>) -> HttpResponse {
    let result = database::insert_message_from_web(info.0).await;
    match result {
        Ok(_) => HttpResponse::Ok().body("Message Sent"),
        Err(_) => {
            // Later to Log Errors
            HttpResponse::BadRequest().body("Unknow Error!")
        }
    }
}

pub async fn message_mongoasync(info: web::Json<models::Message>) -> HttpResponse {
    let collections = database::establish_connection_mongodb()
        .await
        .expect("Failed to Connect to Mongodb");

    let result = collections.insert_one(info.0, None).await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Message Sent"),
        Err(_) => {
            // Later to Log Errors
            HttpResponse::BadRequest().body("Unknow Error!")
        }
    }
}

pub async fn _get_message_using_email(
    info: web::Json<models::QueryUsingEmail>,
) -> Result<impl Responder> {
    let msgs: Vec<models::QueryMessage> = database::get_message_using_email(&info.email).await;
    Ok(web::Json(msgs))
}
