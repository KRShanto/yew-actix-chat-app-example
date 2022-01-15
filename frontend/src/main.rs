use serde::{Deserialize, Serialize};
use yew::prelude::*;

mod components;
mod reducers;

use components::{CreateGroup, Login, ShowRooms, Signup};
use reducers::RoomListState;

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

    html! {
        <>
            <div>
                <Login />
                <Signup />

            </div>

            <ContextProvider <UseReducerHandle<RoomListState>> context={room_list.clone()}>
                <CreateGroup />
                <ShowRooms/>
            </ContextProvider<UseReducerHandle<RoomListState>>>
        </>
    }
}
