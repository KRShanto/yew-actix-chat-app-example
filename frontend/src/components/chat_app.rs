#![allow(dead_code, unused)]
use gloo_storage::{LocalStorage, Storage};
use reqwasm::http::{FormData, Request};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::rc::Rc;
use uuid::Uuid;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::WebSocket;
use web_sys::{Element, HtmlDivElement, HtmlElement, HtmlInputElement};
use weblog::{console_log, console_warn};
use yew::prelude::*;

use crate::{
    components::{ChatBody, ChatHeader, CreateRoom, JoinRoom, JoinRoomRequests},
    reducers::{
        CurrentRoomAction, CurrentRoomMessageAction, CurrentRoomMessageState, CurrentRoomState,
        RoomListAction, RoomListState,
    },
    websocket::{
        ws_onclose, ws_onerror, ws_onmessage, ws_opopen, MessageInfoForServer,
        UserAndRoomIDForServer, UserIDandRoomIDforServer, WebsocketServerCommand, WsRoomID,
    },
};

// Struct for holding details about any room;
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Room {
    pub id: i32,
    pub nickname: String,
    pub img_url: String,
}

// User's full info
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct User {
    pub id: i32,
    pub nickname: String,
    pub username: String,
    pub password: String,
    pub img_url: String,
}

// Message for a room;
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Message {
    pub id: i32,
    pub msg: String,
    pub user_id: i32,
    pub room_id: i32,
}
// *************** User and Room id send for getting new messages ***************** //
#[derive(Debug, Serialize, Deserialize)]
pub struct UserAndRoomID {
    pub user_id: i32, // user's id
    pub room_id: i32, // room's id
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoomID {
    room_id: i32, // room's id
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserID {
    user_id: i32, // user's id
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoomInfo {
    pub img_url: String,
    pub nickname: String,
    pub user_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserIDAndRoomID {
    pub user_id: i32, // user's id
    pub room_id: i32, // room's id
}

#[derive(PartialEq, Clone)]
pub struct MessageBarRef(pub NodeRef);

// pub struct CreateNewRoomRender(pub bool);

#[derive(PartialEq)]
pub struct JoinRoomRender(pub bool);

#[derive(PartialEq)]
pub struct ChatInputRender(pub bool);

#[derive(PartialEq)]
pub struct CreateNewRoomRender(pub bool);

#[derive(PartialEq)]
pub struct ChatOptionRender(pub bool);

#[derive(PartialEq)]
pub struct JoinRoomRequestsRender(pub bool);

#[derive(PartialEq)]
pub struct CreateAccountRender(pub bool);

#[derive(PartialEq)]
pub struct LoginRender(pub bool);

pub fn no_context_error(context: &str) -> String {
    format!("No context found for {}", context)
}
pub fn image_link(img_url: &str) -> String {
    format!("http://127.0.0.1:8000/get-user-image/{}", img_url)
}

/// The main entry point for the chat application
#[function_component(ChatApp)]
pub fn chatapp() -> Html {
    /// Main Component for the chat application
    let room_list = use_reducer(RoomListState::new); // list of all rooms that the user is currently joined
    let user_details: User = LocalStorage::get("user_info").unwrap(); // details of user. // TODO: For now I am getting these values from localhost, later I will use cookies instead of localhost
    let current_room_details = use_reducer(CurrentRoomState::new);
    let current_room_messages = use_reducer(CurrentRoomMessageState::new);

    let ws: UseStateHandle<Option<WebSocket>> = use_state(|| None); // I will use async approach here later.

    /// reference of MessageBar component;
    let message_bar_ref = MessageBarRef(NodeRef::default());

    /// Some rendering states
    let create_new_room_render = use_state(|| CreateNewRoomRender(false));
    let join_room_render = use_state(|| JoinRoomRender(false));
    let chat_input_render = use_state(|| ChatInputRender(false));
    let join_room_requests_render = use_state(|| JoinRoomRequestsRender(false));
    let chat_option_render = use_state(|| ChatOptionRender(false));

    {
        // First render
        let ws = ws.clone();

        use_effect_with_deps(
            move |_| {
                // TODO: I need to make the ws = Option<WebSocket> in async way so that ws.await.send() can wait when the websocket is connected;
                // TODO: I need to learn async and then I will start from `frontend/src/main:78`

                ws.set(Some(WebSocket::new("ws://127.0.0.1:8000/ws/").expect(
                    "Websocket connection failed, maybe you forgot to start the server",
                )));

                move || ()
            },
            (),
        );
    }
    {
        // When the `ws` state changes.
        let ws = ws.clone();
        let user_details = user_details.clone();
        let current_room_messages = current_room_messages.clone();
        let current_room_details = current_room_details.clone();
        let room_list = room_list.clone();

        use_effect_with_deps(
            move |ws| {
                let ws = ws.clone();

                if let Some(_) = (*ws).clone() {
                    ws_opopen((*ws).clone().unwrap(), user_details.clone());
                    ws_onerror((*ws).clone().unwrap());
                    ws_onmessage(
                        (*ws).clone().unwrap(),
                        current_room_messages,
                        current_room_details,
                        room_list,
                    );
                    ws_onclose((*ws).clone().unwrap());
                }

                || ()
            },
            ws,
        );
    }
    {
        // When a room changes, Change the messages of that room;
        let current_room_messages = current_room_messages.clone();

        use_effect_with_deps(
            move |current_room_details| {
                // is any room is selected
                if let Some(room) = current_room_details.current_room.clone() {
                    // When the room is changed, fetch new messages.

                    // info for making post request on `get-messages` route
                    let user_room_info = RoomID { room_id: room.id };

                    // json data of `user_room_info`
                    let user_room_info_json = serde_json::to_string(&user_room_info).unwrap();

                    // {
                    spawn_local(async move {
                        let resp = Request::post("http://127.0.0.1:8000/get-messages")
                            .header("Content-Type", "application/json")
                            .body(user_room_info_json)
                            .send()
                            .await
                            .unwrap();

                        // getting messages from the response
                        let response_messages = resp.json::<Vec<Message>>().await.unwrap();

                        current_room_messages
                            .dispatch(CurrentRoomMessageAction::ResetMessages(response_messages));
                    });
                    // }
                }

                || ()
            },
            current_room_details.clone(),
        );
    }
    {
        // When a room changes, fetch users inside that room and put their info inside the current room state;
        use_effect_with_deps(
            move |current_room_details| {
                // putting users into `current_room_details.current_room_users`
                if let None = current_room_details.current_room_users {
                    if let Some(current_room) = current_room_details.current_room.clone() {
                        let current_room_details = current_room_details.clone();
                        spawn_local(async move {
                            let room_info = RoomID {
                                room_id: current_room.id,
                            };
                            let room_info_json = serde_json::to_string(&room_info).unwrap();

                            let resp = Request::post("http://127.0.0.1:8000/get-users-from-room")
                                .header("Content-Type", "application/json")
                                .body(room_info_json)
                                .send()
                                .await
                                .unwrap();

                            // getting users from the response
                            let response_users = resp.json::<Vec<User>>().await.unwrap();

                            current_room_details
                                .dispatch(CurrentRoomAction::PutUsers(response_users))
                        });
                    }
                }

                || ()
            },
            current_room_details.clone(), // dependents
        );
    }
    {
        // When a room changes, change refetch the join reqeusts;
        let current_room_details = current_room_details.clone();
        use_effect_with_deps(
            move |current_room_details| {
                if let Some(room) = current_room_details.current_room.clone() {
                    // info for making post request on `get-join-requests` route
                    let room_info = RoomID { room_id: room.id };

                    // json data of `user_room_info`
                    let room_info_json = serde_json::to_string(&room_info).unwrap();

                    let current_room_details = current_room_details.clone();
                    spawn_local(async move {
                        let resp = Request::post("http://127.0.0.1:8000/get-join-requests")
                            .header("Content-Type", "application/json")
                            .body(room_info_json)
                            .send()
                            .await
                            .unwrap();

                        // getting messages from the response
                        let response_messages = resp.json::<Vec<User>>().await.unwrap();

                        current_room_details
                            .dispatch(CurrentRoomAction::PutJoinRequests(response_messages));
                    });
                }

                || ()
            },
            current_room_details,
        );
    }
    {
        /// Getting all rooms for the current user;
        let room_list = room_list.clone();

        use_effect_with_deps(
            move |_| {
                let user_info = UserID {
                    user_id: user_details.id,
                };

                // json data of `user_info`
                let user_info_json = serde_json::to_string(&user_info).unwrap();

                // Getting all rooms for the logged in user
                spawn_local(async move {
                    let resp = Request::post("http://127.0.0.1:8000/get-rooms")
                        .header("Content-Type", "application/json")
                        .body(user_info_json)
                        .send()
                        .await
                        .unwrap();

                    let all_rooms: Vec<Room> = resp.json::<Vec<Room>>().await.unwrap();

                    for room in all_rooms {
                        room_list.dispatch(RoomListAction::AddRoom(room));
                    }
                });

                || ()
            },
            (),
        );
    }
    {
        /// When a room changes, change the state ChatInputRender;
        let chat_input_render = chat_input_render.clone();
        let current_room_details = current_room_details.clone();
        let chat_option_render = chat_option_render.clone();

        use_effect_with_deps(
            move |current_room_details| {
                match current_room_details.current_room.clone() {
                    Some(room) => {
                        console_log!(format!("{:?}", room));
                        chat_input_render.set(ChatInputRender(true));
                        chat_option_render.set(ChatOptionRender(true));
                    }
                    None => {
                        chat_input_render.set(ChatInputRender(false));
                    }
                }
                || ()
            },
            current_room_details,
        )
    }
    {
        /// When a message changes, the element #message-bar should scroll to the bottom;
        let current_room_messages = current_room_messages.clone();
        let message_bar_ref = message_bar_ref.clone();

        use_effect_with_deps(
            move |current_room_messages| {
                console_log!("Message changed");
                let message_bar = message_bar_ref.0.cast::<Element>();
                if let Some(message_bar) = message_bar {
                    message_bar.scroll_by_with_x_and_y(0.0, 10000000.0);
                }

                || ()
            },
            (current_room_messages),
        )
    }

    // let ws = ws.clone();

    html! {
        <>

            <ContextProvider <UseStateHandle<Option<WebSocket>>> context={ws}>
            <ContextProvider <UseReducerHandle<CurrentRoomMessageState>> context={current_room_messages.clone()}>
            <ContextProvider <UseReducerHandle<CurrentRoomState>> context={current_room_details.clone()}>
            <ContextProvider <UseReducerHandle<RoomListState>> context={room_list.clone()}>
            <ContextProvider <User> context={user_details.clone()}>
            // <ContextProvider<UseStateHandle<CreateNewRoomRender>> context={create_new_room_render.clone()}>
            // <ContextProvider<UseStateHandle<JoinRoomRender>> context={join_room_render.clone()}>
            // <ContextProvider<UseStateHandle<ChatInputRender>> context={chat_input_render.clone()}>
            // <ContextProvider<UseStateHandle<JoinRoomRequestsRender>> context={join_room_requests_render.clone()}>
            // <ContextProvider<UseStateHandle<ChatOptionRender>> context={chat_option_render.clone()}>
            // <ContextProvider<MessageBarRef> context={message_bar_ref.clone()}>

                <main id="main-chat-app">
                    <ChatHeader
                        create_new_room_render = {create_new_room_render.clone()}
                        join_room_render = {join_room_render.clone()}
                    />
                    <ChatBody
                        {chat_option_render}
                        {chat_input_render}
                        {message_bar_ref}
                        join_room_requests_render={join_room_requests_render.clone()}
                    />

                    if (*create_new_room_render).0{
                        <CreateRoom {create_new_room_render} />
                    }
                    if (*join_room_render).0 {
                        <JoinRoom {join_room_render} />
                    }
                    if (*join_room_requests_render).0 {
                        <JoinRoomRequests  {join_room_requests_render} />
                    }

                </main>
            // </ContextProvider<MessageBarRef>>
            // </ContextProvider<UseStateHandle<ChatOptionRender>>>
            // </ContextProvider<UseStateHandle<JoinRoomRequestsRender>>>
            // </ContextProvider<UseStateHandle<ChatInputRender>>>
            // </ContextProvider<UseStateHandle<JoinRoomRender>>>
            // </ContextProvider<UseStateHandle<CreateNewRoomRender>>>
            </ ContextProvider <User> >
            </ContextProvider<UseReducerHandle<RoomListState>>>
            </ContextProvider<UseReducerHandle<CurrentRoomState>>>
            </ContextProvider<UseReducerHandle<CurrentRoomMessageState>>>
            </ContextProvider <UseStateHandle<Option<WebSocket>>>>
        </>
    }
}
