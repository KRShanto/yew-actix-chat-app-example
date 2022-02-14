use yew::prelude::*;

use crate::components::{
    chat_app::{CreateNewRoomRender, JoinRoomRender},
    {MoreOptions, UserDetails},
};

// props of the ```ChatHeader``` component
#[derive(PartialEq, Properties)]
pub struct ChatHeaderProps {
    pub join_room_render: UseStateHandle<JoinRoomRender>,
    pub create_new_room_render: UseStateHandle<CreateNewRoomRender>,
}

// Main component for displaying user's details and some options related to chatapp
// Header of the chat application
// This component is called by the ```ChatApp``` component
#[function_component(ChatHeader)]
pub fn chat_header(props: &ChatHeaderProps) -> Html {
    let join_room_render = props.join_room_render.clone();
    let create_new_room_render = props.create_new_room_render.clone();

    html! {
        <>
            <header id="chat-header">
                <UserDetails />
                <MoreOptions {join_room_render} {create_new_room_render}/>
            </header>
        </>
    }
}
