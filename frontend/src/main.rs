use yew::prelude::*;

pub mod components;
pub mod reducers;
pub mod websocket;

use components::{
    chat_app::{CreateAccountRender, LoginRender},
    ChatApp, CreateAccount, Login, NavBar,
};

fn main() {
    yew::start_app::<App>();
}

// The root component of the application.
#[function_component(App)]
fn app() -> Html {
    let create_account_render = use_state(|| CreateAccountRender(false));
    let login_render = use_state(|| LoginRender(false));

    html! {

        <>

        <header>
            <NavBar create_account_render={create_account_render.clone()} login_render={login_render.clone()}/>
        </header>

        if (*create_account_render).0 {
            <CreateAccount {create_account_render}/>
        }
        if (*login_render).0 {
            <Login {login_render} />
        }

        <ChatApp />

        </>
    }
}
