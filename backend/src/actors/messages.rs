// use actix_cors::Cors;
// use actix_web::{
//     body::Body,
//     get,
//     http::{header, StatusCode},
//     middleware::Logger,
//     post,
//     web::Json,
//     App, HttpResponse, HttpServer,
// };
// use std::collections::HashMap;
// use std::time::{Duration, Instant};

use actix::prelude::*;
// use actix_files as fs;
// use actix_web::{middleware, web, Error, HttpRequest};
// use actix_web_actors::ws;
// use rand::{thread_rng, Rng};

// The Message for sending messages to clients
#[derive(Message)]
#[rtype(result = "()")]
pub struct SendMessage {
    pub message: String,
    pub current_room_id: i32, // actor's current room
}

// The message when the Actor `ChatSession` first statrted. This message is for `ChatServer`. This will add the recipient of the Actor `ChatSession` in server's `addr_of_all_other_actors` field.
#[derive(Message)]
#[rtype(result = "()")]
pub struct Join {
    pub addr: Recipient<SendMessage>, // address of `ChatSession`
}

// When a message comes from a chat room, this message will send to the `ChatServer` to pass all other actors.
#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientSendMessage {
    pub message: String,
    pub current_room_id: i32,
}
