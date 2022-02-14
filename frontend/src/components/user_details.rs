use yew::prelude::*;

use crate::components::chat_app::{image_link, no_context_error, User};

// Some details about the current/logged-in user
// User's nickname, image, username will shown here
// This component is called by the ```ChatHeader``` component
#[function_component(UserDetails)]
pub fn user_details() -> Html {
    let user_details: User = use_context().expect(&no_context_error("User"));

    html! {
        <>
        <section id="user-details-header">

            <div class="user-image">
                <img src={image_link(&user_details.img_url)}alt="Your image" />
            </div>

            <div class="names">
                <h1 class="nickname">{user_details.nickname}</h1>
                <p class="username">{user_details.username}</p>
            </div>

        </section>
        </>
    }
}
