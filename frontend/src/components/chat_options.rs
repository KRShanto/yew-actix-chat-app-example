use yew::prelude::*;

use crate::{
    components::chat_app::{no_context_error, JoinRoomRequestsRender, User},
    reducers::CurrentRoomState,
};

// props of the ```ChatOptions``` component
#[derive(PartialEq, Properties)]
pub struct ChatOptionsProps {
    pub join_room_requests_render: UseStateHandle<JoinRoomRequestsRender>,
}

// Option component for ```ChatBar``` component
// This component will render some options for the individual chat rooms.
// This component is called by the ```ChatBar``` component
#[function_component(ChatOptions)]
pub fn chat_options(props: &ChatOptionsProps) -> Html {
    // Currently this component is only renders the numbers of join requests of the room and render the ```JoinRoomRequests``` component.

    let current_room_details: UseReducerHandle<CurrentRoomState> =
        use_context().expect(&no_context_error("UseReducerHandle<CurrentRoomState>"));

    // numbers of join requests
    let user_len = use_state(|| 0);

    let join_room_requests_render = props.join_room_requests_render.clone();

    // onclick Event of the <button> element
    let onclick = {
        move |_| {
            // Display the ```JoinRoomRequests``` component
            join_room_requests_render.set(JoinRoomRequestsRender(true));
        }
    };

    // After the first render of this component set the value of `user_len`
    {
        let user_len = user_len.clone();
        use_effect_with_deps(
            move |current_room_details| {
                let current_room_join_requests: Option<Vec<User>> =
                    current_room_details.current_room_join_requests.clone();

                if let Some(users) = current_room_join_requests {
                    user_len.set(users.len());
                };

                || ()
            },
            current_room_details,
        )
    }

    html! {
        <>
        <section id="chat-options">

            <div id="room-requests">
                <span class="number-of-requests">{*user_len}</span>
                <button {onclick}>{"Room Requests"}</button>
            </div>

        </section>
        </>
    }
}
