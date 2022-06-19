use bevy::prelude::Component;
use naia_shared::{Property, Replicate};

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct ClientKeepAlive;


impl ClientKeepAlive {
    pub fn new() -> ClientKeepAlive {
        ClientKeepAlive
    }
}

