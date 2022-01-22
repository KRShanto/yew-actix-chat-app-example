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

pub mod components2;
pub mod reducers;
pub mod websocket;

use components2::{ChatApp, NavBar};

use reducers::{
    CurrentRoomAction, CurrentRoomMessageAction, CurrentRoomMessageState, CurrentRoomState,
    RoomListAction, RoomListState,
};
use websocket::{
    ws_onclose, ws_onerror, ws_onmessage, ws_opopen, MessageInfoForServer, UserAndRoomIDForServer,
    UserIDandRoomIDforServer, WebsocketServerCommand, WsRoomID,
};

fn main() {
    yew::start_app::<App>();
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <>

                <header>
                    <NavBar />
                </header>
                <Temporary />
                <ChatApp />

        </>
    }
}
#[function_component(Temporary)]
fn temp() -> Html {
    html! {
        <>


        </>
    }
}
