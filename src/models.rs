use super::serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable};
use super::schema::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct Message{
  pub name: String,
  pub email: String,
  pub description: String,
}

#[derive(Deserialize)]
pub struct QueryUsingEmail{
  pub email: String
}

#[derive(Insertable)]
#[table_name = "messages"]
pub struct NewMessage{
  pub messenger_name: String,
  pub messenger_email: String,
  pub message_description: String,
}

#[derive(Queryable, QueryableByName, Serialize)]
#[table_name = "messages"]
pub struct QueryMessage{
  pub id: i32,
  pub messenger_name: String,
  pub messenger_email: String,
  pub message_description: String,
}