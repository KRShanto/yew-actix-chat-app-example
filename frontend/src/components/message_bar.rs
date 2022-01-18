use crate::reducers::{
    CurrentRoomAction, CurrentRoomMessageAction, CurrentRoomMessageState, CurrentRoomState,
    RoomListAction, RoomListState,
};
use crate::{User, UserID};

use yew::prelude::*;
use weblog::{console_log, console_warn};
use std::collections::HashMap;
use wasm_bindgen_futures::spawn_local;
use reqwasm::http::Request;

// The component ChatBar should render this
// #[tokio::main]
#[function_component(MessageBar)]
pub  fn message() -> Html {
    let current_room_messages: UseReducerHandle<CurrentRoomMessageState> = use_context().expect( 
        "No context provided!!!. A context should be provided with `<UseReducerHandle<CurrentRoomMessageState>>"
    );
    let user_details: User = use_context().expect("No context provided!!!. A user should be provided with `User`"); // details of current user;
    
    let users_details: UseStateHandle<HashMap<i32, User>> /* User's id || User */= use_state(|| HashMap::new()); // details of all message's users;

    let current_room_details: UseReducerHandle<CurrentRoomState> = use_context().expect("No context provided!!!. A user should be provided with `UseReducerHandle<CurrentRoomState>`");


    html! {
        <>
        <h2>{"Your all messages shown here"}</h2>
        <ul>
            {
                current_room_messages.messages.iter().map(|message| {                          
                    html! {
                        if message.user_id == user_details.id {
                            <li>{"You: "}{message.msg.clone()}</li>
                        }
                        else {
                            if let Some(user_map) = current_room_details.current_room_users.clone() {
                                <li>{format!("{}: ", user_map.get(&message.user_id).unwrap().nickname)} {message.msg.clone()}</li>
                            }
                            else  {
                                <li>{message.msg.clone()}</li>
                            }    
                            
                        }
                    }
                }).collect::<Html>()
            }
        </ul>
        <p>{(format!("{:?}", *users_details).clone())}</p>
        </>
    }
}
