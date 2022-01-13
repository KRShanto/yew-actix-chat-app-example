use reqwasm::http::Request;
use uuid::Uuid;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::File;
use web_sys::HtmlInputElement;
use weblog::console_log;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    type FormData;

    #[wasm_bindgen(constructor)]
    fn new() -> FormData;

    #[wasm_bindgen(method)]
    fn append(this: &FormData, name: &str, value: File, filename: String);

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
                <input ref={nickname_ref.clone()} type="text" id="nickname" />

                <label for="username" >{"Enter your username"}</label>
                <input ref={username_ref.clone()} type="text" id="username" />

                <label for="password" >{"Enter your password"}</label>
                <input ref={password_ref.clone()} type="text" id="password" />

                <label for="img" >{"Enter your img"}</label>
                <input ref={img_ref.clone()} type="file" id="img" />

                <button onclick={move |_| {

                    let nickname = nickname_ref.cast::<HtmlInputElement>().unwrap().value();
                    let username = username_ref.cast::<HtmlInputElement>().unwrap().value();
                    let password = password_ref.cast::<HtmlInputElement>().unwrap().value();
                    let img = img_ref.cast::<HtmlInputElement>().unwrap().files().unwrap();
                    let img_url =  Uuid::new_v4().to_string() + "----" +   &img.get(0).unwrap().name();

                    console_log!("Nickname: ", nickname);
                    console_log!("Username: ", username);
                    console_log!("Password: ", password);
                    console_log!("Image url: ", img_url.clone());

                    let form_data = FormData::new();
                    form_data.append("imgForm", img.get(0).unwrap(), img_url);


                }}>{"Create account"}</button>

            </div>
        </>
    }
}
