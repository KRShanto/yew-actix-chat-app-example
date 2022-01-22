use yew::prelude::*;

use crate::{
    components2::chat_app::{no_context_error, JoinRoomRequestsRender, User},
    reducers::CurrentRoomState,
};

#[derive(PartialEq, Properties)]
pub struct ChatOptionsProps {
    pub join_room_requests_render: UseStateHandle<JoinRoomRequestsRender>,
}

#[function_component(ChatOptions)]
pub fn chat_options(props: &ChatOptionsProps) -> Html {
    let current_room_details: UseReducerHandle<CurrentRoomState> =
        use_context().expect(&no_context_error("UseReducerHandle<CurrentRoomState>"));

    let user_len = use_state(|| 0);

    // let join_room_requests_render: UseStateHandle<JoinRoomRequestsRender> =
    //     use_context().expect(&no_context_error("UseStateHandle<JoinRoomRequestsRender>"));

    let join_room_requests_render = props.join_room_requests_render.clone();

    let onclick = {
        move |_| {
            join_room_requests_render.set(JoinRoomRequestsRender(true));
        }
    };

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
