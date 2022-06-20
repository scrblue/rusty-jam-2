use std::process::Termination;

use lazy_static::lazy_static;
use naia_shared::{derive_serde, serde};

// TODO: Implement diet
// TODO: Special effects per genome
// TODO: Genetic modifications
// TODO: Smarts-types for builders

lazy_static! {
    // Flying-types

    /// Chickens are the speed-oriented flying-type
    pub static ref CHICKEN: AnimalType = AnimalType {
        name: "Chicken".to_owned(),
        head: HeadStats {
            attack_damage: 5,
            smarts: 2,
        },
        body: BodyStats {
            health: 5,
            size_penalty: 0.5,
        },
        limbs: LimbStats {
            terrain_a: TerrainMovementStats {
                terrain_type: TerrainType::Air,
                tiles_per_turn: 3,
            },

            terrain_b: Some(TerrainMovementStats {
                terrain_type: TerrainType::Ground,
                tiles_per_turn: 2,
            }),
        }
    };

    /// Vampire bats are the damage-oriented flying-type
    pub static ref VAMPIRE_BAT: AnimalType = AnimalType {
        name: "Vampire-Bat".to_owned(),
        head: HeadStats {
            attack_damage: 10,
            smarts: 4,
        },
        body: BodyStats {
            health: 3,
            size_penalty: 0.5,
        },
        limbs: LimbStats {
            terrain_a: TerrainMovementStats {
                terrain_type: TerrainType::Air,
                tiles_per_turn: 2,
            },

            terrain_b: Some(TerrainMovementStats {
                terrain_type: TerrainType::Ground,
                tiles_per_turn: 1,
            }),
        }
    };

    /// Vultures are the body-oriented flying-type
    pub static ref VULTURE: AnimalType =  AnimalType {
        name: "Vulture".to_owned(),
        head: HeadStats {
            attack_damage: 8,
            smarts: 3,
        },
        body: BodyStats {
            health: 8,
            size_penalty: 1.5,
        },
        limbs: LimbStats {
            terrain_a: TerrainMovementStats {
                terrain_type: TerrainType::Air,
                tiles_per_turn: 2,
            },

            terrain_b: Some(TerrainMovementStats {
                terrain_type: TerrainType::Ground,
                tiles_per_turn: 2,
            }),
        }
    };

    // Ground-types

    // Deer are the speed-oriented ground-type
    // Rattlesnakes are the damage-oriented ground-type
    // Elephants are the body-oriented ground-type

    // Water-types

    // Sailfish are the speed-oriented water-type
    // Electric eels are the damage-oriented water-type
    // Whales are the body-oriented water-type
}

#[derive_serde]
pub struct Hybrid {
    head: AnimalType,
    body: AnimalType,
    limbs: AnimalType,

    name: String,
}

impl Hybrid {
    pub fn new(head: AnimalType, body: AnimalType, limbs: AnimalType) -> Self {
        Hybrid {
            name: format!("{}-{}-{}", head.name, body.name, limbs.name),
            head,
            body,
            limbs,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn head(&self) -> HeadStats {
        self.head.head
    }

    pub fn body(&self) -> BodyStats {
        self.body.body
    }

    pub fn limbs(&self) -> LimbStats {
        self.limbs.limbs
    }
}

#[derive_serde]
pub struct AnimalType {
    pub name: String,
    pub head: HeadStats,
    pub body: BodyStats,
    pub limbs: LimbStats,
}

#[derive(Copy)]
#[derive_serde]
pub struct HeadStats {
    attack_damage: u16,
    smarts: u16,
}

#[derive(Copy)]
#[derive_serde]
pub struct BodyStats {
    health: u16,
    // The amount to divide the tiles per turn by. Higher is worse.
    size_penalty: f32,
}

#[derive(Copy)]
#[derive_serde]
pub struct LimbStats {
    pub terrain_a: TerrainMovementStats,
    // The second [`TerrainMovementStats`] is optional in case an animal can only traverse one
    // [`TerrainType`]
    pub terrain_b: Option<TerrainMovementStats>,
}

#[derive(Copy)]
#[derive_serde]
pub struct TerrainMovementStats {
    pub terrain_type: TerrainType,
    pub tiles_per_turn: u8,
}

// TODO: Organize these components better. [`TerrainType`] should be elsewhere
#[derive(Copy)]
#[derive_serde]
pub enum TerrainType {
    Ground,
    Water,
    Air,
}
