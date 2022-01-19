// #![allow(dead_code, unused)]

use crate::{
    actors::{ChatServer, ClientSendMessage, Join, SendMessage, SendType},
    db::{
        add_user_into_room, create_message, delete_user_from_room, establish_connection,
        get_room_from_id,
    },
    models::Room,
};

use actix::prelude::*;
use actix_web_actors::ws;
use colored::*;
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

const HEARTBEAT_TIMEOUT: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

// A command comes from client to server. client -> server
#[derive(Debug, Serialize, Deserialize, PartialEq)]
enum WebsocketServerCommand {
    UserSetUp,
    ChangeRoom,
    SendMessage,
    SendJoinRequest,
    AcceptJoinRequest,
}
// A command sends to client from server. server -> client
#[derive(Debug, Serialize, Deserialize, PartialEq)]
enum WebsocketClientCommand {
    AddMessage,
    ShowJoinRequest,
    AppendRoom,
    RemoveRequest, // This will remove the list of join requests. Not reject the request. This command should execute when a request is accepted
}

// `UserSetUp` commmand
// info of user when the `UserSetUp` command is executed from client;
// Use this command when the websocket is first connected to the server
#[derive(Debug, Serialize, Deserialize)]
struct UserID {
    command_type: WebsocketServerCommand,
    user_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct RoomID {
    command_type: WebsocketServerCommand,
    room_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct UserIDandRoomIDforServer {
    command_type: WebsocketServerCommand,
    room_id: i32,
    user_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct UserIDandRoomIDforClient {
    command_type: WebsocketClientCommand,
    room_id: i32,
    user_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct UserAndRoomIDForServer {
    command_type: WebsocketServerCommand,
    room_id: i32,
    user_id: i32,
    nickname: String,
    username: String,
    img_url: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct UserAndRoomIDForClient {
    command_type: WebsocketClientCommand,
    room_id: i32,
    user_id: i32,
    nickname: String,
    username: String,
    password: String,
    img_url: String,
}

// This is for client
#[derive(Debug, Serialize, Deserialize)]
struct RoomInfo {
    command_type: WebsocketClientCommand,
    room_id: i32,
    nickname: String,
    img_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct MessageInfoForServer {
    // This will come from the client;
    command_type: WebsocketServerCommand,
    msg: String,
    room_id: i32,
    user_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct MessageInfoForClient {
    // This will send to the client;
    id: i32,
    command_type: WebsocketClientCommand,
    msg: String,
    room_id: i32,
    user_id: i32,
}

fn room_not_found_error() -> String {
    format!("{}", "Developer Error: Looks like you forgot to execute the command `ChangeRoom`. No room found in this actor. Consider sending this command when user has clicked a chatroom".red().bold().italic())
}

fn user_not_found_error() -> String {
    format!("{}", "Developer Error: Looks like you forgot to execute the command `UserSetUp`!!! No user found in this actor. Consider sending this command when the page is loaded, check if the user logged in and then send this command")
}

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
        // Sending message to client/browser
        if msg.send_type == SendType::Plural {
            if let Some(room_id) = self.current_room_id {
                println!("{}", "Sending message to client/browser".green().bold());

                if msg.current_room_id == room_id {
                    ctx.text(msg.message)
                }
            }
        } else if msg.send_type == SendType::Singular {
            if let Some(user_id) = self.user_id {
                if msg.user_id == user_id {
                    ctx.text(msg.message)
                }
            }
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
            Ok(WsMessage::Binary(bin)) => ctx.binary(bin),
            Ok(WsMessage::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            Ok(WsMessage::Text(text)) => {
                // TODO: These commands are not following DRY. I would consider better approach. maybe I will make a macro for these;

                // *************** User setup command *************** //
                if let Ok(command) = serde_json::from_str::<UserID>(&text) {
                    if command.command_type == WebsocketServerCommand::UserSetUp {
                        // Set the self.user_id
                        self.user_id = Some(command.user_id);
                        println!("{}", "Command executed for setting up user".green());
                    }
                }
                // *************** Room changing command *************** //
                if let Ok(command) = serde_json::from_str::<RoomID>(&text) {
                    if command.command_type == WebsocketServerCommand::ChangeRoom {
                        // change the self.current_room;
                        self.current_room_id = Some(command.room_id);
                        println!("{}", "Command executed for changing room".green());
                    }
                }
                // *************** Message Sending and Message Creating command *************** //
                if let Ok(command) = serde_json::from_str::<MessageInfoForServer>(&text) {
                    if command.command_type == WebsocketServerCommand::SendMessage {
                        // Create new message
                        // TODO: For now I am assuming that the user's id and room's id will be valid. But later I will validate this

                        if let Ok(message) =
                            create_message(command.msg, command.user_id, command.room_id)
                        {
                            // Send AddMessage command to the client;
                            self.server.do_send(ClientSendMessage {
                                send_type: SendType::Plural,
                                user_id: self.user_id.expect(&user_not_found_error()),
                                current_room_id: self
                                    .current_room_id
                                    .expect(&room_not_found_error()),
                                message: serde_json::to_string(&MessageInfoForClient {
                                    command_type: WebsocketClientCommand::AddMessage,
                                    id: message.id,
                                    msg: message.msg,
                                    room_id: message.room_id,
                                    user_id: message.user_id,
                                })
                                .unwrap(),
                            });
                            println!(
                                "{}",
                                "Command executed for Sending and creating and sending new message"
                                    .green()
                                    .bold()
                            );
                        } else {
                            println!(
                                "{}",
                                "Failed to send AddMessage command. Message didn't created"
                                    .red()
                                    .bold()
                            );
                        }
                    }
                }
                // *************** Room Join Request command *************** //
                if let Ok(command) = serde_json::from_str::<UserAndRoomIDForServer>(&text) {
                    if command.command_type == WebsocketServerCommand::SendJoinRequest {
                        // Send `ShowJoinRequest` comamnd to client;
                        self.server.do_send(ClientSendMessage {
                            send_type: SendType::Plural,
                            user_id: self.user_id.expect(&user_not_found_error()),
                            current_room_id: command.room_id,
                            message: serde_json::to_string(&UserAndRoomIDForClient {
                                room_id: command.room_id,
                                command_type: WebsocketClientCommand::ShowJoinRequest,
                                img_url: command.img_url,
                                nickname: command.nickname,
                                username: command.username,
                                user_id: command.user_id,
                                password: command.password,
                            })
                            .unwrap(),
                        });
                        println!(
                            "{}",
                            "Command executed Sending and showing join requests"
                                .green()
                                .bold()
                        );
                    }
                }
                // *************** Accept Room Join Request command *************** //
                if let Ok(command) = serde_json::from_str::<UserIDandRoomIDforServer>(&text) {
                    if command.command_type == WebsocketServerCommand::AcceptJoinRequest {
                        // Create a new `rooms_users` column with the `accepted` = true; and delete the old one;
                        // Delete the old rooms_users
                        delete_user_from_room(
                            command.user_id,
                            command.room_id,
                            &establish_connection(),
                        )
                        .unwrap();

                        // Create a new one/add that user into that room again with the field `accepted` = true;
                        add_user_into_room(
                            command.user_id,
                            command.room_id,
                            establish_connection(),
                            true,
                        )
                        .unwrap();

                        // send the command `RemoveRequest` to client to so that the request can removed from all client
                        self.server.do_send(ClientSendMessage {
                            message: serde_json::to_string(&UserIDandRoomIDforClient {
                                command_type: WebsocketClientCommand::RemoveRequest,
                                user_id: command.user_id,
                                room_id: command.room_id,
                            })
                            .unwrap(),
                            current_room_id: command.room_id,
                            send_type: SendType::Plural,
                            user_id: self.user_id.expect(&user_not_found_error()),
                        });

                        // getting the room details:
                        let room: Room =
                            get_room_from_id(command.room_id, &establish_connection()).unwrap();

                        // send the command `AppendRoom` so that the request's user's room can get updated;
                        self.server.do_send(ClientSendMessage {
                            message: serde_json::to_string(&RoomInfo {
                                command_type: WebsocketClientCommand::AppendRoom,
                                img_url: room.img_url,
                                room_id: room.id,
                                nickname: room.nickname,
                            })
                            .unwrap(),
                            current_room_id: command.room_id,
                            user_id: command.user_id,
                            send_type: SendType::Singular,
                        });

                        println!(
                            "{}",
                            "Command executed For accepting join request".green().bold()
                        );
                    }
                }

                ctx.text(text);
            }

            _ => ctx.stop(),
        }
    }
}
