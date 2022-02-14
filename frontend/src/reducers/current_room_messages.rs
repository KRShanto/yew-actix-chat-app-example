use std::rc::Rc;
use yew::prelude::*;

use crate::components::chat_app::Message;

pub enum CurrentRoomMessageAction {
    ResetMessages(Vec<Message>), // reset messages
    AddMessage(Message),
    // RemoveMessage(Message), // currently not supported
}

// messages state for currently selected room
#[derive(PartialEq, Debug)]
pub struct CurrentRoomMessageState {
    pub messages: Vec<Message>,
}

impl CurrentRoomMessageState {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
        }
    }
}

impl Reducible for CurrentRoomMessageState {
    type Action = CurrentRoomMessageAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            // Add new message to the current room message. When a user send any message to the current room, call this action
            CurrentRoomMessageAction::AddMessage(msg) => {
                let mut new_msg = self.messages.clone();
                new_msg.push(msg);

                Self { messages: new_msg }.into()
            }
            // Fetch new messages then call this action. Call this action when the current room state changes
            CurrentRoomMessageAction::ResetMessages(messages) => Self { messages }.into(),
        }
    }
}
