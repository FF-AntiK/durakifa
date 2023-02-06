use bevy_ecs::prelude::Component;
use naia_shared::{EntityProperty, Replicate};

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct Player {
    pub user: EntityProperty,
}

impl Player {
    pub fn new() -> Self {
        Player::new_complete()
    }
}
