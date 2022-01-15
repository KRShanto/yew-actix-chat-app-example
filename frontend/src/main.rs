use yew::prelude::*;

mod components;
use components::{CreateGroup, Login, Signup};

fn main() {
    yew::start_app::<App>();
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <div>
                <Login />
                <Signup />

            </div>
            <CreateGroup />
        </>
    }
}
