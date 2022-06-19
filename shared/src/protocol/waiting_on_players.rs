use bevy::prelude::Component;
use naia_shared::{Property, Replicate};

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct WaitingOnPlayers {
    pub num_waiting_for: Property<u8>,
}
