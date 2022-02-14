use crate::components::chat_app::{no_context_error, User};
use yew::prelude::*;

// props of ```MessageComponent``` component
#[derive(PartialEq, Properties)]
pub struct MessageComponentProps {
    pub user_id: i32,
    pub message: String,  // message
    pub nickname: String, // nickname of current user
    pub img_url: String, // img url of current user. give the full path with http://127.0.0.1:8000/get-user-image/{}
}

// The component for each messages;
// This component is called by the ```MessageBar``` component
#[function_component(MessageComponent)]
pub fn message(props: &MessageComponentProps) -> Html {
    let user_details: User = use_context().expect(&no_context_error("User"));

    // class name of <section> element
    // if this user is the logged in user, then class will be "owner", otherwise class will be "other"
    let class_name = if props.user_id == user_details.id {
        "owner"
    } else {
        "other"
    };

    // user's nickname for displaying on the browser.
    // if this user is the logged in user, then nickname will be "You", otherwise nickname will be prop's user's nickname
    let nickname = if props.nickname == user_details.nickname {
        String::from("You")
    } else {
        props.nickname.clone()
    };

    html! {
        <>
        <section class={format!("user-message {}", class_name)}>

            <img
                class="user-image"
                src={props.img_url.clone()}
                alt="User image"
            />
            <div class="message-and-nickname">
                <h1 class="nickname">{nickname}</h1>
                <p
                    class="message"
                >
                    {props.message.clone()
                }</p>
            </div>



        </section>



        </>
    }
}
