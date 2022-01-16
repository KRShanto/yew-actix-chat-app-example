use gloo_storage::{LocalStorage, Storage};
use reqwasm::http::{FormData, Request};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use weblog::console_log;
use yew::prelude::*;

use crate::{
    reducers::{RoomListAction, RoomListState},
    Room, User,
};

// *************** Room's info send to server; ***************** //
#[derive(Debug, Serialize, Deserialize)]
pub struct RoomInfo {
    user_id: i32,
    nickname: String,
    img_url: String,
}

// *************** Room's info comes from server; ***************** //
// #[derive(Debug, Serialize, Deserialize)]
// pub struct Room {
//     id: i32,
//     user_id: i32,
//     nickname: String,
//     img_url: String,
// }

#[function_component(CreateRoom)]
pub fn create_room() -> Html {
    let user_id_state: UseStateHandle<Option<i32>> = use_state(|| None);
    let nickname_ref = NodeRef::default();
    let img_ref = NodeRef::default();
    let room_list = use_context::<UseReducerHandle<RoomListState>>().expect("No context provided!!!. A prop should be provided with `<UseReducerHandle<RoomListState>>`"); // list of all rooms;

    // Getting user's id from localhost
    {
        let user_id_state = user_id_state.clone();
        use_effect_with_deps(
            move |_| {
                let user_info: User = LocalStorage::get("user_info").unwrap();
                user_id_state.set(Some(user_info.id));

                // TODO: I will do this work(get the user details from localstorage) in main component, then pass the info to this component as a prop;
                || ()
            },
            (),
        );
    }

    html! {
        <>
        <br />
        <hr />

        <h1>{"Create Room"}</h1>

            <label for="create-group-nickname">{"Enter a name for this group"}</label>
            <input ref={nickname_ref.clone()} type="text" name="create-group-nickname" id="create-group-nickname" />

            <label for="create-group-image">{"Give an image"}</label>
            <input ref={img_ref.clone()} type="file" name="create-group-image" id="create-group-image" />

            <button onclick={ move |_| {

                let nickname = nickname_ref
                    .cast::<HtmlInputElement>()
                    .expect("You must enter a nickname")
                    .value();
                let img = img_ref
                    .cast::<HtmlInputElement>()
                    .expect("You must enter a image")
                    .files()
                    .unwrap();
                let img_url =
                    Uuid::new_v4().to_string() + "----" + &img.get(0).expect("You must enter a image").name();

                // send to the server
                {
                    let user_id_state = user_id_state.clone();
                    let room_list = room_list.clone();
                    // TODO: I am not doing the currect way. Later on I will verify if the user is logged in or not and then I will send this request for creating group; For now I am assuming that the user is valid
                    spawn_local(async move {
                        let room_info = RoomInfo {
                            img_url: img_url.clone(),
                            nickname: nickname.clone(),
                            user_id: (*user_id_state).unwrap().clone(),
                        };

                        console_log!(format!("{:?}", room_info));
                        let room_info_json = serde_json::to_string(&room_info).unwrap();

                        let resp = Request::post("http://127.0.0.1:8000/create-room")
                            .header("Content-Type", "application/json")
                            .body(room_info_json)
                            .send()
                            .await
                            .unwrap();

                        // Server will send the newly created user with `id`
                        let room_info = resp.json::<Room>().await.unwrap();

                        if resp.status() == 200 {
                            // Uploading image
                            let form_data = FormData::new().unwrap();
                            form_data. set_with_blob_and_filename("myform", &img.clone().get(0).unwrap(), &img_url).unwrap() ;
                            {

                                let room_info  = room_info.clone();
                                spawn_local(async move {

                                    let resp = Request::post("http://127.0.0.1:8000/upload-image")
                                    .body(form_data)
                                    .send()
                                    .await
                                    .unwrap();


                                    if resp.status() == 200 {
                                        // Incrementing the room to the `RoomListState` state;
                                        room_list.dispatch(RoomListAction::AddRoom(room_info.clone()));

                                        console_log!("The image of the new room has successfully uploaded");
                                    }

                                });
                            }
                            console_log!(format!("new room has been created, room: {:?}", room_info.clone()));
                        };
                    });
                }
            }}>{"Create"}</button>

            <hr />
            <br />
        </>
    }
}
