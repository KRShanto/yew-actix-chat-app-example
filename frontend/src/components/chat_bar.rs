use crate::components::{
    chat_app::{ChatInputRender, ChatOptionRender, JoinRoomRequestsRender, MessageBarRef},
    {ChatInput, ChatOptions, MessageBar},
};
use yew::prelude::*;

// props of the ```ChatBar``` component
#[derive(PartialEq, Properties)]
pub struct ChatBarProps {
    pub chat_option_render: UseStateHandle<ChatOptionRender>,
    pub chat_input_render: UseStateHandle<ChatInputRender>,
    pub message_bar_ref: MessageBarRef,
    pub join_room_requests_render: UseStateHandle<JoinRoomRequestsRender>,
}

// Main component for displaying chat rooms, chat messages
// This component is called by ```ChatBody``` component
#[function_component(ChatBar)]
pub fn chat_bar(props: &ChatBarProps) -> Html {
    let chat_option_render = &props.chat_option_render;
    let chat_input_render = &props.chat_input_render;
    let message_bar_ref = props.message_bar_ref.clone();
    let join_room_requests_render = props.join_room_requests_render.clone();

    html! {
    <>
    <section id="chat-bar">

        // if the value of ChatOptionRendered is true
        if (*chat_option_render).0 {
            <ChatOptions {join_room_requests_render} />
        }

        <MessageBar {message_bar_ref}/>

        // if the value of ChatInputRendered is true
        if (*chat_input_render).0 {
            <ChatInput />
        }

    </ section>
    </>
    }
}
