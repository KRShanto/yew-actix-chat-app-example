#![allow(dead_code, unused)]
use web_sys::{HtmlInputElement, WebSocket};
use weblog::{console_log, console_warn};
use yew::prelude::*;

use crate::{
    reducers::{
        CurrentRoomAction, CurrentRoomMessageAction, CurrentRoomMessageState, CurrentRoomState,
        RoomListAction, RoomListState,
    },
    websocket::{MessageInfoForServer, WebsocketServerCommand},
    Room, User,
};

#[function_component(ChatBar)]
pub fn chat_bar() -> Html {
    let current_room_messages = use_context::<UseReducerHandle<CurrentRoomMessageState>>().expect(
        "No context provided!!!. A prop should be provided with `<UseReducerHandle<CurrentRoomMessageState>>`"
    );
    let current_room_details = use_context::<UseReducerHandle<CurrentRoomState>>().expect(
        "No context provided!!!. A prop should be provided with `UseReducerHandle<CurrentRoomState>"
    );
    let ws = use_context::<UseStateHandle<Option<WebSocket>>>()
        .expect("No context provided!!!. A context should be provided with `UseStateHandle<Option<WebSocket>>`");

    let user_details = use_context::<User>()
        .expect("No context provided!!!. A context should be provided with `User`");

    let input_ref = NodeRef::default();

    let render = use_state(|| false);
    let room_name = use_state(|| String::new());
    {
        let render = render.clone();
        let room_name = room_name.clone();

        use_effect_with_deps(
            move |current_room_details| {
                match current_room_details.current_room.clone() {
                    Some(room) => {
                        render.set(true);
                        room_name.set(room.nickname);
                    }
                    None => {
                        render.set(false);
                    }
                }
                || ()
            },
            (current_room_details.clone()),
        );
    }

    let room_name = room_name.clone();
    let user_details = user_details.clone();
    let ws = ws.clone();
    let current_room_details = current_room_details.clone();
    html! {
        <>

        if *render {
            <h1>
            {"Your "} <i>{(*room_name).clone()}</i> {"'s messages show here"}
            </h1>


            <input placeholder="Send a message" type="text" ref={input_ref.clone()}/>

            <button onclick={ move |_| {
                let input_msg = input_ref.cast::<HtmlInputElement>().unwrap().value();

                // Send this message to websocket
                if let Some(ws) = (*ws).clone() {
                    ws.send_with_str(
                        &serde_json::to_string(&MessageInfoForServer {
                            msg: input_msg,
                            command_type: WebsocketServerCommand::SendMessage,
                            room_id: current_room_details.current_room.clone().unwrap().id, // TODO: I will handle this error later
                            user_id: user_details.id,
                        })
                        .unwrap(),
                    );
                }
                else {
                    console_warn!("Websocket is not ready yet. The value of the context is still None;")
                }

            }}>{"Send message"}</button>
        }
        else {
            <h1>{"You haven't choose any room! Select a room"}</h1>
        }

        </>
    }
}
