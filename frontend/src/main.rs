use serde::{Deserialize, Serialize};
use web_sys::WebSocket;
use yew::prelude::*;

mod components;
mod reducers;
mod websocket;

use components::{CreateRoom, Login, ShowRooms, Signup};
use reducers::RoomListState;
use websocket::{ws_onerror, ws_onmessage, ws_opclose, ws_opopen};

// Struct for holding details about any room;
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Room {
    pub id: i32,
    pub nickname: String,
    pub img_url: String,
}

// User's full info
#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: i32,
    pub nickname: String,
    pub username: String,
    pub password: String,
    pub img_url: String,
}

fn main() {
    yew::start_app::<App>();
}

#[function_component(App)]
fn app() -> Html {
    let room_list = use_reducer(RoomListState::new);
    let ws = WebSocket::new("ws://127.0.0.1:8000/ws/")
        .expect("Websocket connection failed, maybe you forgot to start the server");

    use_effect_with_deps(
        move |_| {
            ws_opopen(ws.clone());
            ws_onerror(ws.clone());
            ws_onmessage(ws.clone());
            ws_opclose(ws.clone());

            || ()
        },
        (),
    );

    html! {
        <>
            <div>
                <Login />
                <Signup />

            </div>

            <ContextProvider <UseReducerHandle<RoomListState>> context={room_list.clone()}>
                <CreateRoom />
                <ShowRooms/>
            </ContextProvider<UseReducerHandle<RoomListState>>>
        </>
    }
}
