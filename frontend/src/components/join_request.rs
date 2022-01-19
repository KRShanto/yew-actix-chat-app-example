use reqwasm::http::Request;
use web_sys::WebSocket;
use weblog::console_log;
use yew::prelude::*;

use crate::reducers::CurrentRoomState;
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
                users.iter().map(|user| {
                    html! {
                        <li>{user.nickname.clone()}</li>
                    }
                }).collect::<Html>()
            }
            </ol>
        }

        </>
    }
}
