use yew::prelude::*;

use crate::components::chat_app::{CreateNewRoomRender, JoinRoomRender};

// props of ```MoreOptions``` component
#[derive(PartialEq, Properties)]
pub struct MoreOptionsProps {
    pub join_room_render: UseStateHandle<JoinRoomRender>,
    pub create_new_room_render: UseStateHandle<CreateNewRoomRender>,
}

// Options for the chat application. "create new group", "join group" etc. will be shown here.
// This component will be called by the ```ChatHeader``` component
#[function_component(MoreOptions)]
pub fn more_options(props: &MoreOptionsProps) -> Html {
    let join_room_render = props.join_room_render.clone();

    let create_new_room_render = props.create_new_room_render.clone();

    html! {
        <>
        <section id="more-options">

            <button class="create-new-room" onclick={ move |_| {
                // display the ```CreateNewRoom``` component
                create_new_room_render.set(CreateNewRoomRender(true));
            }}>{"Create new room"}</button>

            <button class="join-room" onclick={ move |_| {
                // display the ```JoinRoom``` component
                join_room_render.set(JoinRoomRender(true));
            }}>{"Join room"}</button>

        </section>
        </>
    }
}
