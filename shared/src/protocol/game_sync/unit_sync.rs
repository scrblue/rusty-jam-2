use bevy::prelude::Component;
use naia_shared::{Property, Replicate};

use crate::{behavior::AxialCoordinates, components::{genome::Hybrid, players::PlayerId}};

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
/// The synchronization of a tile on the map
pub struct UnitSync {
    pub position: Property<AxialCoordinates>,
    pub layer: Property<i32>,

	pub player_id: Property<PlayerId>,

    pub hybrid_type: Property<Hybrid>,

    pub current_health: Property<u16>,
    pub stamina_remaining: Property<u16>,
}
