-- Your SQL goes here
CREATE TABLE messages(
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  messenger_name VARCHAR(255) NOT NULL,
  messenger_email VARCHAR(255) NOT NULL,
  message_description TEXT NOT NULL
);