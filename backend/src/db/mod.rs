// This module will re-export some functions to work with database
pub mod connection;
pub mod rooms;
pub mod users;

pub use connection::establish_connection;
pub use rooms::{add_user_into_room, create_room, get_all_rooms_for_a_user, is_room_present};
pub use users::{create_user, is_user_present};
