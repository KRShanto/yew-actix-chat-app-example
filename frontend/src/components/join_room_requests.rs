use web_sys::WebSocket;
use yew::prelude::*;

use crate::{
    components::{
        chat_app::{image_link, no_context_error, JoinRoomRequestsRender, User},
        Highlight,
    },
    reducers::CurrentRoomState,
    websocket::{UserIDandRoomIDforServer, WebsocketServerCommand},
};

// props of ```JoinRoomRequests``` component
#[derive(PartialEq, Properties)]
pub struct JoinRoomRequestsProps {
    pub join_room_requests_render: UseStateHandle<JoinRoomRequestsRender>,
}
// It will show all join requests and users they requested for the current room.
// Its a popup component.
// This component will be called by the ```ChatApp``` component
#[function_component(JoinRoomRequests)]
pub fn join_room_requests(props: &JoinRoomRequestsProps) -> Html {
    let join_room_requests_render = props.join_room_requests_render.clone();

    let current_room_details: UseReducerHandle<CurrentRoomState> =
        use_context().expect(&no_context_error("UseReducerHandle<CurrentRoomState>"));

    let current_room_join_requests: Option<Vec<User>> =
        current_room_details.current_room_join_requests.clone();

    // click event of <button class="cancel-btn">
    let cancel_form = move |_| {
        // Hide this component
        join_room_requests_render.set(JoinRoomRequestsRender(false));
    };

    let ws = use_context::<UseStateHandle<Option<WebSocket>>>().expect("No context provided!!!. A context should be provided with `UseStateHandle<Option<WebSocket>>`");

    html! {

        <>
        <Highlight>
        <section id="join-room-requests-main">
        <h1 class="title">{"Join Requests"}</h1>

        if let Some(users) = current_room_join_requests {

            <div id="join_room_requests">
            {
                users.into_iter().map(|user| {
                    let ws = ws.clone();
                    let current_room_details = current_room_details.clone();

                    // onclick event of <button class="accept">
                    let on_accpet = {
                        let ws = ws.clone();
                        let current_room_details = current_room_details.clone();

                        move |_| {
                            // Send the ```AcceptJoinRequest``` command to the websocket so that it can accpet the join request
                            if let Some(ws) = &*ws {
                                ws.send_with_str(
                                    &serde_json::to_string(&UserIDandRoomIDforServer {
                                        command_type: WebsocketServerCommand::AcceptJoinRequest,
                                        user_id: user.id,
                                        room_id: current_room_details
                                        .current_room
                                            .clone()
                                            .unwrap()
                                            .id,
                                    })
                                    .unwrap(),
                                )
                                .unwrap();
                            }
                        }
                    };

                    // onclick on <button class="reject">
                    let on_reject = {
                        move |_| {
                            // Send the ```RejectRequest``` command to the websocket so that it can remove the request
                            if let Some(ws) = &*ws {
                                ws.send_with_str(
                                    &serde_json::to_string(&UserIDandRoomIDforServer {
                                        command_type: WebsocketServerCommand::RejectRequest,
                                        user_id: user.id,
                                        room_id: current_room_details
                                            .current_room
                                            .clone()
                                            .unwrap()
                                            .id,
                                    })
                                    .unwrap(),
                                )
                                .unwrap();
                            }
                        }
                    };

                    html! {
                        <div class="requests">
                            <div class="user-details">

                                <img src={image_link(&user.img_url.clone())} alt="User's image" />

                                <div class="names">
                                    <h1 class="nickname">{user.nickname.clone()}</h1>
                                    <p class="username">{user.username.clone()}</p>
                                </div>

                            </div>

                            <div class="buttons">
                                <button class="accept" onclick={on_accpet}>{"Accept"}</button>
                                <button class="reject" onclick={on_reject}>{"Reject"}</button>
                            </div>
                        </div>
                    }

                }).collect::<Html>()
            }
            </div>

        }
                <button class="cancel-btn" onclick={cancel_form}>{"Done"}</button>
            </section>
        </Highlight>
        </>
    }
}
