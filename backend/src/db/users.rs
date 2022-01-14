use crate::models::NewUser;
use crate::schema::users;
use diesel::prelude::*;
use diesel::result;

// #################### Create new User ##################### //
pub fn create_user(
    connection: PgConnection,
    username: String,
    password: String,
    nickname: String,
) -> Result<(), Option<String>> {
    let new_user = NewUser {
        username,
        password,
        nickname,
    };

    let result = diesel::insert_into(users::table)
        .values(&new_user)
        .execute(&connection);

    match result {
        Ok(_) => Ok(()),
        Err(error) => match error {
            result::Error::DatabaseError(e, _) => match e {
                result::DatabaseErrorKind::UniqueViolation => Err(Some(
                    String::from("Username already exists in database. Use a different username"), // show this message to client
                )),
                _ => {
                    println!("{:?}", e);
                    Err(None)
                } // TODO: I will handle more errors laters.
            },
            _ => {
                println!("{:?}", error);
                Err(None)
            }
        },
    }
}
