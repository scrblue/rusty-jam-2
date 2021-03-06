use std::collections::HashMap;

use bevy::prelude::Entity;
use rgj_shared::{
    behavior::AxialCoordinates,
    components::genome::AnimalType,
    protocol::{notifications::WhoseTurn, player_input::PlayerInputVariant, MapSync, UnitSync},
};

pub struct TileSelectedEvent(pub AxialCoordinates);

#[derive(Default)]
pub struct TileSelectedState {
    pub error: String,
    pub moving_unit: Option<Entity>,

    pub build_screen: bool,
    pub head: Option<AnimalType>,
    pub body: Option<AnimalType>,
    pub limbs: Option<AnimalType>,

    pub tile: Option<AxialCoordinates>,
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
    pub coords_to_tile: HashMap<(i32, i32, i32), Entity>,
    pub coords_to_unit: HashMap<(i32, i32, i32), Entity>,
}

pub struct UnlockedGenomes(pub Vec<AnimalType>);
