use bevy::prelude::Component;
use naia_shared::{derive_serde, serde, EntityHandle, Property, Replicate};

use crate::behavior::AxialCoordinates;

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct PlayerInput {
    pub partial_turn_inputs: Property<Vec<PlayerInputVariant>>,
    pub end_turn: Property<bool>,
}

#[derive_serde]
pub enum PlayerInputVariant {
    MoveEntity(EntityHandle, AxialCoordinates),
}
