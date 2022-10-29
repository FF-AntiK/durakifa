use bevy_ecs::prelude::Component;
use naia_shared::{EntityProperty, Replicate};

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct Join {
    pub room: EntityProperty,
}

impl Join {
    pub fn new() -> Self {
        Join::new_complete()
    }
}
