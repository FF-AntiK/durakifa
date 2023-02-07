use bevy_ecs::prelude::Component;
use naia_shared::{Property, Replicate};

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct Authorize {
    pub key: Property<String>,
}

impl Authorize {
    pub fn new(key: String) -> Self {
        Authorize::new_complete(key)
    }
}
