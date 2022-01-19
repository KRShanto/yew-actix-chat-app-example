use reqwasm::http::Request;
use web_sys::WebSocket;
use weblog::console_log;
use yew::prelude::*;

use crate::reducers::CurrentRoomState;
use crate::websocket::{UserIDandRoomIDforServer, WebsocketServerCommand};
use crate::User;

#[function_component(JoinRequest)]
pub fn joinrequest() -> Html {
    let requests: UseStateHandle<Vec<User>> = use_state(|| Vec::new());

    let current_room_details = use_context::<UseReducerHandle<CurrentRoomState>>().expect(
        "No context provided!!!. A prop should be provided with `UseReducerHandle<CurrentRoomState>"
    );

    let ws = use_context::<UseStateHandle<Option<WebSocket>>>()
        .expect("No context provided!!!. A context should be provided with `UseStateHandle<Option<WebSocket>>`");

    html! {
        <>

        <h1>{"All join requests shown here"}</h1>

        if let Some(users) = current_room_details.current_room_join_requests.clone() {

            <ol>
            {
                users.into_iter().map(|user| {
                    let ws = ws.clone();
                    let current_room_details = current_room_details.clone();
                    html! {
                        <>
                            <li>{user.nickname.clone()}</li>
                            <button onclick={ move |_| {

                                if let Some(ws) = (*ws).clone() {
                                    ws.send_with_str(&serde_json::to_string(&UserIDandRoomIDforServer {
                                        command_type: WebsocketServerCommand::AcceptJoinRequest,
                                        user_id: user.id,
                                        room_id: current_room_details.clone().current_room.clone().unwrap().id
                                    }).unwrap());
                                 }

                             }}>{"Accept"}</button>
                            <button>{"Reject"}</button>
                        </>
                    }
                }).collect::<Html>()
            }
            </ol>
        }

        </>
    }
}
