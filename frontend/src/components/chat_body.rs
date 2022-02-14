use yew::prelude::*;

use crate::components::{
    chat_app::{ChatInputRender, ChatOptionRender, JoinRoomRequestsRender, MessageBarRef},
    {ChatBar, RoomBar},
};

#[derive(PartialEq, Properties)]
pub struct ChatBodyProps {
    pub chat_option_render: UseStateHandle<ChatOptionRender>,
    pub chat_input_render: UseStateHandle<ChatInputRender>,
    pub message_bar_ref: MessageBarRef,
    pub join_room_requests_render: UseStateHandle<JoinRoomRequestsRender>,
}

// Main component for rendering rooms, chat messages
// Body of the Chat component.
// This component is called by the ```ChatApp``` component
#[function_component(ChatBody)]
pub fn chat_body(props: &ChatBodyProps) -> Html {
    let chat_option_render = props.chat_option_render.clone();
    let chat_input_render = props.chat_input_render.clone();
    let message_bar_ref = props.message_bar_ref.clone();
    let join_room_requests_render = props.join_room_requests_render.clone();

    html! {
        <section id="chat-body">
            <RoomBar />
            <ChatBar {chat_input_render} {chat_option_render} {message_bar_ref} {join_room_requests_render}/>
        </section>
    }
}
