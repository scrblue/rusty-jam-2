use bevy::prelude::Component;
use naia_shared::{Property, Replicate};

use super::WhoseTurn;

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct TurnChangeNotification {
    pub whose_turn: Property<WhoseTurn>,
}
