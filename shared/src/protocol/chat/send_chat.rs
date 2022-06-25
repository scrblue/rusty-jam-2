use bevy::prelude::Component;
use naia_shared::{Property, Replicate};

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct SendChat {
    pub message: Property<String>,
}

impl SendChat {
    pub fn new(message: String) -> SendChat {
        SendChat::new_complete(message)
    }
}

