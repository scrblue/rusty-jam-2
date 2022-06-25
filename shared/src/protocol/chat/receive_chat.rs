use bevy::prelude::Component;
use naia_shared::{Property, Replicate};

use crate::components::players::PlayerId;

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct ReceiveChat {
    pub sending_player: Property<Option<PlayerId>>,
    pub message: Property<String>,
}

impl ReceiveChat {
    pub fn new(sending_player: Option<PlayerId>, message: String) -> ReceiveChat {
        ReceiveChat::new_complete(sending_player, message)
    }
}
