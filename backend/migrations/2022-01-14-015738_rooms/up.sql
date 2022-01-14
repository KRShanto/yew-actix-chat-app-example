-- Your SQL goes here
CREATE TABLE rooms (
  id SERIAL PRIMARY KEY,
  nickname VARCHAR(255) NOT NULL,
  img_url VARCHAR(255) NOT NULL
)