-- Your SQL goes here
CREATE TABLE messages (
    id SERIAL PRIMARY KEY,
    msg TEXT NOT NULL,
    user_id  SERIAL references users(id) ON DELETE CASCADE,
    room_id SERIAL references rooms(id) ON DELETE CASCADE

)   