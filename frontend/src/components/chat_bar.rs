use crate::components::{
    chat_app::{
        no_context_error, ChatInputRender, ChatOptionRender, JoinRoomRequestsRender, MessageBarRef,
    },
    ChatInput, ChatOptions, MessageBar,
};
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct ChatBarProps {
    pub chat_option_render: UseStateHandle<ChatOptionRender>,
    pub chat_input_render: UseStateHandle<ChatInputRender>,
    pub message_bar_ref: MessageBarRef,
    pub join_room_requests_render: UseStateHandle<JoinRoomRequestsRender>,
}

#[function_component(ChatBar)]
pub fn chat_bar(props: &ChatBarProps) -> Html {
    let chat_option_render = props.chat_option_render.clone();
    let chat_input_render = props.chat_input_render.clone();
    let message_bar_ref = props.message_bar_ref.clone();
    let join_room_requests_render = props.join_room_requests_render.clone();

    html! {
    <>
    <section id="chat-bar">

        if (*chat_option_render).0 {
            <ChatOptions {join_room_requests_render} />
        }
        <MessageBar {message_bar_ref}/>

        if (*chat_input_render).0 {
            <ChatInput />
        }

    </ section>
    </>
    }
}
