use crate::schema::*;

// ################### users ################# //
#[derive(Identifiable, Queryable, Associations, Debug, Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub password: String,
}
