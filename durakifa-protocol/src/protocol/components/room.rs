use bevy_ecs::prelude::Component;
use naia_shared::Replicate;

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct Room;

impl Room {
    pub fn new() -> Self {
        Room::new_complete()
    }
}
