use gloo_storage::LocalStorage;
use gloo_storage::Storage;
use reqwasm::http::{FormData, Request};
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use uuid::Uuid;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::WebSocket;
use web_sys::{Element, HtmlDivElement, HtmlElement, HtmlInputElement};
use weblog::{console_log, console_warn};
use yew::prelude::*;

use crate::{
    components::{
        chat_app::{
            no_context_error, CreateAccountRender, JoinRoomRender, Room, RoomInfo, User, UserID,
            UserIDAndRoomID,
        },
        Highlight,
    },
    websocket::{UserAndRoomIDForServer, WebsocketServerCommand},
};

// *********** User info send to the server ************* //
#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    username: String,
    password: String,
    nickname: String,
    img_url: String,
}

#[derive(PartialEq, Properties)]
pub struct CreateAccountProps {
    pub create_account_render: UseStateHandle<CreateAccountRender>,
}

/// Creates a new account/sign up account
// TODO: For now I am not validating anything. Later on I will.
#[function_component(CreateAccount)]
pub fn create_account(props: &CreateAccountProps) -> Html {
    let nickname_ref = NodeRef::default();
    let username_ref = NodeRef::default();
    let password_ref = NodeRef::default();
    let img_ref = NodeRef::default();

    let create_account_render = props.create_account_render.clone();

    let on_submit = {
        let nickname_ref = nickname_ref.clone();
        let username_ref = username_ref.clone();
        let password_ref = password_ref.clone();
        let img_ref = img_ref.clone();
        let create_account_render = create_account_render.clone();

        move |_| {
            let nickname = nickname_ref
                .cast::<HtmlInputElement>()
                .expect("You need to enter a nickname")
                .value();
            let username = username_ref
                .cast::<HtmlInputElement>()
                .expect("You need to enter a username")
                .value();
            let password = password_ref
                .cast::<HtmlInputElement>()
                .expect("You need to enter a password")
                .value();
            let img = img_ref
                .cast::<HtmlInputElement>()
                .expect("YOu need to give an image")
                .files()
                .unwrap();
            let img_url = Uuid::new_v4().to_string() + "----" + &img.get(0).unwrap().name();

            let create_account_render = create_account_render.clone();

            spawn_local(async move {
                let user_info = UserInfo {
                    username,
                    password,
                    nickname,
                    img_url: img_url.clone(),
                };

                let user_info_json = serde_json::to_string(&user_info).unwrap();

                /// Sending user's info to the Server.
                let resp = Request::post("http://127.0.0.1:8000/auth/sign-up")
                    .header("Content-Type", "application/json")
                    .body(user_info_json)
                    .send()
                    .await
                    .unwrap();

                // Server will send the newly created user with `id`
                let user_info = resp.json::<User>().await.unwrap();

                if resp.status() == 200 {
                    // TODO: Show a success message to the user;

                    /// Hide the component
                    create_account_render.set(CreateAccountRender(false));

                    let form_data = FormData::new().unwrap();
                    form_data.set_with_blob_and_filename(
                        "myform",
                        &img.clone().get(0).unwrap(),
                        &img_url,
                    );

                    /// Uploading user's image
                    spawn_local(async move {
                        let resp = Request::post("http://127.0.0.1:8000/upload-image")
                            .body(form_data)
                            .send()
                            .await
                            .unwrap()
                            .ok();
                    });

                    /// Saving user's info in localstorage;
                    // TODO: Later on I will store these info in cookies. For now, I am storing them in localstorage
                    LocalStorage::set("user_info", user_info).unwrap();
                    console_log!(
                                "Your account has been created successfully and you are logged in automatically"
                            );
                }
            });
        }
    };

    let on_cancel = {
        move |_| {
            /// Hide the component;
            create_account_render.set(CreateAccountRender(false));
        }
    };

    html! {
        <>
        <Highlight>
            <section class="form">

                <h1 class="form-title">{"Create an YewChat account"}</h1>

                <div class="form-wrapper">
                    <label for="nickname">{"Enter your Nickname"}</label>
                    <input ref={nickname_ref} type="text" id="nickname" />
                </div>

                <div class="form-wrapper">
                    <label for="username">{"Enter your Username"}</label>
                    <input ref={username_ref} type="text" id="username" />
                </div>

                <div class="form-wrapper">
                    <label for="password">{"Enter your Password"}</label>
                    <input ref={password_ref} type="text" /> // TODO: For now I am making type="text". But later I will make type="password"
                </div>

                <div class="form-wrapper">
                    <label
                    for="img"
                    style="
                            background-color: rgb(55, 119, 158); 
                            padding: 0.5rem;
                            font-size: 1.5rem; 
                            border-radius: 0.5rem;
                    "   
                    >
                        {"Give your image"}
                    </label>
                    <input ref={img_ref} type="file" id="img" accept="image/*" style="display: none;"/>
                </div>

                <div class="buttons-div">
                    <button class="submit-btn" onclick={on_submit}>{"Create Account"}</button>
                    <button class="cancel-btn" onclick={on_cancel}>{"Cancel"}</button>
                </div>

            </section>
        </Highlight>
        </>
    }
}
