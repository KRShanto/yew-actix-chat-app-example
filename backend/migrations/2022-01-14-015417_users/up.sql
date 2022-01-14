-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  nickname VARCHAR(255) NOT NULL,
  username VARCHAR(100) NOT NULL UNIQUE,
  password VARCHAR(100) NOT NULL,
  img_url VARCHAR(255) NOT NULL
)