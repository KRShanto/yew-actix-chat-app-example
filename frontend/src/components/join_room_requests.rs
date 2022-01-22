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
        chat_app::{image_link, no_context_error, JoinRoomRequestsRender, User},
        Highlight,
    },
    reducers::CurrentRoomState,
    websocket::{UserIDandRoomIDforServer, WebsocketServerCommand},
};

#[derive(PartialEq, Properties)]
pub struct JoinRoomRequestsProps {
    pub join_room_requests_render: UseStateHandle<JoinRoomRequestsRender>,
}

#[function_component(JoinRoomRequests)]
pub fn join_room_requests(props: &JoinRoomRequestsProps) -> Html {
    // let join_room_requests_render: UseStateHandle<JoinRoomRequestsRender> =
    //     use_context().expect(&no_context_error("UseStateHanlde<JoinRoomRequestsRender>"));

    let join_room_requests_render = props.join_room_requests_render.clone();

    let current_room_details: UseReducerHandle<CurrentRoomState> =
        use_context().expect(&no_context_error("UseReducerHandle<CurrentRoomState>"));

    let current_room_join_requests: Option<Vec<User>> =
        current_room_details.current_room_join_requests.clone();

    let cancel_form = move |_| {
        join_room_requests_render.set(JoinRoomRequestsRender(false));
    };

    let user_details: User = use_context().expect(&no_context_error("User"));

    html! {

        <>
        <Highlight>
        <section id="join-room-requests-main">
        <h1 class="title">{"Join Requests"}</h1>

        if let Some(users) = current_room_join_requests {

            <div id="join_room_requests">
            {
                users.into_iter().map(|user| {
                    let ws_for_accept_btn = use_context::<UseStateHandle<Option<WebSocket>>>()
                        .expect("No context provided!!!. A context should be provided with `UseStateHandle<Option<WebSocket>>`");

                    let ws_for_reject_btn = use_context::<UseStateHandle<Option<WebSocket>>>()
                        .expect("No context provided!!!. A context should be provided with `UseStateHandle<Option<WebSocket>>`");

                    let current_room_details_for_accept_btn = current_room_details.clone();
                    let current_room_details_for_reject_btn = current_room_details.clone();


                    html! {
                        <div class="requests">
                            <div class="user-details">

                                <img src={image_link(&user.img_url.clone())} alt="User's image" />

                                <div class="names">
                                    <h1 class="nickname">{user.nickname.clone()}</h1>
                                    <p class="username">{user.username.clone()}</p>
                                </div>

                            </div>

                            <div class="buttons">

                                <button class="accept" onclick={move |_| {

                                    if let Some(ws) = (*ws_for_accept_btn).clone() {
                                        ws.send_with_str(&serde_json::to_string(&UserIDandRoomIDforServer {
                                            command_type: WebsocketServerCommand::AcceptJoinRequest,
                                            user_id: user.id,
                                            room_id: current_room_details_for_accept_btn.clone().current_room.clone().unwrap().id
                                         }).unwrap());
                                    }

                                }}>{"Accept"}</button>

                                <button class="reject" onclick={move |_| {

                                    if let Some(ws) = (*ws_for_reject_btn).clone() {
                                        ws.send_with_str(&serde_json::to_string(&UserIDandRoomIDforServer {
                                            command_type: WebsocketServerCommand::RejectRequest,
                                            user_id: user.id,
                                            room_id: current_room_details_for_reject_btn.clone().current_room.clone().unwrap().id
                                        }).unwrap());
                                    }

                                }}>{"Reject"}</button>

                            </div>
                        </div>
                    }

                }).collect::<Html>()
            }
            </div>

        }
                <button class="cancel-btn" onclick={cancel_form}>{"Done"}</button>
            </section>
        </Highlight>
        </>
    }
}
