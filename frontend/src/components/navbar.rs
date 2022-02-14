use yew::prelude::*;

use crate::components::chat_app::{CreateAccountRender, LoginRender};

// props of ```NavBar``` component
#[derive(PartialEq, Properties)]
pub struct NavBarProps {
    pub create_account_render: UseStateHandle<CreateAccountRender>,
    pub login_render: UseStateHandle<LoginRender>,
}

// navbar of the application
// This component is called by the the ```App``` component
#[function_component(NavBar)]
pub fn navbar(props: &NavBarProps) -> Html {
    let create_account_render = props.create_account_render.clone();
    let login_render = props.login_render.clone();

    let register_click = {
        move |_| {
            // Display the ```CreateAccountRender``` component
            create_account_render.set(CreateAccountRender(true));
        }
    };

    let login_click = {
        move |_| {
            // Display ```Login``` component
            login_render.set(LoginRender(true));
        }
    };

    html! {
        <>
        <nav>
            <h1 class="logo">{"YewChat"}</h1>
            <ul>
                <li><a href="">{"Home"}</a></li>
                <li><a onclick={login_click}>{"Login"}</a></li>
                <li onclick={register_click}><a>{"Register"}</a></li>
            </ul>
        </nav>
        </>
    }
}
