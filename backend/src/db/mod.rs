// This module will re-export some functions to work with database
pub mod connection;
pub mod messages;
pub mod rooms;
pub mod users;

pub use connection::establish_connection;
pub use messages::{create_message, get_all_messages_for_a_room};
pub use rooms::{
    add_user_into_room, create_room, get_all_rooms_for_a_user, get_all_users_from_a_room,
    is_room_present,
};
pub use users::{create_user, get_a_user_from_id, is_user_present};
