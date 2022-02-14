use web_sys::WebSocket;
use weblog::console_warn;
use yew::prelude::*;

use crate::{
    components::chat_app::{image_link, Room},
    reducers::{CurrentRoomAction, CurrentRoomState},
    websocket::{WebsocketServerCommand, WsRoomID},
};

// props of ```RoomComponent``` component
#[derive(PartialEq, Properties)]
pub struct RoomComponentProps {
    pub room: Room,
}

// Component for each rooms
// This component is called by the ```RoomBar``` component
#[function_component(RoomComponent)]
pub fn room_component(props: &RoomComponentProps) -> Html {
    let room = &props.room;

    let current_room_details = use_context::<UseReducerHandle<CurrentRoomState>>().expect(
        "No context provided!!!. A context should be provided with `UseReducerHandle<CurrentRoomState>"
    );
    let ws = use_context::<UseStateHandle<Option<WebSocket>>>()
        .expect("No context provided!!!. A context should be provided with `UseStateHandle<Option<WebSocket>>`");

    // onclick event of <section class="chat-room"> element
    let onclick = {
        let room = room.clone();

        move |_| {
            // changing the current room state;
            current_room_details.dispatch(CurrentRoomAction::SelectRoom(room.clone()));

            // executing `ChangeRoom` command on websocket to set this room in websocket server;
            if let Some(ws) = &*ws {
                ws.send_with_str(
                    &serde_json::to_string(&WsRoomID {
                        command_type: WebsocketServerCommand::ChangeRoom,
                        room_id: room.id,
                    })
                    .unwrap(),
                )
                .unwrap();
            } else {
                console_warn!("Websocket is not ready yet. The value of the context is still None;")
            }
        }
    };

    html! {
        <>
        <section class="chat-room" {onclick} >
            <div class="room-image">
                <img src={image_link(&room.img_url)} alt="Room image" />
            </div>
            <div class="names">
                <h1 class="nickname">{&room.nickname}</h1>
                <p class="id">{room.id}</p>
            </div>
        </section>
        </>
    }
}
