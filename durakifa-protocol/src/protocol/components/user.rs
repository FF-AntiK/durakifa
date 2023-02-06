use bevy_ecs::prelude::Component;
use naia_shared::Replicate;

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct User;

impl User {
    pub fn new() -> Self {
        User::new_complete()
    }
}
