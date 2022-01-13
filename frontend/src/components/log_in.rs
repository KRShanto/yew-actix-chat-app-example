use web_sys::HtmlInputElement;
use weblog::console_log;
use yew::prelude::*;

#[function_component(Login)]
pub fn login() -> Html {
    let username_ref = NodeRef::default();
    let password_ref = NodeRef::default();

    html! {
        <>
            <div class="login">
                <label for="username">{"Enter your username"}</label>
                <input ref={username_ref.clone()} type="text" id="username" />

                <label for="password">{"Enter your password"}</label>
                <input ref={password_ref.clone()} type="text" id="password" />

                <button id="login" onclick={move |_| {
                    let username = username_ref.cast::<HtmlInputElement>().unwrap().value();
                    let password = password_ref.cast::<HtmlInputElement>().unwrap().value();

                    console_log!("username: ", username);
                    console_log!("password: ", password);

                } }>{"Login"}</button>
            </div>
        </>
    }
}
