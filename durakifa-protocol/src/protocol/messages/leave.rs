use bevy_ecs::prelude::Component;
use naia_shared::Replicate;

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct Leave;

impl Leave {
    pub fn new() -> Self {
        Leave::new_complete()
    }
}
