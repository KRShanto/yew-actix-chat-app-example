use std::rc::Rc;
use yew::prelude::*;

use crate::Room;

pub enum CurrentRoomAction {
    SelectRoom(Room),
}
// You should change this when user is clicking on the chat bar's room;
// Use a use_effect hook when this state changes
#[derive(PartialEq, Debug)]
pub struct CurrentRoomState {
    current_room: Option<Room>,
}

impl CurrentRoomState {
    pub fn new() -> Self {
        Self { current_room: None }
    }
}
impl Reducible for CurrentRoomState {
    type Action = CurrentRoomAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let new_room = match action {
            CurrentRoomAction::SelectRoom(room) => room,
        };

        Self {
            current_room: Some(new_room),
        }
        .into()
    }
}
