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
    components2::{
        chat_app::{image_link, no_context_error, JoinRoomRequestsRender, User},
        Highlight,
    },
    reducers::{CurrentRoomMessageState, CurrentRoomState},
    websocket::{UserIDandRoomIDforServer, WebsocketServerCommand},
};

#[derive(PartialEq, Properties)]
pub struct MessageComponentProps {
    pub user_id: i32,
    pub message: String,  // message
    pub nickname: String, // nickname of current user
    pub img_url: String, // img url of current user. give the full path with http://127.0.0.1:8000/get-user-image/{}
}

/// Complete**********
#[function_component(MessageComponent)]
pub fn message(props: &MessageComponentProps) -> Html {
    let user_details: User = use_context().expect(&no_context_error("User"));

    let class_name = if props.user_id == user_details.id {
        "owner"
    } else {
        "other"
    };

    let nickname = if props.nickname == user_details.nickname {
        String::from("You")
    } else {
        props.nickname.clone()
    };

    html! {
        <>
        <section class={format!("user-message {}", class_name)}>

            <img
                class="user-image"
                src={props.img_url.clone()}
                alt="User image"
            />
            <div class="message-and-nickname">
                <h1 class="nickname">{nickname}</h1>
                <p
                    class="message"
                    // style={format!("width: {}px;", props.message.len())}
                >
                    {props.message.clone()
                }</p>
            </div>



        </section>



        </>
    }
}
