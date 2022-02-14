use yew::prelude::*;

use crate::{
    components::{ MessageComponent, chat_app::{image_link, MessageBarRef}},
    
    reducers::{CurrentRoomState, CurrentRoomMessageState},
    
};

// props of ```MessageBar``` component
#[derive(PartialEq, Properties)]
pub struct MessageBarProps{
    pub message_bar_ref:  MessageBarRef,
}

// Show all messages from the current room;
// This component is called by the ```ChatBar``` component
#[function_component(MessageBar)]
pub fn message_bar(props: &MessageBarProps) -> Html {
    let current_room_messages: UseReducerHandle<CurrentRoomMessageState> = use_context().expect( 
        "No context provided!!!. A context should be provided with `<UseReducerHandle<CurrentRoomMessageState>>"
    );
    let current_room_details: UseReducerHandle<CurrentRoomState> = use_context().expect("No context provided!!!. A user should be provided with `UseReducerHandle<CurrentRoomState>`");

    let message_bar_ref=  props.message_bar_ref.clone();

    html! {
        <>
        <section id="message-bar" ref={message_bar_ref.0}>
            {
                current_room_messages.messages.iter().map(|message| {                    
                    html! {
                        if let Some(user_map) = &current_room_details.current_room_users {
                            if let Some(user) = user_map.get(&message.user_id) {
                                // render messages of current users       
                                <MessageComponent 
                                    user_id={message.user_id}
                                    message={message.clone().msg} 
                                    nickname={user.nickname.clone()}
                                    img_url={image_link(&user.img_url.clone())}
                                />
                            }

                        }
                    }
                }).collect::<Html>()
            }        
        </section>
        </>
    }
}