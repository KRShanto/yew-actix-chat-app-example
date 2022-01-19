#![allow(dead_code, unused)]
use gloo_storage::{LocalStorage, Storage};
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::WebSocket;
use weblog::{console_log, console_warn};
use yew::prelude::*;

pub mod components;
pub mod reducers;
pub mod websocket;

use components::{
    ChatBar, CreateRoom, JoinRequest, JoinRoom, Login, MessageBar, ShowRooms, Signup,
};
use reducers::{
    CurrentRoomAction, CurrentRoomMessageAction, CurrentRoomMessageState, CurrentRoomState,
    RoomListState,
};
use websocket::{ws_onclose, ws_onerror, ws_onmessage, ws_opopen};

#[wasm_bindgen]
extern "C" {
    fn setTimeout(closure: &Closure<dyn FnMut()>, time: u32) -> i32;
    fn clearTimeout(timeout_id: i32);
}

// Struct for holding details about any room;
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Room {
    pub id: i32,
    pub nickname: String,
    pub img_url: String,
}

// User's full info
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct User {
    pub id: i32,
    pub nickname: String,
    pub username: String,
    pub password: String,
    pub img_url: String,
}

// Message for a room;
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Message {
    pub id: i32,
    pub msg: String,
    pub user_id: i32,
    pub room_id: i32,
}
// *************** User and Room id send for getting new messages ***************** //
#[derive(Debug, Serialize, Deserialize)]
pub struct UserAndRoomID {
    pub user_id: i32, // user's id
    pub room_id: i32, // room's id
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoomID {
    room_id: i32, // room's id
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserID {
    user_id: i32, // user's id
}

fn main() {
    yew::start_app::<App>();
}

#[function_component(App)]
fn app() -> Html {
    let room_list = use_reducer(RoomListState::new); // list of all rooms that the user is currently joined
    let user_details: User = LocalStorage::get("user_info").unwrap(); // details of user. // TODO: For now I am getting these values from localhost, later I will use cookies instead of localhost
    let current_room_details = use_reducer(CurrentRoomState::new);
    let current_room_messages = use_reducer(CurrentRoomMessageState::new);

    let ws: UseStateHandle<Option<WebSocket>> = use_state(|| None); // I will use async approach here later.

    {
        // First render
        let ws = ws.clone();

        use_effect_with_deps(
            move |_| {
                // TODO: I need to make the ws = Option<WebSocket> in async way so that ws.await.send() can wait when the websocket is connected;
                // TODO: I need to learn async and then I will start from `frontend/src/main:78`

                ws.set(Some(WebSocket::new("ws://127.0.0.1:8000/ws/").expect(
                    "Websocket connection failed, maybe you forgot to start the server",
                )));

                move || ()
            },
            (),
        );
    }
    {
        // When the `ws` state changes.
        let ws = ws.clone();
        let user_details = user_details.clone();
        let current_room_messages = current_room_messages.clone();
        let current_room_details = current_room_details.clone();

        use_effect_with_deps(
            move |ws| {
                let ws = ws.clone();

                if let Some(_) = (*ws).clone() {
                    ws_opopen((*ws).clone().unwrap(), user_details.clone());
                    ws_onerror((*ws).clone().unwrap());
                    ws_onmessage(
                        (*ws).clone().unwrap(),
                        current_room_messages,
                        current_room_details,
                    );
                    ws_onclose((*ws).clone().unwrap());
                }

                || ()
            },
            ws,
        );
    }
    {
        // When a room changes, Change the messages of that room;
        let current_room_messages = current_room_messages.clone();

        use_effect_with_deps(
            move |current_room_details| {
                // is any room is selected
                if let Some(room) = current_room_details.current_room.clone() {
                    // When the room is changed, fetch new messages.

                    // info for making post request on `get-messages` route
                    let user_room_info = RoomID { room_id: room.id };

                    // json data of `user_room_info`
                    let user_room_info_json = serde_json::to_string(&user_room_info).unwrap();

                    // {
                    spawn_local(async move {
                        let resp = Request::post("http://127.0.0.1:8000/get-messages")
                            .header("Content-Type", "application/json")
                            .body(user_room_info_json)
                            .send()
                            .await
                            .unwrap();

                        // getting messages from the response
                        let response_messages = resp.json::<Vec<Message>>().await.unwrap();

                        current_room_messages
                            .dispatch(CurrentRoomMessageAction::ResetMessages(response_messages));
                    });
                    // }
                }

                || ()
            },
            current_room_details.clone(),
        );
    }
    {
        // When a room changes, fetch users inside that room and put their info inside the current room state;

        use_effect_with_deps(
            move |current_room_details| {
                // putting users into `current_room_details.current_room_users`
                if let None = current_room_details.current_room_users {
                    if let Some(current_room) = current_room_details.current_room.clone() {
                        let current_room_details = current_room_details.clone();
                        spawn_local(async move {
                            let room_info = RoomID {
                                room_id: current_room.id,
                            };
                            let room_info_json = serde_json::to_string(&room_info).unwrap();

                            let resp = Request::post("http://127.0.0.1:8000/get-users-from-room")
                                .header("Content-Type", "application/json")
                                .body(room_info_json)
                                .send()
                                .await
                                .unwrap();

                            // getting users from the response
                            let response_users = resp.json::<Vec<User>>().await.unwrap();

                            current_room_details
                                .dispatch(CurrentRoomAction::PutUsers(response_users))
                        });
                    }
                }

                || ()
            },
            current_room_details.clone(), // dependents
        );
    }
    {
        // When a room changes, change refetch the join reqeusts;
        let current_room_details = current_room_details.clone();
        use_effect_with_deps(
            move |current_room_details| {
                if let Some(room) = current_room_details.current_room.clone() {
                    // info for making post request on `get-join-requests` route
                    let room_info = RoomID { room_id: room.id };

                    // json data of `user_room_info`
                    let room_info_json = serde_json::to_string(&room_info).unwrap();

                    let current_room_details = current_room_details.clone();
                    spawn_local(async move {
                        let resp = Request::post("http://127.0.0.1:8000/get-join-requests")
                            .header("Content-Type", "application/json")
                            .body(room_info_json)
                            .send()
                            .await
                            .unwrap();

                        // getting messages from the response
                        let response_messages = resp.json::<Vec<User>>().await.unwrap();

                        current_room_details
                            .dispatch(CurrentRoomAction::PutJoinRequests(response_messages));
                    });
                }

                || ()
            },
            current_room_details,
        );
    }

    let ws = ws.clone();

    html! {
        <>
            <div>
                <Login />
                <Signup />

            </div>
            <ContextProvider <UseStateHandle<Option<WebSocket>>> context={ws}>
            <ContextProvider <UseReducerHandle<CurrentRoomMessageState>> context={current_room_messages.clone()}>
            <ContextProvider <UseReducerHandle<CurrentRoomState>> context={current_room_details.clone()}>
            <ContextProvider <UseReducerHandle<RoomListState>> context={room_list.clone()}>
            <ContextProvider <String> context={String::from("Shanto")}>
            <ContextProvider <User> context={user_details.clone()}>

                <CreateRoom />
                <JoinRoom />
                <ShowRooms/>
                <ChatBar />
                <MessageBar />
                <JoinRequest />
                <Temporary />

            </ ContextProvider <User> >
            </ ContextProvider< String>>
            </ContextProvider<UseReducerHandle<RoomListState>>>
            </ContextProvider<UseReducerHandle<CurrentRoomState>>>
            </ContextProvider<UseReducerHandle<CurrentRoomMessageState>>>
            </ContextProvider <UseStateHandle<Option<WebSocket>>>>
        </>
    }
}
#[function_component(Temporary)]
fn temp() -> Html {
    html! {}
}
