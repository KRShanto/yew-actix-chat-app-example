use reqwasm::http::{FormData, Request};
use uuid::Uuid;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use weblog::console_log;
use yew::prelude::*;

use crate::{
    components2::{
        chat_app::{no_context_error, CreateNewRoomRender, Room, RoomInfo, User},
        Highlight,
    },
    reducers::{RoomListAction, RoomListState},
};

#[derive(PartialEq, Properties)]
pub struct CreateRoomProps {
    pub create_new_room_render: UseStateHandle<CreateNewRoomRender>,
}

#[function_component(CreateRoom)]
pub fn create_room(props: &CreateRoomProps) -> Html {
    // let create_new_room_render: UseStateHandle<CreateNewRoomRender> =
    //     use_context().expect(&no_context_error("UseStateHanlde<CreateNewRoomRender>"));
    let create_new_room_render = props.create_new_room_render.clone();
    let user_details: User = use_context().expect(&no_context_error("User"));
    let nickname_ref = NodeRef::default();
    let img_ref = NodeRef::default();
    let room_list = use_context::<UseReducerHandle<RoomListState>>().expect("No context provided!!!. A prop should be provided with `<UseReducerHandle<RoomListState>>`"); // list of all rooms;

    let submit_form = {
        let nickname_ref = nickname_ref.clone();
        let img_ref = img_ref.clone();
        let create_new_room_render = create_new_room_render.clone();

        move |_| {
            let nickname = nickname_ref
                .cast::<HtmlInputElement>()
                .expect("You must enter a nickname")
                .value();

            let img = img_ref
                .cast::<HtmlInputElement>()
                .expect("You must enter a image")
                .files()
                .unwrap();

            let img_url = Uuid::new_v4().to_string()
                + "----"
                + &img.get(0).expect("You must enter a image").name();

            let room_list = room_list.clone();
            let create_new_room_render = create_new_room_render.clone();
            // TODO: I am not doing the currect way. Later on I will verify if the user is logged in or not and then I will send this requestfor creating group; For now I am assuming that the user is valid
            /// Sending room's info;
            spawn_local(async move {
                let room_info = RoomInfo {
                    img_url: img_url.clone(),
                    nickname: nickname.clone(),
                    user_id: user_details.id,
                };

                let room_info_json = serde_json::to_string(&room_info).unwrap();

                let resp = Request::post("http://127.0.0.1:8000/create-room")
                    .header("Content-Type", "application/json")
                    .body(room_info_json)
                    .send()
                    .await
                    .unwrap();

                // Server will send the newly created user with `id`
                let room_info = resp.json::<Room>().await.unwrap();

                let form_data = FormData::new().unwrap();
                form_data
                    .set_with_blob_and_filename("myform", &img.clone().get(0).unwrap(), &img_url)
                    .unwrap();

                /// Uploading image
                let resp = Request::post("http://127.0.0.1:8000/upload-image")
                    .body(form_data)
                    .send()
                    .await
                    .unwrap();

                if resp.status() == 200 {
                    /// Incrementing the room to the `RoomListState` state;
                    room_list.dispatch(RoomListAction::AddRoom(room_info.clone()));
                    console_log!("The image of the new room has successfully uploaded");

                    /// Success ///
                    create_new_room_render.set(CreateNewRoomRender(false));

                    console_log!(format!(
                        "new room has been created, room: {:?}",
                        room_info.clone()
                    ));
                    // TODO: I will show a success message to the user
                }
            });
        }
    };

    let cancel_form = {
        move |_| {
            create_new_room_render.set(CreateNewRoomRender(false));
        }
    };

    html! {
        <>
        <Highlight >

            <section class="form">
                <center>
                <h1 class="form-title">{"Create Room"}</h1>
                </center>

                <div class="form-wrapper">
                    <label for="create-group-nickname">{"Enter a name for this group"}</label>
                    <input ref={nickname_ref.clone()} type="text" name="create-group-nickname" id="create-group-nickname" />
                </div>

                <div class="form-wrapper">
                    <label
                        for="create-group-image"
                        style="
                            background-color: rgb(55, 119, 158); 
                            padding: 0.5rem;
                            font-size: 1.5rem; 
                            border-radius: 0.5rem;
                        "
                    >
                        {"Choose an image"}
                    </label>
                    <input
                        ref={img_ref.clone()}
                        type="file"
                        name="create-group-image"
                        id="create-group-image"
                        style="display:none;"
                    />
                </div>

                <div class="buttons-div">
                    <button class="submit-btn" onclick={submit_form}>{"Create"}</button>
                    <button class="cancel-btn" onclick={cancel_form}>{"Cancel"}</button>
                </div>

            </section>

            </Highlight>
        </>
    }
}
