use crate::db::establish_connection;
use crate::models::{Message, NewMessage};
// use crate::schema::messages;
use crate::schema::messages::dsl::*;
use colored::*;
use diesel::prelude::*;

// ************************************************************************* //
// ############################# Create new Message ########################### //
// ************************************************************************* //
pub fn create_message(
    argu_msg: String,
    argu_user_id: i32,
    argu_room_id: i32,
) -> Result<Message, ()> {
    // NOTE: You need to check if the user and room id is valid before calling this function
    let results: Result<Message, _> = diesel::insert_into(crate::schema::messages::table)
        .values(&NewMessage {
            msg: argu_msg,
            user_id: argu_user_id,
            room_id: argu_room_id,
        })
        .get_result::<Message>(&establish_connection());

    match results {
        Ok(value) => Ok(value),
        Err(error) => {
            println!(
                "{}",
                format!("An error occurred while creating new messages: {}", error).red()
            );
            Err(())
        }
    }
}

// ************************************************************************* //
// ############################# Get all messages for a room ######################## //
// ************************************************************************* //
pub fn get_all_messages_for_a_room(argu_room_id: i32) -> Result<Vec<Message>, ()> {
    // NOTE: You need to check if the user and room id is valid before calling this function
    let result: Result<Vec<Message>, _> = messages
        .filter(room_id.eq(argu_room_id))
        .get_results::<Message>(&establish_connection());

    match result {
        Ok(message) => Ok(message),
        Err(error) => {
            println!(
                "{}",
                format!("An error occured when fetching messages: {}", error).red()
            );
            Err(())
        }
    }
}
