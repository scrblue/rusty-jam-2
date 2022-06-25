use bevy::prelude::Component;
use naia_shared::{Property, Replicate};

use crate::components::players::PlayerId;

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct ClientConnected {
    pub username: Property<String>,
    pub id: Property<PlayerId>,
}

impl ClientConnected {
    pub fn new(username: String, id: PlayerId) -> ClientConnected {
        ClientConnected::new_complete(username, id)
    }
}
