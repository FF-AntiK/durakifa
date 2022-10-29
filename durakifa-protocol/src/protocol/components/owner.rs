use bevy_ecs::prelude::Component;
use naia_shared::Replicate;

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct Owner;

impl Owner {
    pub fn new() -> Self {
        Owner::new_complete()
    }
}
