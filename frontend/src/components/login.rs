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
use web_sys::{Document, Element, HtmlDivElement, HtmlElement, HtmlInputElement};
use weblog::{console_error, console_log, console_warn};
use yew::prelude::*;

use crate::components::{
    chat_app::{LoginRender, User},
    Highlight,
};

#[derive(Debug, Serialize, Deserialize)]
struct UsernameAndPassword {
    username: String,
    password: String,
}

#[derive(PartialEq, Properties)]
pub struct LoginProps {
    pub login_render: UseStateHandle<LoginRender>,
}

#[function_component(Login)]
pub fn login(props: &LoginProps) -> Html {
    let login_render = props.login_render.clone();

    let username_ref = NodeRef::default();
    let password_ref = NodeRef::default();

    let on_submit = {
        let username_ref = username_ref.clone();
        let password_ref = password_ref.clone();
        let login_render = login_render.clone();

        move |_| {
            // TODO: I will validate handle these errors later;
            let username = username_ref.cast::<HtmlInputElement>().unwrap().value();
            let password = password_ref.cast::<HtmlInputElement>().unwrap().value();
            let login_render = login_render.clone();

            spawn_local(async move {
                /// Data for sending to the server
                let username_password =
                    serde_json::to_string(&UsernameAndPassword { username, password }).unwrap();

                /// post request for validating username and password
                let resp = Request::post("http://127.0.0.1:8000/auth/login")
                    .header("Content-Type", "application/json")
                    .body(username_password)
                    .send()
                    .await
                    .unwrap();

                if resp.status() == 200 {
                    // user is valid
                    console_log!("Logged in successfully");

                    /// Save user's info in localstorage
                    LocalStorage::set("user_info", resp.json::<User>().await.unwrap());

                    /// reload the window for update all the states according to the new user
                    let window = web_sys::window().expect("No Window object found!!");
                    let document = window.document().expect("No Document object found!!");
                    let location = document.location().expect("No Location object found!!");

                    /// reload
                    location.reload().unwrap();
                } else if resp.status() == 401 {
                    // user is not valid
                    console_warn!("Invalid credentials!"); // TODO: Show a alert message
                } else {
                    // Server error;
                    console_error!("Server error: {}", resp.status());
                }
            })
        }
    };

    let on_cancel = {
        move |_| {
            login_render.set(LoginRender(false));
        }
    };

    html! {
        <>

        <Highlight>
            <section class="form">
                <h1 class="form-title">{"Login Account"}</h1>

                <div class="form-wrapper">
                    <label for="username-login">{"Enter your Username"}</label>
                    <input ref={username_ref.clone()} type="text" name="username-login" id="username-login" />
                </div>
                <div class="form-wrapper">
                    <label for="password-login">{"Enter your Password"}</label>
                    <input ref={password_ref.clone()} type="text" name="password-login" id="password-login" />
                </div>
                <div class="buttons-div">
                    <button onclick={on_submit} class="submit-btn">{"Login Account"}</button>
                    <button onclick={on_cancel} class="cancel-btn">{"Cancel"}</button>
                </div>
            </section>
        </Highlight>
        </>
    }
}
