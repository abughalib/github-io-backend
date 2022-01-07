use crate::models;

use super::diesel::prelude::*;
use super::models::{NewMessage, QueryMessage};
use super::schema::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use diesel::result::Error;
use std::env;

pub fn establish_connection() -> SqliteConnection{
  dotenv().ok();

  let database_url = env::var("DATABASE_URL")
    .expect("DATABASE_URL must be set");
  SqliteConnection::establish(&database_url)
  .expect(&format!("Error Connecting to {}", database_url))
}

pub fn post_message(new_message: NewMessage) -> Result<(), Error>{
  let conn = establish_connection();

  let _result = diesel::insert_into(messages::table)
    .values(new_message)
    .execute(&conn)?;
  Ok(())
}

pub async fn insert_message_from_web(info: models::Message){
  let new_message: NewMessage = NewMessage{
    messenger_name: info.name,
    messenger_email: info.email,
    message_description: info.description
  };
  post_message(new_message).unwrap();
}

pub async fn get_message_using_email(sender: &String)->Vec<QueryMessage>{
  let conn = establish_connection();

  use super::schema::messages::dsl::*;

  let result = messages
    .filter(messenger_email.eq(sender))
    .load::<QueryMessage>(&conn).unwrap();
  
  return result;
}

pub fn delete_message(msg_id: i32) -> Result<(), Error>{
  let conn = establish_connection();

  use super::schema::messages::dsl::*;

  match  diesel::delete(
    messages.filter(id.eq(msg_id)))
    .execute(&conn){
      Ok(_) =>Ok(()),
      Err(e) =>{
        Err(e)
    }
  }
}