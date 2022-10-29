use bevy_ecs::prelude::Component;
use naia_shared::{Property, Replicate};

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct Add {
    pub name: Property<String>,
}

impl Add {
    pub fn new(name: String) -> Self {
        Add::new_complete(name)
    }
}
