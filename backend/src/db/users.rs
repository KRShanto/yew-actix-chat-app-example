use crate::models::{NewUser, User};
use crate::schema::users;
use diesel::prelude::*;
use diesel::result;

// ************************************************************************* //
// ########################### Create new User ############################# //
// ************************************************************************* //
pub fn create_user(
    connection: PgConnection,
    username: String,
    password: String,
    nickname: String,
    img_url: String,
) -> Result<User, Option<String>> {
    let new_user = NewUser {
        username,
        password,
        nickname,
        img_url,
    };

    let result = diesel::insert_into(users::table)
        .values(&new_user)
        .get_result::<User>(&connection);

    match result {
        Ok(user) => Ok(user),
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

// ************************************************************************* //
// ###################  Show a user is present or not  ##################### //
// ************************************************************************* //
pub fn is_user_present(user_id: i32, connection: &PgConnection) -> Result<bool, ()> {
    use crate::schema::users::dsl::*;

    // trying to load the user
    let results = users.find(user_id).first::<User>(connection);

    match results {
        Ok(_user) => Ok(true),
        Err(error) => match error {
            result::Error::NotFound => Ok(false),
            _ => {
                println!("Error occur when finding the User: {}", error);

                Err(())
            }
        },
    }
}
