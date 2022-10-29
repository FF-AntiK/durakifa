use bevy_ecs::prelude::Component;
use naia_shared::Replicate;

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct Player;

impl Player {
    pub fn new() -> Self {
        Player::new_complete()
    }
}
