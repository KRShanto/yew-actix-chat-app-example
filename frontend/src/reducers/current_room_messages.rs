use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;
use weblog::console_log;
use yew::prelude::*;

use crate::components::chat_app::Message;

pub enum CurrentRoomMessageAction {
    ResetMessages(Vec<Message>), // / reset messages
    AddMessage(Message),
    // RemoveMessage(Message),
}

// You should change this when user is clicking on the chat bar's room;
#[derive(PartialEq, Debug)]
pub struct CurrentRoomMessageState {
    pub messages: Vec<Message>,
}

impl CurrentRoomMessageState {
    pub fn new() -> Self {
        Self {
            // room_id: None,
            messages: Vec::new(),
        }
    }
}

impl Reducible for CurrentRoomMessageState {
    type Action = CurrentRoomMessageAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            CurrentRoomMessageAction::AddMessage(msg) => {
                let mut new_msg: Vec<Message> = Vec::new();
                for i in self.messages.clone() {
                    new_msg.push(i);
                }
                new_msg.push(msg);
                Self { messages: new_msg }.into()
            }
            CurrentRoomMessageAction::ResetMessages(messages) => Self { messages }.into(),
        }
    }
}
