use bevy_ecs::prelude::Component;
use naia_shared::{EntityProperty, Replicate};

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct JoinRoom {
    pub room: EntityProperty,
}

impl JoinRoom {
    pub fn new() -> Self {
        JoinRoom::new_complete()
    }
}
