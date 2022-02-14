use web_sys::HtmlInputElement;
use web_sys::WebSocket;
use yew::prelude::*;

use crate::{
    components::chat_app::{no_context_error, User},
    reducers::CurrentRoomState,
    websocket::{MessageInfoForServer, WebsocketServerCommand},
};

// Message/Text sending component
// This component will send the message from the client to the room
// This component is called by the ```ChatBar``` component
#[function_component(ChatInput)]
pub fn chat_input() -> Html {
    // TODO: For now the component only sends the data if the user click on the button, later it will send the data when the user hits Enter
    // reference of the <input /> element
    let input_ref = &NodeRef::default();

    // onclick event of the <button> element
    let onclick = {
        let input_ref = input_ref.clone();

        let ws: UseStateHandle<Option<WebSocket>> =
            use_context().expect(&no_context_error("UseStateHanlde<Option<WebSocket>>"));

        let current_room_details: UseReducerHandle<CurrentRoomState> =
            use_context().expect(&no_context_error("UseReducerHandle<CurrentRoomState>"));

        let user_details: User = use_context().expect(&no_context_error("User"));

        move |_| {
            // <input /> element
            let input = input_ref.cast::<HtmlInputElement>().unwrap();

            // value of the input element
            let input_value = input.value();

            // It doesn't make sense to send blank/empty messages to the room.
            // So checking the length of the input element
            // It its greater than 0 then send the message to the room
            if input_value.len() > 0 {
                // Send this message to websocket
                if let Some(ws) = &*ws {
                    ws.send_with_str(
                        &serde_json::to_string(&MessageInfoForServer {
                            msg: input_value,
                            command_type: WebsocketServerCommand::SendMessage,
                            room_id: current_room_details.current_room.clone().unwrap().id, // TODO: I will handle this error later
                            user_id: user_details.id,
                        })
                        .unwrap(),
                    )
                    .unwrap();

                    // make the input field empty
                    input.set_value("");

                    // Focus on the input field
                    input.focus().unwrap();
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
                ref={input_ref}
            />
            <button {onclick}>{"Send"}</button>

        </section>
        </>
    }
}
