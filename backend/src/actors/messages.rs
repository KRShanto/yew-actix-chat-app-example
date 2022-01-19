use actix::prelude::*;

// TODO: I will write better comments later;

#[derive(PartialEq, Clone)]
pub enum SendType {
    Singular, // send only specific client
    Plural,   // send all clients
}

// The Message for sending messages to clients
#[derive(Message)]
#[rtype(result = "()")]
pub struct SendMessage {
    pub send_type: SendType,
    pub user_id: i32,
    pub message: String,
    pub current_room_id: i32, // actor's current room
}

// The message when the Actor `ChatSession` first statrted. This message is for `ChatServer`. This will add the recipient of the Actor `ChatSession` in server's `addr_of_all_other_actors` field.
#[derive(Message)]
#[rtype(result = "()")]
pub struct Join {
    pub addr: Recipient<SendMessage>, // address of `ChatSession`
}

// When a message comes from a chat room, this message will send to the `ChatServer` to pass that message all other actors.
#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientSendMessage {
    pub send_type: SendType,
    pub user_id: i32,
    pub message: String,
    pub current_room_id: i32,
}

// The message to send only a perticular client.
// ChatSession -> ChatServer
#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientSendOneMessage {
    pub message: String,
    pub user_id: i32,
}
// The message to send only a perticular client.
// ChatServer -> ChatSession
#[derive(Message)]
#[rtype(result = "()")]
pub struct SendOneMessage {
    pub message: String,
    pub user_id: i32,
}
