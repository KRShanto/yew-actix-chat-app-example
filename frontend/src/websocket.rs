// This module contains some functions for dealing with websocket;
#![allow(dead_code, unused)]

use js_sys::JsString;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{ErrorEvent, InputEvent, MessageEvent, WebSocket};
use web_sys::{HtmlElement, HtmlInputElement};
use weblog::{console_error, console_log, console_warn};
use yew::prelude::*;
use yew::NodeRef;

use crate::reducers::{CurrentRoomMessageAction, CurrentRoomMessageState};
use crate::{Message, User};

// ############################# Websocket commands for Server ########################### //

#[derive(Debug, Serialize, Deserialize)]
pub enum WebsocketServerCommand {
    UserSetUp,   // When websocket first connected, this command will execute.
    ChangeRoom,  // When the user clicks on a room, this command will execute.
    SendMessage, // When the user sends a message(clicks the send button), this command will execute
}

// ############################# Websocket commands for client ########################### //

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum WebsocketClientCommand {
    AddMessage,
}

// Info for changing `UserSetUp` command
#[derive(Debug, Serialize, Deserialize)]
struct WsUserID {
    command_type: WebsocketServerCommand,
    user_id: i32,
}

// Info for changing `ChangeRoom` command
#[derive(Debug, Serialize, Deserialize)]
pub struct WsRoomID {
    pub command_type: WebsocketServerCommand,
    pub room_id: i32,
}

// Send chat messages to server; client -> server
#[derive(Debug, Serialize, Deserialize)]
pub struct MessageInfoForServer {
    pub command_type: WebsocketServerCommand,
    pub msg: String,
    pub room_id: i32,
    pub user_id: i32,
}

// Recieve chat message from server. server -> client
#[derive(Debug, Serialize, Deserialize)]
pub struct MessageInfoForClient {
    pub id: i32,
    pub command_type: WebsocketClientCommand,
    pub msg: String,
    pub room_id: i32,
    pub user_id: i32,
}

// ************************************************************************* //
// ############### When Websocket sends message to the client ################# //
// ************************************************************************* //
pub fn ws_onmessage(
    ws: WebSocket,
    current_room_messages: UseReducerHandle<CurrentRoomMessageState>,
) {
    let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
        if let Ok(text) = e.data().dyn_into::<js_sys::JsString>() {
            // converting JsString to String;
            if let Some(text) = text.as_string() {
                console_log!("message event, received Text: {:?}", text.clone());

                if let Ok(message) = serde_json::from_str::<MessageInfoForClient>(&text) {
                    // AddMessage command
                    if message.command_type == WebsocketClientCommand::AddMessage {
                        // Set the new message in the message state;
                        current_room_messages.dispatch(CurrentRoomMessageAction::AddMessage(
                            Message {
                                id: message.id,
                                msg: message.msg,
                                room_id: message.room_id,
                                user_id: message.user_id,
                            },
                        ));
                        console_log!("Server is trying to send a command `AddMessage`");
                    }
                    console_log!("Server is trying to send data `MessageInfoForClient`")
                }
            } else {
                console_error!("websocket message recieved was not a string!");
            }
        } else {
            console_error!("message event, received Unknown: {:?}", e.data());
        }
    }) as Box<dyn FnMut(MessageEvent)>);
    ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
    onmessage_callback.forget();
}

// ************************************************************************* //
// ############ When server stop getting websocket connections ############# //
// ************************************************************************* //
pub fn ws_onclose(ws: WebSocket) {
    let onclose_callback = Closure::wrap(Box::new(move |_| {
        console_error!("Socket closed :(");
    }) as Box<dyn FnMut(JsValue)>);
    ws.set_onclose(Some(onclose_callback.as_ref().unchecked_ref()));
    onclose_callback.forget();
}

// ************************************************************************* //
// ############ When an error occur from Websocket ############# //
// ************************************************************************* //
pub fn ws_onerror(ws: WebSocket) {
    let onerror_callback = Closure::wrap(Box::new(move |e: ErrorEvent| {
        console_error!("error event: {:?}", e);
    }) as Box<dyn FnMut(ErrorEvent)>);
    ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
    onerror_callback.forget();
}

// ************************************************************************* //
// ############ When Websocket is opened ############# //
// ************************************************************************* //
pub fn ws_opopen(ws: WebSocket, user_details: User) {
    let ws_clone = ws.clone();
    let onopen_callback = Closure::wrap(Box::new(move |_| {
        // ************************************** //

        console_log!("socket opened");
        ws_clone.send_with_str("I've connected with you").unwrap();

        // Running UserSetUp command
        ws_clone.send_with_str(
            &serde_json::to_string(&WsUserID {
                command_type: WebsocketServerCommand::UserSetUp,
                user_id: user_details.id,
            })
            .unwrap(),
        );

        // ************************************** //
    }) as Box<dyn FnMut(JsValue)>);

    ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
    onopen_callback.forget();
}
