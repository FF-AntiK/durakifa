use bevy_ecs::prelude::Component;
use naia_shared::{EntityProperty, Replicate};

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct Own {
    pub player: EntityProperty,
}

impl Own {
    pub fn new() -> Self {
        Own::new_complete()
    }
}
