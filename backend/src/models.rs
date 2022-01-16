use crate::schema::*;
use serde::{Deserialize, Serialize};

// ################### users ################# //
#[derive(Identifiable, Queryable, Associations, Debug, Clone, Deserialize, Serialize)]
pub struct User {
    pub id: i32,
    pub nickname: String,
    pub username: String,
    pub password: String,
    pub img_url: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub nickname: String,
    pub username: String,
    pub password: String,
    pub img_url: String,
}

// ################### rooms ################# //
#[derive(Identifiable, Queryable, Associations, Debug, Clone, Deserialize, Serialize)]
pub struct Room {
    pub id: i32,
    // TODO: Later on I will give another field called `uniqe_name`;
    pub nickname: String,
    pub img_url: String,
}

#[derive(Insertable)]
#[table_name = "rooms"]
pub struct NewRoom {
    pub nickname: String,
    pub img_url: String,
}

#[derive(Identifiable, Queryable, Associations, Debug, Clone, Deserialize, Serialize)]
pub struct RoomsUser {
    pub id: i32,
    pub user_id: i32,
    pub room_id: i32,
    pub accepted: bool, // if the User is trying to join a room than the room's members accepted this User or not.  If the User is trying to create his/her own room, this field should be `true` by default
}

#[derive(Insertable)]
#[table_name = "rooms_users"]
pub struct NewRoomsUser {
    pub user_id: i32,
    pub room_id: i32,
    pub accepted: bool,
}
