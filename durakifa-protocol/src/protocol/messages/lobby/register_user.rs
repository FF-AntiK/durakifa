use bevy_ecs::prelude::Component;
use naia_shared::{Property, Replicate};

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct RegisterUser {
    pub name: Property<String>,
}

impl RegisterUser {
    pub fn new(name: String) -> Self {
        RegisterUser::new_complete(name)
    }
}
