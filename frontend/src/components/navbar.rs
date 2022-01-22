use yew::prelude::*;

#[function_component(NavBar)]
pub fn navbar() -> Html {
    html! {
        <>
        <nav>
            <h1 class="logo">{"Yew Chat"}</h1>
            <ul>
                <li><a href="">{"Home"}</a></li>
                <li><a href="">{"Login"}</a></li>
                <li><a href="">{"Register"}</a></li>
            </ul>
        </nav>
        <hr />
        </>
    }
}
