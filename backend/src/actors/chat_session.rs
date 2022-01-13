use crate::actors::{ChatServer, ClientSendMessage, Join, SendMessage};

use colored::*;
use std::time::{Duration, Instant};

use actix::prelude::*;
use actix_web_actors::ws;

const HEARTBEAT_TIMEOUT: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

// Actor for each reqeust/client. This actor will responsible for taking client's messages and pass them in the server `ChatServer`. And then `ChatServer` will pass that message to other Actors or will do something else
pub struct ChatSession {
    pub hb: Instant,
    pub server: Addr<ChatServer>, // address of the server. In this case the server is `ChatServer`
    pub current_room_id: Option<i32>, // currently selected room id.
    pub user_id: Option<i32>, // user's id. When client sends the command `set_user`, it will be fulfilled with the user's id.
}

impl ChatSession {
    pub fn new(server: Addr<ChatServer>) -> Self {
        Self {
            server,
            current_room_id: None,
            user_id: None,
            hb: Instant::now(),
        }
    }

    pub fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_TIMEOUT, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                println!("Websocket Client heartbeat failed, disconnecting!");

                ctx.stop();
                return;
            }

            ctx.ping(b"");
        });
    }
}

impl Actor for ChatSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        // Sending the address of this Actor to the server `ChatServer` so that the server can send other Actor message to this Actor
        let addr = ctx.address();

        self.server.do_send(Join {
            addr: addr.recipient(),
        });

        self.hb(ctx);
        ctx.text("You are connected with server :)");
    }
    // TODO: I will make `stoped` function for printing in terminal that the actor is dead or the client is disconnected.
}

impl Handler<SendMessage> for ChatSession {
    type Result = ();

    fn handle(&mut self, msg: SendMessage, ctx: &mut Self::Context) {
        let err_msg =format!("{}", "Developer error: Looks like you forgot to send the command `change_room` to server :(\nreason: When the user clicks on any room a command `change_room` should sent to the server. Else it cannot specify which room is the user joined.".red());

        // Sending message to client/browser
        if msg.current_room_id == self.current_room_id.expect(&err_msg) {
            ctx.text(msg.message)
        }
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ChatSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        type WsMessage = ws::Message;

        match msg {
            Ok(WsMessage::Ping(msg)) => {
                self.hb = Instant::now();
                println!("Ping");
                ctx.pong(&msg);
            }
            Ok(WsMessage::Pong(_)) => {
                println!("Pong");
                self.hb = Instant::now();
            }
            Ok(WsMessage::Text(text)) => {
                let room_id_not_found_err_msg = format!("{}", "Developer error: Looks like you forgot to send the command `change_room` to server :(\nreason: When the user clicks on any room a command `change_room` should sent to the server. Else it cannot specify which room is the user joined.".red());

                self.server.do_send(ClientSendMessage {
                    message: text,
                    current_room_id: self.current_room_id.expect(&room_id_not_found_err_msg),
                })
            }
            Ok(WsMessage::Binary(bin)) => ctx.binary(bin),
            Ok(WsMessage::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}
