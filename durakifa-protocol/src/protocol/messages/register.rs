use bevy_ecs::prelude::Component;
use naia_shared::{Property, Replicate};

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct Register {
    pub name: Property<String>,
}

impl Register {
    pub fn new(name: String) -> Self {
        Register::new_complete(name)
    }
}
