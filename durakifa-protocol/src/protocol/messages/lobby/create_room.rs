use bevy_ecs::prelude::Component;
use naia_shared::Replicate;

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct CreateRoom;

impl CreateRoom {
    pub fn new() -> Self {
        CreateRoom::new_complete()
    }
}
