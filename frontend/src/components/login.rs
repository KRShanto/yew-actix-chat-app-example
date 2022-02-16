use gloo_storage::{LocalStorage, Storage};
use reqwasm::http::{Request, RequestCredentials, RequestMode};
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use weblog::{console_error, console_log, console_warn};
use yew::prelude::*;

use crate::components::{
    chat_app::{server_url, LoginRender, User},
    Highlight,
};

// username && password for sending in server
#[derive(Debug, Serialize, Deserialize)]
struct UsernameAndPassword {
    username: String,
    password: String,
}

// props of ```Login``` component
#[derive(PartialEq, Properties)]
pub struct LoginProps {
    pub login_render: UseStateHandle<LoginRender>,
}

// Login User component
// This component is called by the ```App``` component
#[function_component(Login)]
pub fn login(props: &LoginProps) -> Html {
    let login_render = props.login_render.clone();

    let username_ref = NodeRef::default();
    let password_ref = NodeRef::default();

    // onclick event of <button class="submit-btn"> element
    let on_submit = {
        let username_ref = username_ref.clone();
        let password_ref = password_ref.clone();

        move |_| {
            // TODO: I will validate handle these errors later;
            let username = username_ref.cast::<HtmlInputElement>().unwrap().value();
            let password = password_ref.cast::<HtmlInputElement>().unwrap().value();

            spawn_local(async move {
                // Data for sending to the server
                let username_password = UsernameAndPassword { username, password };

                // Json data of `username_password`
                let username_password = serde_json::to_string(&username_password).unwrap();

                // post request for validating username and password
                let resp = Request::post(&server_url(Some("auth/login")))
                    .header("Content-Type", "application/json")
                    .body(username_password)
                    // The property "credentials" and "mode" is needed for cookie related works. This not be need if the frontend and backend is the same domain/port. While the frontend is developed in different port, so this is needed
                    .credentials(RequestCredentials::Include)
                    .mode(RequestMode::Cors)
                    .send()
                    .await
                    .unwrap();

                if resp.status() == 200 {
                    // user is valid
                    console_log!("Logged in successfully");

                    // Save user's info in localstorage
                    // TODO: I will save these info in cookies later

                    LocalStorage::set("user_info", resp.json::<User>().await.unwrap()).unwrap();

                    // javascript `Window` object
                    let window = web_sys::window().expect("No Window object found!!");
                    // javascript `Document` object
                    let document = window.document().expect("No Document object found!!");
                    // javascript `Location` object
                    let location = document.location().expect("No Location object found!!");

                    // reload the window for update all the states according to the new user
                    location.reload().unwrap();
                } else if resp.status() == 401 {
                    // user is not valid
                    console_warn!("Invalid credentials!"); // TODO: Show a alert message
                } else {
                    // Server error;
                    // TODO: Do not show this message in production
                    console_error!("Server error: {}", resp.status());
                }
            })
        }
    };

    // onclick event of <button class="cancel-btn">
    let on_cancel = {
        move |_| {
            // Hide this component
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
