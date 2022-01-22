use yew::prelude::*;

use crate::components2::{
    chat_app::{CreateNewRoomRender, JoinRoomRender},
    {MoreOptions, UserDetails},
};

#[derive(PartialEq, Properties)]
pub struct ChatHeaderProps {
    pub join_room_render: UseStateHandle<JoinRoomRender>,
    pub create_new_room_render: UseStateHandle<CreateNewRoomRender>,
}

#[function_component(ChatHeader)]
pub fn chat_header(props: &ChatHeaderProps) -> Html {
    /// Header of the chat application
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
