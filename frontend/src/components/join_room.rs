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
            no_context_error, JoinRoomRender, Room, RoomInfo, User, UserID, UserIDAndRoomID,
        },
        Highlight,
    },
    websocket::{UserAndRoomIDForServer, WebsocketServerCommand},
};

#[derive(PartialEq, Properties)]
pub struct JoinRoomProps {
    pub join_room_render: UseStateHandle<JoinRoomRender>,
}

#[function_component(JoinRoom)]
pub fn join_room(props: &JoinRoomProps) -> Html {
    // let join_room_render: UseStateHandle<JoinRoomRender> = use_context().expect(&no_context_error(
    //     "UseStateHandle<UseStateHandle<JoinRoomRender>>",
    // ));

    let join_room_render = props.join_room_render.clone();
    let user_details: User = use_context().expect(&no_context_error("User"));

    let input_ref = NodeRef::default();

    let ws: UseStateHandle<Option<WebSocket>> =
        use_context().expect(&no_context_error("UseStateHandle<Option<WebSocket>>"));

    let submit_form = {
        let input_ref = input_ref.clone();
        let join_room_render = join_room_render.clone();

        move |_| {
            let user_id = user_details.id;
            let room_id_result = input_ref
                .cast::<HtmlInputElement>()
                .unwrap()
                .value()
                .parse::<i32>();
            // .unwrap();

            if let Ok(room_id) = room_id_result {
                let details = serde_json::to_string(&UserIDAndRoomID { user_id, room_id }).unwrap();

                // Sending the request to the server
                spawn_local(async move {
                    let resp = Request::post("http://127.0.0.1:8000/room-join-request")
                        .body(details)
                        .header("Content-Type", "application/json")
                        .send()
                        .await
                        .unwrap();

                    // TODO: I will show an Alert message if the response return 204 http status;
                    console_log!(format!("{:?}", resp));
                });
                // Send a command to websocket;
                if let Some(ws) = (*ws).clone() {
                    ws.send_with_str(
                        &serde_json::to_string(&UserAndRoomIDForServer {
                            command_type: WebsocketServerCommand::SendJoinRequest,
                            img_url: user_details.img_url.clone(),
                            username: user_details.username.clone(),
                            nickname: user_details.nickname.clone(),
                            password: user_details.password.clone(),
                            user_id: user_details.id.clone(),
                            room_id: room_id.clone(),
                        })
                        .unwrap(),
                    );
                }

                // Hide this component;
                join_room_render.set(JoinRoomRender(false));
            }
        }
    };

    let cancel_form = {
        move |_| {
            // Hide this component;
            join_room_render.set(JoinRoomRender(false));
        }
    };

    html! {
        <>
        <Highlight>
            <section class="form">

                <h1 class="form-title">{"Join Room"}</h1>

                <div class="form-wrapper">
                    <label for="join-group-id">{"Enter your group's ID"}</label>
                    <input ref={input_ref.clone()} type="number" id="join-group-id" />
                </div>

                <div class="buttons-div">
                    <button class="submit-btn" onclick={submit_form}>{"Request for Join"}</button>
                    <button class="cancel-btn" onclick={cancel_form}>{"Cancel"}</button>
                </div>

            </section>
        </Highlight>
        </>
    }
}
