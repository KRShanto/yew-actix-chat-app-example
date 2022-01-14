#![allow(dead_code, unused)]
use crate::db::users::is_user_present;
use crate::models::{NewRoom, NewRoomsUser, Room, RoomsUser};
use crate::schema::{rooms, rooms_users};
use diesel::prelude::*;
use diesel::result;

// ************************************************************************* //
// ############################# Create new Room ########################### //
// ************************************************************************* //
pub fn create_room(
    connection: &PgConnection,
    nickname: String,
    img_url: String,
) -> Result<Room, diesel::result::Error> {
    // TODO: I will return Result<> from this function and try to handle some errors

    let new_room = NewRoom { nickname, img_url };

    let result = diesel::insert_into(rooms::table)
        .values(&new_room)
        .get_result::<Room>(connection);

    match result {
        Ok(room) => Ok(room),
        Err(error) => Err(error),
    }
}

// ************************************************************************* //
// ########################### Add User in a Room ########################## //
// ************************************************************************* //
pub fn add_user_into_room(
    user_id: i32,
    room_id: i32,
    connection: PgConnection,
    accepted: bool,
) -> Result<(), Option<String>> {
    if let Ok(value) = is_user_present(user_id, &connection) {
        match value {
            true => {
                // Create a new `rooms_users` row and add this user's id inside that
                let new_rooms_users = NewRoomsUser {
                    room_id,
                    user_id,
                    accepted,
                };

                diesel::insert_into(rooms_users::table)
                    .values(&new_rooms_users)
                    .execute(&connection)
                    .unwrap();
                Ok(())
            }
            false => {
                // User is not found
                Err(Some("".to_owned())) // ignore the string, the porpose of returning an Option is that to figure out wheather its an server error or username not found.
                                         // TODO: I will consider a better approach later
            }
        }
    } else {
        println!("Some error occurs when trying the call the `is_user_present` function.");
        Err(None) // it's probebly a server side error
    }
    // TODO: I will provide some helpful errors later
}