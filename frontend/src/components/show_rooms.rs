use gloo_storage::{LocalStorage, Storage};
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use weblog::console_log;
use yew::prelude::*;

use crate::{
    reducers::{RoomListAction, RoomListState},
    Room, User,
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
    let room_list = use_context::<UseReducerHandle<RoomListState>>().expect("No context provided!!!. A prop should be provided with `<UseReducerHandle<RoomListState>>`");

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
                    html! {
                        <li>{room.nickname.clone()}</li>
                    }
                }).collect::<Html>()
            }
            </ul>

            <br />
            <hr />

        </>
    }
}
