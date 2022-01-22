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
    components2::{Highlight, MessageComponent, chat_app::{image_link, no_context_error,JoinRoomRequestsRender, User, MessageBarRef,}},
    
    reducers::{CurrentRoomState, CurrentRoomMessageState},
    websocket::{UserIDandRoomIDforServer, WebsocketServerCommand},
    
};

#[derive(PartialEq, Properties)]
pub struct MessageBarProps {
    pub message_bar_ref: MessageBarRef,
}

#[function_component(MessageBar)]
pub fn message_bar(props: &MessageBarProps) -> Html {
    /// Show all messages from the current room;

      let current_room_messages: UseReducerHandle<CurrentRoomMessageState> = use_context().expect( 
        "No context provided!!!. A context should be provided with `<UseReducerHandle<CurrentRoomMessageState>>"
    );
    let current_room_details: UseReducerHandle<CurrentRoomState> = use_context().expect("No context provided!!!. A user should be provided with `UseReducerHandle<CurrentRoomState>`");

    let user_details: User = use_context().expect("No context provided!!!. A user should be provided with `User`"); // details of current user;

    let message_bar_ref=  props.message_bar_ref.clone();

    html! {
        <>
        <section id="message-bar" ref={message_bar_ref.0}>
        // <ul>
            {
                current_room_messages.messages.iter().map(|message| {                    
                    html! {
                        if let Some(user_map) = current_room_details.current_room_users.clone() {

                            if let Some(user) = user_map.get(&message.user_id) {
                                
                                <MessageComponent 
                                    user_id={message.user_id}
                                    message={message.clone().msg} 
                                    nickname={user.nickname.clone()}
                                    img_url={image_link(&user.img_url.clone())}
                                />
                            }

                        }
                    }
                }).collect::<Html>()
            }
            // </ul>
            </section>
        </>
    }
}