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
