use gloo_storage::{LocalStorage, Storage};
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use web_sys::Element;
use web_sys::WebSocket;
use yew::prelude::*;

use crate::{
    components::{ChatBody, ChatHeader, CreateRoom, JoinRoom, JoinRoomRequests},
    reducers::{
        CurrentRoomAction, CurrentRoomMessageAction, CurrentRoomMessageState, CurrentRoomState,
        RoomListAction, RoomListState,
    },
    websocket::{ws_onclose, ws_onerror, ws_onmessage, ws_opopen},
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

// User and Room id
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

// Reference of ```MessageBar``` component. When a message/text send from any room then that component will scroll down, so this struct will hold that component's reference and with this reference we can scroll that component
#[derive(PartialEq, Clone)]
pub struct MessageBarRef(pub NodeRef);

//if the ```JoinRoom``` component render or disappear
#[derive(PartialEq)]
pub struct JoinRoomRender(pub bool);

//if the ```ChatInput``` component render or disappear
#[derive(PartialEq)]
pub struct ChatInputRender(pub bool);

//if the ```CreateNewRoom``` component render or disappear
#[derive(PartialEq)]
pub struct CreateNewRoomRender(pub bool);

//if the ```ChatOption``` component render or disappear
#[derive(PartialEq)]
pub struct ChatOptionRender(pub bool);

//if the ```JoinRoomRequests``` component render or disappear
#[derive(PartialEq)]
pub struct JoinRoomRequestsRender(pub bool);

//if the ```CreateAccount``` component render or disappear
#[derive(PartialEq)]
pub struct CreateAccountRender(pub bool);

//if the ```LoginRender``` component render or disappear
#[derive(PartialEq)]
pub struct LoginRender(pub bool);

// The main entry point for the chat application
// Main Component for the chat application
// This component is called by the ```App``` component
#[function_component(ChatApp)]
pub fn chatapp() -> Html {
    // list of all rooms that the user is currently joined
    let room_list = use_reducer(RoomListState::new);

    // details of logged in user. // TODO: For now I am getting these values from localhost, later I will use cookies instead of localhost
    let user_details: User = LocalStorage::get("user_info").unwrap();

    // details of currently selected room
    let current_room_details = use_reducer(CurrentRoomState::new);

    // state of current room's messages.
    let current_room_messages = use_reducer(CurrentRoomMessageState::new);

    // websocket; Initially the websocket will not be valid/connected. After the first render this state will be Some(WebSocket) TODO: I will use async approach here later.
    let ws: UseStateHandle<Option<WebSocket>> = use_state(|| None);

    // reference of MessageBar component; When a user sends message, that MessageBar will be scrolled to bottom, thats why we need the reference of the MessageBar
    let message_bar_ref = MessageBarRef(NodeRef::default());

    // rendering states.
    // these states means that if the value of the last child is true, then the component will be render. else the component will not render or if already rendered, then disappear
    let create_new_room_render = use_state(|| CreateNewRoomRender(false));
    let join_room_render = use_state(|| JoinRoomRender(false));
    let chat_input_render = use_state(|| ChatInputRender(false));
    let join_room_requests_render = use_state(|| JoinRoomRequestsRender(false));
    let chat_option_render = use_state(|| ChatOptionRender(false));

    // Set the value of the `ws` //
    {
        let ws = ws.clone();

        use_effect_with_deps(
            move |_| {
                // TODO: I need to make the ws = Option<WebSocket> in async way so that ws.await.send() can wait when the websocket is connected;

                ws.set(Some(WebSocket::new("ws://127.0.0.1:8000/ws/").expect(
                    "Websocket connection failed, maybe you forgot to start the server",
                )));

                move || ()
            },
            (),
        );
    }

    // When the `ws` state changes call websocket functions //
    {
        let ws = ws.clone();
        let user_details = user_details.clone();
        let current_room_messages = current_room_messages.clone();
        let current_room_details = current_room_details.clone();
        let room_list = room_list.clone();

        use_effect_with_deps(
            move |ws| {
                let ws = ws.clone();

                // At this time it should be Option<Some> rather than None
                if let Some(ws) = (*ws).clone() {
                    ws_opopen(&ws, user_details.clone());
                    ws_onerror(&ws);
                    ws_onmessage(&ws, current_room_messages, current_room_details, room_list);
                    ws_onclose(&ws);
                }

                || ()
            },
            ws,
        );
    }

    // When a room changes, Change the messages of that room //
    {
        let current_room_messages = current_room_messages.clone();

        use_effect_with_deps(
            move |current_room_details| {
                // is any room is selected
                if let Some(room) = current_room_details.current_room.clone() {
                    // When the room is changed, fetch new messages //

                    // info for making post request on `get-messages` route
                    let user_room_info = RoomID { room_id: room.id };

                    // json data of `user_room_info`
                    let user_room_info = serde_json::to_string(&user_room_info).unwrap();

                    // make request to `get-messages` route
                    spawn_local(async move {
                        let resp = Request::post(&server_url(Some("get-messages")))
                            .header("Content-Type", "application/json")
                            .body(user_room_info)
                            .send()
                            .await
                            .unwrap();

                        // getting messages from the response
                        let response_messages = resp.json::<Vec<Message>>().await.unwrap();

                        // Calling the `ResetMessages` action so that it can reset (change all old messages with the new ones) the messages.
                        current_room_messages
                            .dispatch(CurrentRoomMessageAction::ResetMessages(response_messages));
                    });
                }

                || ()
            },
            current_room_details.clone(),
        );
    }

    // When a room changes, fetch users associated to current room and put their info inside the current room state //
    {
        use_effect_with_deps(
            move |current_room_details| {
                // If the value of `current_room_details.current_room_users` is Option<Some>>, it means that users of the current room is already fetched and the client has clicked again to the current room.
                // If the value of `current_room_details.current_room_users` is Option<None>, it means that users of the current room has not been fetched and the client has clicked the room that is not currently selected room
                //So checking if the ```current_room_users``` is None or not
                if let None = current_room_details.current_room_users {
                    if let Some(current_room) = current_room_details.current_room.clone() {
                        let current_room_details = current_room_details.clone();

                        spawn_local(async move {
                            // info for making post request on `get-users-from-room`
                            let room_info = RoomID {
                                room_id: current_room.id,
                            };

                            // json data of `room_info`
                            let room_info = serde_json::to_string(&room_info).unwrap();

                            // Making the post request
                            let resp = Request::post(&server_url(Some("get-users-from-room")))
                                .header("Content-Type", "application/json")
                                .body(room_info)
                                .send()
                                .await
                                .unwrap();

                            // getting users from the response
                            let response_users = resp.json::<Vec<User>>().await.unwrap();

                            // Putting these users into CurrentRoomState.current_room_users;
                            current_room_details
                                .dispatch(CurrentRoomAction::PutUsers(response_users))
                        });
                    }
                }

                || ()
            },
            current_room_details.clone(), // dependencies
        );
    }

    // When a room changes, fetch the join reqeusts assosiated to that room //
    {
        use_effect_with_deps(
            move |current_room_details| {
                if let Some(room) = current_room_details.current_room.clone() {
                    // info for making post request on `get-join-requests` route
                    let room_info = RoomID { room_id: room.id };

                    // json data of `user_room_info`
                    let room_info_json = serde_json::to_string(&room_info).unwrap();

                    let current_room_details = current_room_details.clone();

                    // Making post request on `get-join-requests` route
                    spawn_local(async move {
                        let resp = Request::post(&server_url(Some("get-join-requests")))
                            .header("Content-Type", "application/json")
                            .body(room_info_json)
                            .send()
                            .await
                            .unwrap();

                        // getting join requests from the response
                        let response_messages = resp.json::<Vec<User>>().await.unwrap();

                        // Putting join requests into ```CurrentRoomState.current_room_join_requests```
                        current_room_details
                            .dispatch(CurrentRoomAction::PutJoinRequests(response_messages));
                    });
                }

                || ()
            },
            current_room_details.clone(),
        );
    }

    // Getting all rooms for the current user;
    {
        let room_list = room_list.clone();

        use_effect_with_deps(
            move |_| {
                // info for making post request on `get-rooms`
                let user_info = UserID {
                    user_id: user_details.id,
                };

                // json data of `user_info`
                let user_info = serde_json::to_string(&user_info).unwrap();

                // making post request
                spawn_local(async move {
                    let resp = Request::post(&server_url(Some("get-rooms")))
                        .header("Content-Type", "application/json")
                        .body(user_info)
                        .send()
                        .await
                        .unwrap();

                    // getting rooms from the response
                    let all_rooms: Vec<Room> = resp.json::<Vec<Room>>().await.unwrap();

                    // Adding rooms to the ```RoomListState.rooms```
                    for room in all_rooms {
                        room_list.dispatch(RoomListAction::AddRoom(room));
                    }
                });

                || ()
            },
            (),
        );
    }

    // When a room changes, change the state ChatInputRender and ChatOptionRender;
    {
        let chat_input_render = chat_input_render.clone();
        let current_room_details = current_room_details.clone();
        let chat_option_render = chat_option_render.clone();

        use_effect_with_deps(
            move |current_room_details| {
                match current_room_details.current_room.clone() {
                    Some(_room) => {
                        // Displaying ```ChatInput``` component
                        chat_input_render.set(ChatInputRender(true));

                        // Displaying ```ChatOption``` component
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

    // When a message changes/sent, the component MessageBar should scroll to the bottom;
    {
        let current_room_messages = current_room_messages.clone();
        let message_bar_ref = message_bar_ref.clone();

        use_effect_with_deps(
            move |_current_room_messages| {
                // Getting the html element of the `MessageBar` component;
                let message_bar = message_bar_ref.0.cast::<Element>();

                if let Some(message_bar) = message_bar {
                    // It will scroll to the bottom
                    message_bar.scroll_by_with_x_and_y(0.0, 10000000.0);
                }

                || ()
            },
            current_room_messages,
        )
    }

    html! {
        <>

            <ContextProvider <UseStateHandle<Option<WebSocket>>> context={ws}>
            <ContextProvider <UseReducerHandle<CurrentRoomMessageState>> context={current_room_messages}>
            <ContextProvider <UseReducerHandle<CurrentRoomState>> context={current_room_details}>
            <ContextProvider <UseReducerHandle<RoomListState>> context={room_list}>
            <ContextProvider <User> context={user_details}>

            <Temporary />
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

            </ ContextProvider <User> >
            </ContextProvider<UseReducerHandle<RoomListState>>>
            </ContextProvider<UseReducerHandle<CurrentRoomState>>>
            </ContextProvider<UseReducerHandle<CurrentRoomMessageState>>>
            </ContextProvider <UseStateHandle<Option<WebSocket>>>>
        </>
    }
}

// Url of the backend server.
pub fn server_url<'a>(rest: Option<&str>) -> String {
    // The `rest` is the rest path of the server.

    if let Some(rest) = rest {
        format!("http://127.0.0.1:8000/{}", rest)
    } else {
        "http://127.0.0.1:8000".to_owned()
    }
}

pub fn no_context_error(context: &str) -> String {
    format!("No context found for {}", context)
}

pub fn image_link(img_url: &str) -> String {
    format!("{}/get-user-image/{}", server_url(None), img_url)
}

// A temporary component for experimenting some codes. Anyone can ignore this :)
#[function_component(Temporary)]
fn temp() -> Html {
    use weblog::console_log;

    let select_room = CurrentRoomAction::SelectRoom(Room {
        id: 3,
        nickname: String::from("A room"),
        img_url: String::from("an image"),
    });

    let put_users = CurrentRoomAction::PutUsers(vec![User {
        id: 3,
        username: String::from("A user"),
        nickname: String::from("A nickname"),
        img_url: String::from("an image"),
        password: String::from("A password"),
    }]);

    let put_join_requests = CurrentRoomAction::PutJoinRequests(vec![User {
        id: 3,
        username: String::from("A user"),
        nickname: String::from("A nickname"),
        img_url: String::from("an image"),
        password: String::from("A password"),
    }]);

    let append_join_requests = CurrentRoomAction::AppendJoinRequest(User {
        id: 3,
        username: String::from("A user"),
        nickname: String::from("A nickname"),
        img_url: String::from("an image"),
        password: String::from("A password"),
    });

    let remove_join_request = CurrentRoomAction::RemoveJoinRequest(5);

    let select_room_2 = Box::new(Room {
        id: 3,
        nickname: String::from("A room"),
        img_url: String::from("an image"),
    }); // 28

    let put_users_2 = Box::new(vec![User {
        id: 3,
        username: String::from("A user"),
        nickname: String::from("A nickname"),
        img_url: String::from("an image"),
        password: String::from("A password"),
    }]); // 12

    let put_join_requests_2 = Box::new(vec![
        User {
            id: 3,
            username: String::from("A user"),
            nickname: String::from("A nickname"),
            img_url: String::from("an image"),
            password: String::from("A password"),
        },
        User {
            id: 3,
            username: String::from("A user"),
            nickname: String::from("A nickname"),
            img_url: String::from("an image"),
            password: String::from("A password"),
        },
        User {
            id: 3,
            username: String::from("A user"),
            nickname: String::from("A nickname"),
            img_url: String::from("an image"),
            password: String::from("A password"),
        },
        User {
            id: 3,
            username: String::from("A user"),
            nickname: String::from("A nickname"),
            img_url: String::from("an image"),
            password: String::from("A password"),
        },
    ]); // 12

    let append_join_requests_2 = Box::new(User {
        id: 3,
        username: String::from("A user"),
        nickname: String::from("A nickname"),
        img_url: String::from("an image"),
        password: String::from("A password"),
    }); // 52

    let remove_join_requests_2 = 3; // 4

    size(&select_room, "select_room");
    size(&put_users, "put_users");
    size(&put_join_requests, "put_join_requests");
    size(&append_join_requests, "append_join_requests");
    size(&remove_join_request, "remove_join_request");

    size(&select_room_2, "select_room_2");
    size(&put_users_2, "put_users_2");
    size(&put_join_requests_2, "put_join_requests_2");
    size(&append_join_requests_2, "append_join_requests_2");
    size(&remove_join_requests_2, "remove_join_request_2");

    fn size<T>(var: &T, var_name: &str) {
        console_log!(format!(
            "The size of {var_name} is: {}",
            std::mem::size_of_val(var)
        ))
    }

    html! {
        <>

        </>
    }
}
