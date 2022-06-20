use bevy::prelude::Component;
use naia_shared::{derive_serde, serde, EntityProperty, Property, Replicate};

use crate::behavior::AxialCoordinates;

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct PlayerInput {
    pub relevant_entity: EntityProperty,
    pub partial_turn: Property<PlayerInputVariant>,
}

#[derive_serde]
pub enum PlayerInputVariant {
    MoveEntity(AxialCoordinates),
    EndTurn,
}
