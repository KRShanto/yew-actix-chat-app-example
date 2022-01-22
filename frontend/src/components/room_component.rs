#![allow(dead_code, unused)]
use gloo_storage::{LocalStorage, Storage};
use reqwasm::http::{FormData, Request};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::rc::Rc;
use uuid::Uuid;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::WebSocket;
use web_sys::{Element, HtmlDivElement, HtmlElement, HtmlInputElement};
use weblog::{console_log, console_warn};
use yew::prelude::*;

use crate::{
    components::{
        chat_app::{image_link, no_context_error, JoinRoomRequestsRender, Room, User},
        Highlight,
    },
    reducers::{CurrentRoomAction, CurrentRoomMessageState, CurrentRoomState},
    websocket::{UserIDandRoomIDforServer, WebsocketServerCommand, WsRoomID},
};

#[derive(PartialEq, Properties)]
pub struct RoomComponentProps {
    pub room: Room,
}

/// Complete************
#[function_component(RoomComponent)]
pub fn room_component(props: &RoomComponentProps) -> Html {
    let room = props.room.clone();

    // let img_url = format!("http://127.0.0.1:8000/get-user-image/{}", room.img_url);

    let current_room_details = use_context::<UseReducerHandle<CurrentRoomState>>().expect(
        "No context provided!!!. A context should be provided with `UseReducerHandle<CurrentRoomState>"
    );
    let ws = use_context::<UseStateHandle<Option<WebSocket>>>()
        .expect("No context provided!!!. A context should be provided with `UseStateHandle<Option<WebSocket>>`");

    let onclick = {
        let room = room.clone();
        move |_| {
            // changing the current room state;
            current_room_details.dispatch(CurrentRoomAction::SelectRoom(room.clone()));

            // executing `ChangeRoom` command on websocket;
            if let Some(ws) = (*ws).clone() {
                ws.send_with_str(
                    &serde_json::to_string(&WsRoomID {
                        command_type: WebsocketServerCommand::ChangeRoom,
                        room_id: room.id,
                    })
                    .unwrap(),
                );
            } else {
                console_warn!("Websocket is not ready yet. The value of the context is still None;")
            }
        }
    };

    html! {
        <>
        <section class="chat-room" {onclick} >
            <div class="room-image">
                <img src={image_link(&room.img_url.clone())} alt="Room image" />
            </div>
            <div class="names">
                <h1 class="nickname">{room.nickname.clone()}</h1>
                <p class="id">{room.clone().id}</p>
            </div>
        </section>
        </>
    }
}
