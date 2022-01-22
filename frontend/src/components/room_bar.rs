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
        chat_app::{image_link, no_context_error, JoinRoomRequestsRender, MessageBarRef, User},
        Highlight, MessageComponent, RoomComponent,
    },
    reducers::{CurrentRoomMessageState, CurrentRoomState, RoomListState},
    websocket::{UserIDandRoomIDforServer, WebsocketServerCommand},
};

/// Complete**********
#[function_component(RoomBar)]
pub fn room_bar() -> Html {
    /// contains all RoomComponent
    let room_list: UseReducerHandle<RoomListState> =
        use_context().expect(&no_context_error("UseReducerHandle<RoomListState>"));

    // console_log!(format!("{:?}", room_list));
    html! {
        <>
        <section id="room-bar">
            {
                room_list.rooms.iter().map(|room| {
                    html! {
                        <RoomComponent
                            room={room.clone()}
                        />
                    }
                }).collect::<Html>()
            }
        </section>
        </>
    }
}
