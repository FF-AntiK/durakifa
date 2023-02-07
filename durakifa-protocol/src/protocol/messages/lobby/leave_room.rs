use bevy_ecs::prelude::Component;
use naia_shared::Replicate;

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct LeaveRoom;

impl LeaveRoom {
    pub fn new() -> Self {
        LeaveRoom::new_complete()
    }
}
