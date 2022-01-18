use std::collections::HashMap;
use std::rc::Rc;
use yew::prelude::*;

use crate::{Room, User};

pub enum CurrentRoomAction {
    SelectRoom(Room),
    PutUsers(Vec<User>),
}
// You should change this when user is clicking on the chat bar's room;
// Use a use_effect hook when this state changes
#[derive(PartialEq, Debug)]
pub struct CurrentRoomState {
    pub current_room: Option<Room>,
    pub current_room_users: Option<HashMap<i32, User>>,
}

impl CurrentRoomState {
    pub fn new() -> Self {
        Self {
            current_room: None,
            current_room_users: None,
        }
    }
}
impl Reducible for CurrentRoomState {
    type Action = CurrentRoomAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            CurrentRoomAction::SelectRoom(room) => Self {
                current_room_users: None,
                current_room: Some(room),
            }
            .into(),
            CurrentRoomAction::PutUsers(users) => {
                let mut new_users_list: HashMap<i32, User> = HashMap::new();
                for user in users {
                    new_users_list.insert(user.id, user);
                }
                Self {
                    current_room: self.current_room.clone(),
                    current_room_users: Some(new_users_list),
                }
                .into()
            }
        }
    }
}
