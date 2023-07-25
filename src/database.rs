use crate::models;

use super::diesel::prelude::*;
use super::models::{NewMessage, QueryMessage};
use super::schema::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use diesel::result::Error;
use mongodb::{Client, Collection};
use mongodb::options::ClientOptions;
use std::env;
use mongodb::error;

pub fn establish_connection() -> SqliteConnection{
  dotenv().ok();

  let database_url = env::var("DATABASE_URL")
    .expect("DATABASE_URL must be set");
  SqliteConnection::establish(&database_url)
  .expect(&format!("Error Connecting to {}", database_url))
}

pub fn post_message(new_message: NewMessage) -> Result<usize, Error>{
  let mut conn = establish_connection();

  let result = diesel::insert_into(messages::table)
    .values(new_message)
    .execute(&mut conn)?;
  Ok(result)
}

pub async fn establish_connection_mongodb()-> Result<Collection<models::Message>, error::Error> {
  let mongodb_url = env::var("MONGODB_URL")
    .expect("MONGODB_URL must be set");
  
  let mut client_options = ClientOptions::parse(mongodb_url).await?;
  client_options.app_name = Some("github.io.backend".to_string());

  let client = Client::with_options(client_options)?;

  let collections = client.database("github-io")
    .collection::<models::Message>("messages");

  return Ok(collections);
}

pub async fn insert_message_from_web(info: models::Message)-> Result<usize, Error>{
  let new_message: NewMessage = NewMessage{
    messenger_name: info.name,
    messenger_email: info.email,
    message_description: info.description
  };
  post_message(new_message)
}

pub async fn get_message_using_email(sender: &String)->Vec<QueryMessage>{
  let mut conn = establish_connection();

  use super::schema::messages::dsl::*;

  let result = messages
    .filter(messenger_email.eq(sender))
    .load::<QueryMessage>(&mut conn).unwrap();
  
  return result;
}

pub fn delete_message(msg_id: i32) -> Result<(), Error>{
  let mut conn = establish_connection();

  use super::schema::messages::dsl::*;

  match  diesel::delete(
    messages.filter(id.eq(msg_id)))
    .execute(&mut conn){
      Ok(_) =>Ok(()),
      Err(e) =>{
        Err(e)
    }
  }
}