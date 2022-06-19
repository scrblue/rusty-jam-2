use bevy::prelude::Component;
use naia_shared::{derive_serde, serde, Property, Replicate};

use crate::behavior::AxialCoordinates;

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct PlayerInput {
    pub turn_inputs: Property<Vec<PlayerInputVariant>>,
}

#[derive_serde]
pub enum PlayerInputVariant {
    ClaimTile(ClaimTile),
}

#[derive_serde]
pub struct ClaimTile {
    qr: AxialCoordinates,
    layer: u16,
}
