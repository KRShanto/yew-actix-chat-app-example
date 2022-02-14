use gloo_storage::LocalStorage;
use gloo_storage::Storage;
use reqwasm::http::{FormData, Request};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::components::{
    chat_app::{server_url, CreateAccountRender, User},
    Highlight,
};

// User info send to the server //
#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    username: String,
    password: String,
    nickname: String,
    img_url: String,
}

// props of the ```CreateAccount``` component
#[derive(PartialEq, Properties)]
pub struct CreateAccountProps {
    pub create_account_render: UseStateHandle<CreateAccountRender>,
}

// Creates a new account/sign up account
// TODO: For now I am not validating anything. Later on I will.
// This component will be called by the ```App``` component
#[function_component(CreateAccount)]
pub fn create_account(props: &CreateAccountProps) -> Html {
    // Some references of <input /> field
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

            // Joining the original image url with the uuid for making this a unique path/url
            let img_url = Uuid::new_v4().to_string()
                + "----"
                + &img.get(0).expect("You must enter an image").name();

            spawn_local(async move {
                // user's info for making post requests
                let user_info = UserInfo {
                    username,
                    password,
                    nickname,
                    img_url: img_url.clone(),
                };

                // json data of `user_info`
                let user_info = serde_json::to_string(&user_info).unwrap();

                // Making post request to creating new User
                let resp = Request::post(&server_url(Some("auth/sign-up")))
                    .header("Content-Type", "application/json")
                    .body(user_info)
                    .send()
                    .await
                    .unwrap();

                if resp.status() == 200 {
                    // TODO: Show a success message to the user;

                    // Server will send the newly created `User` with `id`
                    let user_info = resp.json::<User>().await.unwrap();

                    // Image file's data
                    let form_data = FormData::new().unwrap();
                    form_data
                        .set_with_blob_and_filename(
                            "myform",
                            &img.clone().get(0).unwrap(),
                            &img_url,
                        )
                        .unwrap();

                    // Uploading user's image
                    Request::post(&server_url(Some("upload-image")))
                        .body(form_data)
                        .send()
                        .await
                        .unwrap()
                        .ok();

                    // Saving user's info in localstorage;
                    // TODO: Later on I will store these info in cookies. For now, I am storing them in localstorage
                    LocalStorage::set("user_info", user_info).unwrap();

                    // The `Window` object of javascript
                    let window = web_sys::window().expect("No Window object found!!");
                    // The `Document` object of javascript
                    let document = window.document().expect("No Document object found!!");
                    // The `Location` object of javascript
                    let location = document.location().expect("No Location object found!!");

                    // reload the window for update all the states according to the new user
                    location.reload().unwrap();
                }
            });
        }
    };

    let on_cancel = {
        move |_| {
            // Hide the component;
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
