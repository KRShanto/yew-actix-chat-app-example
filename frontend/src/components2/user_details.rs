use yew::prelude::*;

use crate::components2::chat_app::{image_link, no_context_error, User};

/// Complete**********
#[function_component(UserDetails)]
pub fn user_details() -> Html {
    /// User's nickname, image, username will shown here
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
