use std::collections::HashMap;
use std::rc::Rc;

use crate::components::chat_app::{Room, User};

// Action of the `CurrentRoomState` state
pub enum CurrentRoomAction {
    // make this room as a current room. This action will only add the `Room` in `current_room` property and keep other fields None
    SelectRoom(Room),

    // reset users list
    PutUsers(Vec<User>),

    // reset the join requests
    PutJoinRequests(Vec<User>),

    // add new join requests inside existing join requests list
    AppendJoinRequest(User),

    // remove a join request from the join requests list
    RemoveJoinRequest(i32),
}

// Info state for the currently selected room.
#[derive(PartialEq, Debug)]
pub struct CurrentRoomState {
    // `Room` of the current room
    pub current_room: Option<Room>,

    // list of users that are currently in the room
    pub current_room_users: Option<HashMap<i32, User>>,

    // list of users that are requested to join the current room.
    pub current_room_join_requests: Option<Vec<User>>,
}

impl CurrentRoomState {
    // Initially this room won't become current room. Use the `CurrentRoomAction::SelectRoom(room)` action for make this room current room
    pub fn new() -> Self {
        Self {
            current_room: None,
            current_room_users: None,
            current_room_join_requests: None,
        }
    }
}

impl yew::prelude::Reducible for CurrentRoomState {
    type Action = CurrentRoomAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            // Change/reset room. This action should be called when a user clicks on the `RoomComponent` component
            CurrentRoomAction::SelectRoom(room) => Self {
                current_room_users: None,
                current_room: Some(room),
                current_room_join_requests: None,
            }
            .into(),
            // Put all users associated with the current room. This action should be called when a user clicks on the `RoomComponent` component or when this state changes
            CurrentRoomAction::PutUsers(users) => {
                // make new HashMap for replacing `self.current_room_users` field
                let mut new_users_list: HashMap<i32, User> = HashMap::new();
                for user in users {
                    new_users_list.insert(user.id, user);
                }
                // return Self with the new HashMap `new_users_list`
                Self {
                    current_room: self.current_room.clone(),
                    current_room_users: Some(new_users_list),
                    current_room_join_requests: self.current_room_join_requests.clone(),
                }
                .into()
            }
            // Put all join requests associated with the current room. This action should be called when a user clicks on the `RoomComponent` component or when this state changes
            CurrentRoomAction::PutJoinRequests(users) => Self {
                current_room_users: self.current_room_users.clone(),
                current_room: self.current_room.clone(),
                current_room_join_requests: Some(users),
            }
            .into(),
            // Add new Join requests to the current room.
            CurrentRoomAction::AppendJoinRequest(user) => {
                if let Some(current_room_join_requests) = self.current_room_join_requests.clone() {
                    // Adding new users into ```current_room_join_requests```
                    let mut new_users = current_room_join_requests;
                    new_users.push(user);

                    Self {
                        current_room: self.current_room.clone(),
                        current_room_join_requests: Some(new_users),
                        current_room_users: self.current_room_users.clone(),
                    }
                    .into()
                } else {
                    // It is developer error/mistake. If the user clicks on the ```RoomComponent``` then the field ```self.current_room_join_requests``` will be Some, if not then the developer has forget to implement this :(

                    let err_msg = "Developer error: There is not join requests found ```current_room_join_requests```! The value is None :( while it should be Some(<Vec<User>>). \nHint: When the state ```CurrentRoomState``` changes, then fetch join requests of the current room. ";

                    panic!("{err_msg}");
                }
            }
            // Remove a join request from the current room join request list
            CurrentRoomAction::RemoveJoinRequest(user_id) => {
                if let Some(current_room_join_requests) = self.current_room_join_requests.clone() {
                    let new_join_request_list: Vec<User> = current_room_join_requests
                        .clone()
                        .into_iter()
                        .filter(|u| u.id != user_id)
                        .collect();

                    Self {
                        current_room: self.current_room.clone(),
                        current_room_join_requests: Some(new_join_request_list),
                        current_room_users: self.current_room_users.clone(),
                    }
                    .into()
                } else {
                    // Usually the programme won't come this block because if ```self.current_room_join_requests``` is empty then the button for removing user from list won't display and if that button won't display then the user cannot click that button and this action wont accure

                    // It is developer error/mistake. If the user clicks on the ```RoomComponent``` then the field ```self.current_room_join_requests``` will be Some, if not then the developer has forget to implement this :(

                    let err_msg = "Developer error: There is not join requests found ```current_room_join_requests```! The value is None :( while it should be Some(<Vec<User>>). \nHint: When the state ```CurrentRoomState``` changes, then fetch join requests of the current room. ";

                    panic!("{err_msg}");
                }
            }
        }
    }
}
