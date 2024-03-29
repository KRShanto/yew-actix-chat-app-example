use crate::actors::*;
use actix::prelude::*;
use colored::*;

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
        println!("{}", "A new client joined the chat server".blue().bold());

        // new client joined the chat server. Save that client's address to the ```addr_of_all_other_actors```
        self.addr_of_all_other_actors.push(Some(msg.addr));
    }
}

impl Handler<ClientSendMessage> for ChatServer {
    type Result = ();

    // Sending messages to all actors.
    fn handle(&mut self, msg: ClientSendMessage, _ctx: &mut Context<Self>) {
        for addr in self.addr_of_all_other_actors.clone() {
            if let Some(addr) = addr {
                match addr.do_send(SendMessage {
                    send_type: msg.send_type.clone(),
                    user_id: msg.user_id,
                    message: msg.message.clone(),
                    current_room_id: msg.current_room_id.clone(),
                }) {
                    Ok(_) => {}
                    Err(error) => {
                        println!(
                            "{}",
                            format!("Failed to send websocket message: {:?}", error)
                                .red()
                                .bold()
                        );
                    }
                }
            }
        }
    }
}
