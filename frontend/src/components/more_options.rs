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
        chat_app::{
            image_link, no_context_error, CreateNewRoomRender, JoinRoomRender,
            JoinRoomRequestsRender, User,
        },
        Highlight,
    },
    reducers::{CurrentRoomMessageState, CurrentRoomState},
    websocket::{UserIDandRoomIDforServer, WebsocketServerCommand},
};

#[derive(PartialEq, Properties)]
pub struct MoreOptionsProps {
    pub join_room_render: UseStateHandle<JoinRoomRender>,
    pub create_new_room_render: UseStateHandle<CreateNewRoomRender>,
}

#[function_component(MoreOptions)]
pub fn more_options(props: &MoreOptionsProps) -> Html {
    /// Options for the chat application. "create new group", "join group" etc. will be shown here.
    // let join_room_render: UseStateHandle<JoinRoomRender> =
    //     use_context().expect(&no_context_error("UseStateHandle<JoinRoomRender>"));

    // let create_new_room_render: UseStateHandle<CreateNewRoomRender> =
    //     use_context().expect(&no_context_error("UseStateHandle<CreateNewRoomRender>"));
    let join_room_render = props.join_room_render.clone();
    let create_new_room_render = props.create_new_room_render.clone();

    // console_log!(format!("{:?}", create_new_room_render));

    html! {
        <>
        <section id="more-options">

            <button class="create-new-room" onclick={ move |_| {
                create_new_room_render.set(CreateNewRoomRender(true));
            }}>{"Create new room"}</button>

            <button class="join-room" onclick={ move |_| {
                join_room_render.set(JoinRoomRender(true));
            }}>{"Join room"}</button>

        </section>
        </>
    }
}
