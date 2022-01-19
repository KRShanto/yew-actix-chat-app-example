use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, WebSocket};
use weblog::console_log;
use yew::prelude::*;

use crate::websocket::{UserAndRoomIDForServer, WebsocketServerCommand};
use crate::User;

#[derive(Debug, Serialize, Deserialize)]
struct JoinRoomInfo {
    room_id: i32,
    user_id: i32,
}

#[function_component(JoinRoom)]
pub fn join_room() -> Html {
    let user_details: User = use_context().unwrap();
    let input_ref = NodeRef::default();
    let ws = use_context::<UseStateHandle<Option<WebSocket>>>()
        .expect("No context provided!!!. A context should be provided with `UseStateHandle<Option<WebSocket>>`");

    html! {
        <>
            <h1>{"Join Room"}</h1>
            <label for="join-group-id">{"Enter your group's ID"}</label>
            <input ref={input_ref.clone()} type="number" id="join-group-id" />
            <button onclick={move |_|{

                let user_id = user_details.id;
                let room_id = input_ref
                    .cast::<HtmlInputElement>()
                    .unwrap()
                    .value()
                    .parse::<i32>()
                    .unwrap();

                let details = serde_json::to_string(&JoinRoomInfo { user_id, room_id }).unwrap();
                let ws = ws.clone();

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
                    ws.send_with_str(&serde_json::to_string(&UserAndRoomIDForServer {
                        command_type: WebsocketServerCommand::SendJoinRequest,
                        img_url: user_details.img_url.clone(),
                        username: user_details.username.clone(),
                        nickname: user_details.nickname.clone(),
                        password: user_details.password.clone(),
                        user_id: user_details.id.clone(),
                        room_id: room_id.clone()
                    } ).unwrap());
                }

            }}>{"Send Request"}</button>
        </>
    }
}
