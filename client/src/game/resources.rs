use std::collections::HashMap;

use bevy::prelude::Entity;
use rgj_shared::{
    behavior::AxialCoordinates,
    protocol::{notifications::WhoseTurn, player_input::PlayerInputVariant, MapSync, UnitSync},
};

pub struct TileSelectedEvent(pub AxialCoordinates);

#[derive(Default)]
pub struct TileSelectedState {
	pub moving_unit: Option<Entity>,

	pub tile: Option<MapSync>,
	pub unit: Option<UnitSync>,
}

pub struct TurnTracker {
    pub whose_turn: WhoseTurn,
    pub recorded_commands: Vec<PlayerInputVariant>,
}

impl TurnTracker {
    pub fn new(wt: &WhoseTurn) -> TurnTracker {
        TurnTracker {
            whose_turn: wt.clone(),
            recorded_commands: Vec::new(),
        }
    }
}

pub struct Map {
    pub coords_to_tile: HashMap<(u16, u16, u16), Entity>,
    pub coords_to_unit: HashMap<(u16, u16, u16), Entity>,
}
