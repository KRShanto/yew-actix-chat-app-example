use crate::actors::{ClientSendMessage, Join, SendMessage};
use actix::prelude::*;

// The main actor for all other actors for communication. This actor will handle all other actor's messages and pass them that messages
pub struct ChatServer {
    pub addr_of_all_other_actors: Vec<Option<Recipient<SendMessage>>>,
}

impl Actor for ChatServer {
    type Context = Context<Self>;
}

impl Handler<Join> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Join, _ctx: &mut Context<Self>) {
        println!("A new client joined the chat server");
        self.addr_of_all_other_actors.push(Some(msg.addr));
    }
}

impl Handler<ClientSendMessage> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: ClientSendMessage, _ctx: &mut Context<Self>) {
        // Sending messages to all actors.
        for addr in self.addr_of_all_other_actors.clone() {
            if let Some(addr) = addr {
                addr.do_send(SendMessage {
                    message: msg.message.clone(),
                    current_room_id: msg.current_room_id.clone(),
                })
                .unwrap();
            }
        }
    }
}
