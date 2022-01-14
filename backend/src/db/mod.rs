// This module will re-export some functions to work with database
pub mod connection;
pub mod users;

pub use connection::establish_connection;
pub use users::create_user;
