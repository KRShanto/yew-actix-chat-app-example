#![allow(dead_code, unused)]
use gloo_storage::LocalStorage;
use gloo_storage::Storage;
use reqwasm::http::FormData;
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use weblog::console_log;
use yew::prelude::*;

// *********** User info send to the server ************* //
#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    username: String,
    password: String,
    nickname: String,
    img_url: String,
}

#[function_component(Signup)]
pub fn signup() -> Html {
    let nickname_ref = NodeRef::default();
    let username_ref = NodeRef::default();
    let password_ref = NodeRef::default();
    let img_ref = NodeRef::default();

    html! {
        <>
            <div class="signup">
                <label for="nickname" >{"Enter your nickname"}</label>
                <input ref={nickname_ref.clone()} type="text" id="nickname" value="5" />

                <label for="username" >{"Enter your username"}</label>
                <input ref={username_ref.clone()} type="text" id="username" value="5"/>

                <label for="password" >{"Enter your password"}</label>
                <input ref={password_ref.clone()} type="text" id="password" value="5" />

                <label for="img" >{"Enter your img"}</label>
                <input ref={img_ref.clone()} type="file" id="img" accept="image/*"/>

                <button onclick={move |_| {

                    let nickname = nickname_ref.cast::<HtmlInputElement>().unwrap().value();
                    let username = username_ref.cast::<HtmlInputElement>().unwrap().value();
                    let password = password_ref.cast::<HtmlInputElement>().unwrap().value();
                    let img = img_ref.cast::<HtmlInputElement>().unwrap().files().unwrap();
                    let img_url =  Uuid::new_v4().to_string() + "----" +   &img.get(0).unwrap().name();


                    spawn_local(async move {
                        let user_info = UserInfo {
                            username,
                            password,
                            nickname,
                            img_url
                        };

                        let user_info_json = serde_json::to_string(&user_info).unwrap();

                        let resp = Request::post("http://127.0.0.1:8000/auth/sign-up")
                                .header("Content-Type", "application/json")
                                .body(user_info_json)
                                .send()
                                .await
                                .unwrap();


                        if resp.status() == 200 {
                            // Saving user's info in localstorage;
                            // TODO: Later on I will store these info in cookies. For now, I am storing them in localstorage
                            LocalStorage::set("user_info", user_info).unwrap();
                            console_log!("Your account has been created successfully and you are logged in automatically");
                        }
                    });

                }}>{"Create account"}</button>

                </div>
                </>
    }
}
