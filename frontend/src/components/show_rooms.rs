#![allow(dead_code, unused)]
use gloo_storage::{LocalStorage, Storage};
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use web_sys::WebSocket;
use weblog::{console_log, console_warn};
use yew::prelude::*;

use crate::{
    reducers::{
        CurrentRoomAction, CurrentRoomMessageAction, CurrentRoomMessageState, CurrentRoomState,
        RoomListAction, RoomListState,
    },
    websocket::{WebsocketServerCommand, WsRoomID},
    Room, RoomID, User,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserId {
    user_id: i32,
}

#[derive(Properties, PartialEq)]
pub struct ShowRoomsProps {
    pub room_list: UseReducerHandle<RoomListState>,
}

#[function_component(ShowRooms)]
pub fn show_rooms() -> Html {
    let user_id_state: UseStateHandle<Option<i32>> = use_state(|| None);

    let room_list = use_context::<UseReducerHandle<RoomListState>>().expect(
        "No context provided!!!. A context should be provided with `<UseReducerHandle<RoomListState>>`"
    );
    let current_room_details = use_context::<UseReducerHandle<CurrentRoomState>>().expect(
        "No context provided!!!. A context should be provided with `UseReducerHandle<CurrentRoomState>"
    );
    let current_room_messages = use_context::<UseReducerHandle<CurrentRoomMessageState>>().expect(
        "No context provided!!!. A context should be provided with `<UseReducerHandle<CurrentRoomMessageState>>`"
    );
    let ws = use_context::<UseStateHandle<Option<WebSocket>>>()
        .expect("No context provided!!!. A context should be provided with `UseStateHandle<Option<WebSocket>>`");

    {
        let room_list = room_list.clone();
        use_effect_with_deps(
            move |_| {
                // TODO: I will do this work(get the user details from localstorage in main component, then pass the info to this component as a prop;

                // Getting user's details from localstorage
                let user_info: User = LocalStorage::get("user_info").unwrap();
                user_id_state.set(Some(user_info.id));

                let user_info = UserId {
                    user_id: user_info.id,
                };

                // json data of `user_info`
                let user_info_json = serde_json::to_string(&user_info).unwrap();

                // Getting all rooms for the logged in user
                spawn_local(async move {
                    let resp = Request::post("http://127.0.0.1:8000/get-rooms")
                        .header("Content-Type", "application/json")
                        .body(user_info_json)
                        .send()
                        .await
                        .unwrap();

                    let all_rooms: Vec<Room> = resp.json::<Vec<Room>>().await.unwrap();

                    for room in all_rooms {
                        room_list.dispatch(RoomListAction::AddRoom(room));
                    }
                });
                || ()
            },
            (),
        );
    }

    html! {
        <>
        <hr />
            <br />
            <h1>{"Showing all your rooms"}</h1>
            <ul>
            {
                room_list.rooms.iter().map(|room| {
                    let room = room.clone();
                    let ws = ws.clone();
                    let current_room_details = current_room_details.clone();
                    let current_room_messages = current_room_messages.clone();
                    html! {
                        <>
                            <li>
                                {room.id}{". "}{room.nickname.clone()}
                                <button onclick={ move |_| {
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
                                    }else {
                                        console_warn!("Websocket is not ready yet. The value of the context is still None;")
                                    }
                                }}>{"Select"}</button>
                            </li>
                        </>
                    }
                }).collect::<Html>()
            }
            </ul>

            <br />
            <hr />

        </>
    }
}
