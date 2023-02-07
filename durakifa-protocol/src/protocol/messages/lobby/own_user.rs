use bevy_ecs::prelude::Component;
use naia_shared::{EntityProperty, Replicate};

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct OwnUser {
    pub user: EntityProperty,
}

impl OwnUser {
    pub fn new() -> Self {
        OwnUser::new_complete()
    }
}
