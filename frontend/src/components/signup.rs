use reqwasm::http::FormData;
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use weblog::console_log;
use yew::prelude::*;

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
                <input ref={nickname_ref.clone()} type="text" id="nickname" />

                <label for="username" >{"Enter your username"}</label>
                <input ref={username_ref.clone()} type="text" id="username" />

                <label for="password" >{"Enter your password"}</label>
                <input ref={password_ref.clone()} type="text" id="password" />

                <label for="img" >{"Enter your img"}</label>
                <input ref={img_ref.clone()} type="file" id="img" accept="image/*"/>

                <button onclick={move |_| {

                    let nickname = nickname_ref.cast::<HtmlInputElement>().unwrap().value();
                    let username = username_ref.cast::<HtmlInputElement>().unwrap().value();
                    let password = password_ref.cast::<HtmlInputElement>().unwrap().value();
                    let img = img_ref.cast::<HtmlInputElement>().unwrap().files().unwrap();
                    let img_url =  Uuid::new_v4().to_string() + "----" +   &img.get(0).unwrap().name();

                    console_log!("Nickname: ", nickname.clone());
                    console_log!("Username: ", username.clone());
                    console_log!("Password: ", password.clone());
                    console_log!("Image url: ", img_url.clone());


                    let form_data = FormData::new().unwrap();
                    form_data.append_with_blob_and_filename("imgage", &img.get(0).unwrap(), &img_url).unwrap();

                    spawn_local(async move {

                        let resp = Request::post("http://127.0.0.1:8000/upload-image/")
                        .body(form_data)
                        .send()
                        .await
                        .unwrap()
                        .ok();

                        console_log!(resp)

                    })


                }}>{"Create account"}</button>

            </div>
        </>
    }
}
