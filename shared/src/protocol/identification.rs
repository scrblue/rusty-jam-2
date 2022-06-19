use bevy::prelude::Component;
use naia_shared::{Property, Replicate};

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct Identification {
    pub username: Property<String>,
    pub room_password: Property<String>,
}
