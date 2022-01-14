-- Your SQL goes here
CREATE TABLE rooms_users (
    id SERIAL PRIMARY KEY, 
    user_id  SERIAL references users(id) ON DELETE CASCADE,
    room_id SERIAL references rooms(id) ON DELETE CASCADE,
    accepted BOOLEAN NOT NULL DEFAULT 'f'
)