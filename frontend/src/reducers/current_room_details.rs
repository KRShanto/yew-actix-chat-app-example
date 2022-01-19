use std::collections::HashMap;
use std::rc::Rc;
use yew::prelude::*;

use crate::{Room, User};

pub enum CurrentRoomAction {
    SelectRoom(Room),
    PutUsers(Vec<User>),
    PutJoinRequests(Vec<User>),
    AppendJoinRequest(User),
}
// You should change this when user is clicking on the chat bar's room;
// Use a use_effect hook when this state changes
#[derive(PartialEq, Debug)]
pub struct CurrentRoomState {
    pub current_room: Option<Room>,
    pub current_room_users: Option<HashMap<i32, User>>,
    pub current_room_join_requests: Option<Vec<User>>,
}

impl CurrentRoomState {
    pub fn new() -> Self {
        Self {
            current_room: None,
            current_room_users: None,
            current_room_join_requests: None,
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
                current_room_join_requests: None,
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
                    current_room_join_requests: self.current_room_join_requests.clone(),
                }
                .into()
            }
            CurrentRoomAction::PutJoinRequests(users) => Self {
                current_room_users: self.current_room_users.clone(),
                current_room: self.current_room.clone(),
                current_room_join_requests: Some(users),
            }
            .into(),
            CurrentRoomAction::AppendJoinRequest(user) => {
                if let Some(current_room_join_requests) = self.current_room_join_requests.clone() {
                    let mut new_users: Vec<User> = Vec::new();
                    for user in current_room_join_requests.clone() {
                        new_users.push(user);
                    }
                    new_users.push(user);

                    Self {
                        current_room: self.current_room.clone(),
                        current_room_join_requests: Some(new_users),
                        current_room_users: self.current_room_users.clone(),
                    }
                    .into()
                } else {
                    Self {
                        current_room: self.current_room.clone(),
                        current_room_join_requests: self.current_room_join_requests.clone(),
                        current_room_users: self.current_room_users.clone(),
                    }
                    .into()
                }
            }
        }
    }
}
