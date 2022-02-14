use std::rc::Rc;
use yew::prelude::*;

use crate::components::chat_app::Room;

pub enum RoomListAction {
    AddRoom(Room),
    RemoveRoom(Room),
}

// List of all rooms that the user has joined
#[derive(PartialEq, Debug)]
pub struct RoomListState {
    pub rooms: Vec<Room>,
}

impl RoomListState {
    pub fn new() -> Self {
        Self { rooms: Vec::new() }
    }
}

impl Reducible for RoomListState {
    type Action = RoomListAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            // Add a new room
            RoomListAction::AddRoom(room) => {
                let mut new_rooms: Vec<Room> = self.rooms.clone();
                new_rooms.push(room);

                Self { rooms: new_rooms }.into()
            }
            // Delete a room
            RoomListAction::RemoveRoom(room_to_remove) => {
                // Return those rooms whose id != room_to_remove.id
                let new_rooms: Vec<Room> = self
                    .rooms
                    .clone()
                    .into_iter()
                    .filter(|room| room.id != room_to_remove.id)
                    .collect();

                // TODO: Currently I have not give any feature to remove any room. Later on I will. This action should work but not tested yet!
                Self { rooms: new_rooms }.into()
            }
        }
    }
}
