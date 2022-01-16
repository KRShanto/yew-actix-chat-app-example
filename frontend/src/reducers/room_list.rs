use std::rc::Rc;
use yew::prelude::*;

use crate::Room;

pub enum RoomListAction {
    AddRoom(Room),
    RemoveRoom(Room),
}
#[derive(PartialEq)]
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
        let new_room_list = match action {
            RoomListAction::AddRoom(room) => {
                let mut new_room: Vec<Room> = Vec::new();
                for room in self.rooms.clone() {
                    new_room.push(room);
                }
                new_room.push(room);

                new_room
            }
            RoomListAction::RemoveRoom(room) => {
                vec![room]
                // TODO: Currently I am not removing any room. Later on I will.
            }
        };
        Self {
            rooms: new_room_list,
        }
        .into()
    }
}