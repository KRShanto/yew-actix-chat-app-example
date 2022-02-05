use web_sys::HtmlInputElement;
use web_sys::WebSocket;
use yew::prelude::*;

use crate::{
    components::chat_app::{no_context_error, User},
    reducers::CurrentRoomState,
    websocket::{MessageInfoForServer, WebsocketServerCommand},
};

/// Complete**********
#[function_component(ChatInput)]
pub fn chat_input() -> Html {
    let input_ref = NodeRef::default();

    let onclick = {
        let input_ref = input_ref.clone();

        let ws: UseStateHandle<Option<WebSocket>> =
            use_context().expect(&no_context_error("UseStateHanlde<Option<WebSocket>>"));

        let current_room_details: UseReducerHandle<CurrentRoomState> =
            use_context().expect(&no_context_error("UseReducerHandle<CurrentRoomState>"));

        let user_details: User = use_context().expect(&no_context_error("User"));

        move |_| {
            let input = input_ref.cast::<HtmlInputElement>().unwrap();
            let input_value = input.value();

            if input_value.len() > 0 {
                // Send this message to websocket
                if let Some(ws) = (*ws).clone() {
                    ws.send_with_str(
                        &serde_json::to_string(&MessageInfoForServer {
                            msg: input_value,
                            command_type: WebsocketServerCommand::SendMessage,
                            room_id: current_room_details.current_room.clone().unwrap().id, // TODO: I will handle this error later
                            user_id: user_details.id,
                        })
                        .unwrap(),
                    );

                    // reset input field
                    input.set_value("");

                    // Focus on the input field
                    input.focus();
                }
            }
        }
    };

    html! {
        <>
        <section id="chat-input">

            <input
                placeholder="Send any message"
                type="text"
                ref={input_ref.clone()}
            />
            <button {onclick}>{"Send"}</button>

        </section>
        </>
    }
}
