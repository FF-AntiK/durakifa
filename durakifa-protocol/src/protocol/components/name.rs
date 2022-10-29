use bevy_ecs::prelude::Component;
use naia_shared::{Property, Replicate};

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct Name {
    pub name: Property<String>,
}

impl Name {
    pub fn new(name: String) -> Self {
        Name::new_complete(name)
    }
}
