use crate::schema::*;

// ################### users ################# //
#[derive(Identifiable, Queryable, Associations, Debug, Clone)]
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
