use std::rc::Rc;
use yew::prelude::*;

use crate::Message;

pub enum CurrentRoomMessageAction {
    ChangeRoom(i32), // pass the new room's id;
    AddMessage(Message),
}

// You should change this when user is clicking on the chat bar's room;
#[derive(PartialEq, Debug)]
pub struct CurrentRoomMessageState {
    pub room_id: Option<i32>,
    pub messages: Vec<Message>,
}

impl CurrentRoomMessageState {
    pub fn new() -> Self {
        Self {
            room_id: None,
            messages: Vec::new(),
        }
    }
}

impl Reducible for CurrentRoomMessageState {
    type Action = CurrentRoomMessageAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            CurrentRoomMessageAction::ChangeRoom(room_id) => Self {
                room_id: Some(room_id),
                messages: self.messages.clone(),
            }
            .into(),
            CurrentRoomMessageAction::AddMessage(msg) => {
                let mut new_msg: Vec<Message> = Vec::new();
                for i in self.messages.clone() {
                    new_msg.push(i);
                }
                new_msg.push(msg);
                Self {
                    room_id: self.room_id,
                    messages: new_msg,
                }
                .into()
            }
        }
    }
}
