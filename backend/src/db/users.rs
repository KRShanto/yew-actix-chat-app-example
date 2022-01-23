use crate::models::{NewUser, User};
use crate::schema::users::dsl::*;

use colored::*;
use diesel::prelude::*;
use diesel::result;

// ************************************************************************* //
// ########################### Create new User ############################# //
// ************************************************************************* //
pub fn create_user(
    connection: PgConnection,
    argu_username: String,
    argu_password: String,
    argu_nickname: String,
    argu_img_url: String,
) -> Result<User, Option<String>> {
    let new_user = NewUser {
        username: argu_username,
        password: argu_password,
        nickname: argu_nickname,
        img_url: argu_img_url,
    };

    let result = diesel::insert_into(crate::schema::users::table)
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

// ************************************************************************* //
// ################### Get a user based on his/her id ###################### //
// ************************************************************************* //
pub fn get_a_user_from_id(user_id: i32, connection: &PgConnection) -> Result<User, ()> {
    let result: Result<User, _> = users.filter(id.eq(user_id)).get_result::<User>(connection);

    match result {
        Ok(user) => Ok(user),
        Err(error) => {
            println!(
                "{}",
                format!("An error occured when finding a user. Error: {}", error).red()
            );
            Err(())
        }
    }
}

// ************************************************************************* //
// ########################### validate a User ############################# //
// ************************************************************************* //
pub fn validate_user(
    argu_username: String,
    argu_password: String,
    connection: &PgConnection,
) -> Result<Option<User>, ()> {
    // if return_type == Ok(Some) user_is_valid()
    // else if return_type == Ok(None) user_is_not_valid()
    // else there_is_an_error()

    let result: Result<User, result::Error> = users
        .filter(username.eq(argu_username))
        .filter(password.eq(argu_password))
        .first(connection);

    match result {
        Ok(user) => {
            // user is valid
            Ok(Some(user))
        }
        Err(error) => {
            match error {
                result::Error::NotFound => {
                    // user is not valid
                    Ok(None)
                }
                _ => {
                    println!("{}", format!("{}", error).red().bold());
                    Err(())
                }
            }
        }
    }
}
