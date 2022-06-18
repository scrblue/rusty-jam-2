use bevy::prelude::Component;
use naia_shared::{Property, Replicate};

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct Countdown {
    pub secs_left: Property<u8>,
}

impl Countdown {
    pub fn new() -> Countdown {
        Countdown::new_complete(10)
    }
}
