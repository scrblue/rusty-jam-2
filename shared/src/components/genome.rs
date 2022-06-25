use lazy_static::lazy_static;
use naia_shared::{derive_serde, serde};

// TODO: Implement diet
// TODO: Special effects per genome
// TODO: Genetic modifications
// TODO: Smarts-types for builders

lazy_static! {
    // Flying-types -- these are generally weaker than other types, but have the advantage of flight

    /// Chickens are the speed-oriented flying-type
    pub static ref CHICKEN: AnimalType = AnimalType {
        name: "Chicken".to_owned(),
        head: HeadStats {
            attack_damage: 5,
            viewing_distance: 3,
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
            viewing_distance: 1,
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
            viewing_distance: 3,
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

    // Ground-types -- these are the middleground

    // Deer are the speed-oriented ground-type
    pub static ref DEER: AnimalType = AnimalType {
        name: "Deer".to_owned(),
        head: HeadStats {
            attack_damage: 8,
            viewing_distance: 2,
            smarts: 5,
        },
        body: BodyStats {
            health: 7,
            size_penalty: 0.8,
        },
        limbs: LimbStats {
            terrain_a: TerrainMovementStats {
                terrain_type: TerrainType::Ground,
                tiles_per_turn: 6,
            },
            terrain_b: Some(TerrainMovementStats {
                terrain_type: TerrainType::Water,
                tiles_per_turn: 1,
            })
        }
    };

    // Rattlesnakes are the damage-oriented ground-type
    pub static ref RATTLESNAKE: AnimalType = AnimalType {
        name: "Rattlesnake".to_owned(),
        head: HeadStats {
            attack_damage: 15,
            viewing_distance: 2,
            smarts: 2,
        },
        body: BodyStats {
            health: 5,
            size_penalty: 0.8,
        },
        limbs: LimbStats {
            terrain_a: TerrainMovementStats {
                terrain_type: TerrainType::Ground,
                tiles_per_turn: 3,
            },
            terrain_b: None,
        }
    };

    // Elephants are the body-oriented ground-type
    pub static ref ELEPHANT: AnimalType = AnimalType {
        name: "Elephants".to_owned(),
        head: HeadStats {
            attack_damage: 12,
            viewing_distance: 4,
            smarts: 10,
        },
        body: BodyStats {
            health: 18,
            size_penalty: 3.0,
        },
        limbs: LimbStats {
            terrain_a: TerrainMovementStats {
                terrain_type: TerrainType::Ground,
                tiles_per_turn: 6,
            },
            terrain_b: Some(TerrainMovementStats {
                terrain_type: TerrainType::Water,
                tiles_per_turn: 3,
            })
        }
    };

    // Water-types -- these are extremes, but have the disadvantage of only moving in water

    // Sailfish are the speed-oriented water-type
    pub static ref SAILFISH: AnimalType = AnimalType {
        name: "Sailfish".to_owned(),
        head: HeadStats  {
            attack_damage: 2,
            viewing_distance: 4,
            smarts: 2,
        },
        body: BodyStats {
            health: 4,
            size_penalty: 0.5,
        },
        limbs: LimbStats {
            terrain_a: TerrainMovementStats {
                terrain_type: TerrainType::Water,
                tiles_per_turn: 6,
            },
            terrain_b: None,
        }
    };

    // Electric eels are the damage-oriented water-type
    pub static ref ELECTRIC_EEL: AnimalType = AnimalType {
        name: "Electric-Eel".to_owned(),
        head: HeadStats  {
            attack_damage: 24,
            viewing_distance: 2,
            smarts: 2,
        },
        body: BodyStats {
            health: 2,
            size_penalty: 0.25,
        },
        limbs: LimbStats {
            terrain_a: TerrainMovementStats {
                terrain_type: TerrainType::Water,
                tiles_per_turn: 1,
            },
            terrain_b: None,
        }
    };

    // Whales are the body-oriented water-type
    pub static ref WHALE: AnimalType = AnimalType {
        name: "Whale".to_owned(),
        head: HeadStats  {
            attack_damage: 12,
            viewing_distance: 4,
            smarts: 10,
        },
        body: BodyStats {
            health: 24,
            size_penalty: 5.0,
        },
        limbs: LimbStats {
            terrain_a: TerrainMovementStats {
                terrain_type: TerrainType::Water,
                tiles_per_turn: 10,
            },
            terrain_b: None,
        }
    };
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

    pub fn head_type(&self) -> &str {
        &self.head.name
    }

    pub fn body(&self) -> BodyStats {
        self.body.body
    }

    pub fn body_type(&self) -> &str {
        &self.body.name
    }

    pub fn limbs(&self) -> LimbStats {
        self.limbs.limbs
    }

    pub fn limbs_type(&self) -> &str {
        &self.limbs.name
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
    pub attack_damage: u16,
    pub viewing_distance: u16,
    pub smarts: u16,
}

#[derive(Copy)]
#[derive_serde]
pub struct BodyStats {
    pub health: u16,
    // The amount to divide the tiles per turn by. Higher is worse.
    pub size_penalty: f32,
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
