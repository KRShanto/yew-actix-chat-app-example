use yew::prelude::*;

mod components;
use components::{Login, Signup};

fn main() {
    yew::start_app::<App>();
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <Login />
            <Signup />
        </>
    }
}
