use reqwasm::http::Request;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use web_sys::WebSocket;
use yew::prelude::*;

use crate::{
    components::{
        chat_app::{no_context_error, server_url, JoinRoomRender, User, UserIDAndRoomID},
        Highlight,
    },
    websocket::{UserAndRoomIDForServer, WebsocketServerCommand},
};

#[derive(PartialEq, Properties)]
pub struct JoinRoomProps {
    pub join_room_render: UseStateHandle<JoinRoomRender>,
}

// This component is for joining a room. Now for this component will ask the room's id. But in future versions, it will ask the uuid of that room
// This component will be called by the ```ChatApp``` component
#[function_component(JoinRoom)]
pub fn join_room(props: &JoinRoomProps) -> Html {
    let join_room_render = props.join_room_render.clone();
    let user_details: User = use_context().expect(&no_context_error("User"));

    // Reference of <input /> element
    let input_ref = NodeRef::default();

    let ws: UseStateHandle<Option<WebSocket>> =
        use_context().expect(&no_context_error("UseStateHandle<Option<WebSocket>>"));

    // onclick event of <button class="submit-btn"> element
    let submit_form = {
        let input_ref = input_ref.clone();
        let join_room_render = join_room_render.clone();

        move |_| {
            let user_id = user_details.id;

            // value of <input /> element
            let room_id_result = input_ref
                .cast::<HtmlInputElement>()
                .unwrap()
                .value()
                .parse::<i32>();

            // Checking if the user has entered a valid number or not
            if let Ok(room_id) = room_id_result {
                // json data for making post request
                let details = serde_json::to_string(&UserIDAndRoomID { user_id, room_id }).unwrap();

                // Making post request
                spawn_local(async move {
                    Request::post(&server_url(Some("room-join-request")))
                        .body(details)
                        .header("Content-Type", "application/json")
                        .send()
                        .await
                        .unwrap();

                    // TODO: I will show an Alert message if the response return 204 http status;
                });
                // Send the command ```SendJoinRequest``` to websocket;
                if let Some(ws) = &*ws {
                    ws.send_with_str(
                        &serde_json::to_string(&UserAndRoomIDForServer {
                            command_type: WebsocketServerCommand::SendJoinRequest,
                            img_url: user_details.img_url.clone(),
                            username: user_details.username.clone(),
                            nickname: user_details.nickname.clone(),
                            password: user_details.password.clone(),
                            user_id: user_details.id,
                            room_id: room_id,
                        })
                        .unwrap(),
                    )
                    .unwrap();
                }

                // Hide this component;
                join_room_render.set(JoinRoomRender(false));
            } else {
                //TODO: show a error message bottom of input field.
            }
        }
    };

    // onclick event of <button class="cancel-btn"> element
    let cancel_form = {
        move |_| {
            // Hide this component;
            join_room_render.set(JoinRoomRender(false));
        }
    };

    html! {
        <>
        <Highlight>
            <section class="form">

                <h1 class="form-title">{"Join Room"}</h1>

                <div class="form-wrapper">
                    <label for="join-group-id">{"Enter your group's ID"}</label>
                    <input ref={input_ref.clone()} type="number" id="join-group-id" />
                </div>

                <div class="buttons-div">
                    <button class="submit-btn" onclick={submit_form}>{"Request for Join"}</button>
                    <button class="cancel-btn" onclick={cancel_form}>{"Cancel"}</button>
                </div>

            </section>
        </Highlight>
        </>
    }
}
