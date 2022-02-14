use yew::prelude::*;

use crate::{
    components::{chat_app::no_context_error, RoomComponent},
    reducers::RoomListState,
};

// container for all RoomComponent
// This component is called by the ```ChatBody``` component
#[function_component(RoomBar)]
pub fn room_bar() -> Html {
    let room_list: UseReducerHandle<RoomListState> =
        use_context().expect(&no_context_error("UseReducerHandle<RoomListState>"));

    html! {
        <>
        <section id="room-bar">
            {
                room_list.rooms.iter().map(|room| {
                    html! {
                        <RoomComponent
                            room={room.clone()}
                        />
                    }
                }).collect::<Html>()
            }
        </section>
        </>
    }
}
