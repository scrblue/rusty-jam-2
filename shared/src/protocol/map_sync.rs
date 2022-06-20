use std::convert::TryFrom;

use bevy::prelude::{Color, Component};
use naia_shared::{derive_serde, serde, Property, Replicate};
use thiserror::Error;

use crate::{behavior::AxialCoordinates, resources::MapConfig};

/// Represents the two layers, ground and air levels
pub const MAP_HEIGHT: u16 = 2;

#[derive(Copy, Debug)]
#[derive_serde]
pub enum TileType {
    Fog,

    // Ground types
    Grass,
    Forest,
    Desert,

    // Water types
    Ocean,
    River,
    DesertOasis,

    // Air types
    ClearSky,
    WindySky,
    StormySky,
}

impl TileType {
    pub fn to_string(&self) -> String {
        match self {
            TileType::Fog => format!("Fog"),

            TileType::Grass => format!("Grass"),
            TileType::Forest => format!("Forest"),
            TileType::Desert => format!("Desert"),

            TileType::Ocean => format!("Ocean"),
            TileType::River => format!("River"),
            TileType::DesertOasis => format!("DesertOasis"),

            TileType::ClearSky => format!("Clear Skies"),
            TileType::WindySky => format!("Windy Skies"),
            TileType::StormySky => format!("Stormy Skies"),
        }
    }
}

#[derive(Debug, Error)]
#[error("tile number given is out of bounds")]
pub struct TileIdOutOfBounds;

#[derive(Debug, Error)]
#[error("the given character is not a valid map character")]
pub struct MapCharacterUnrecognized;

impl TryFrom<u8> for TileType {
    type Error = TileIdOutOfBounds;
    fn try_from(num: u8) -> Result<Self, TileIdOutOfBounds> {
        match num {
            0 => Ok(TileType::Fog),

            1 => Ok(TileType::Grass),
            2 => Ok(TileType::Forest),
            3 => Ok(TileType::Desert),

            4 => Ok(TileType::Ocean),
            5 => Ok(TileType::River),
            6 => Ok(TileType::DesertOasis),

            7 => Ok(TileType::ClearSky),
            8 => Ok(TileType::WindySky),
            9 => Ok(TileType::StormySky),

            _ => Err(TileIdOutOfBounds),
        }
    }
}

impl From<TileType> for u8 {
    fn from(ty: TileType) -> Self {
        match ty {
            TileType::Fog => 0,

            TileType::Grass => 1,
            TileType::Forest => 2,
            TileType::Desert => 3,

            TileType::Ocean => 4,
            TileType::River => 5,
            TileType::DesertOasis => 6,

            TileType::ClearSky => 7,
            TileType::WindySky => 8,
            TileType::StormySky => 9,
        }
    }
}

impl TryFrom<char> for TileType {
    type Error = MapCharacterUnrecognized;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'G' => Ok(TileType::Grass),
            'F' => Ok(TileType::Forest),
            'D' => Ok(TileType::Desert),

            'O' => Ok(TileType::Ocean),
            'R' => Ok(TileType::River),
            'o' => Ok(TileType::DesertOasis),

            'C' => Ok(TileType::ClearSky),
            'W' => Ok(TileType::WindySky),
            'S' => Ok(TileType::StormySky),

            _ => Err(MapCharacterUnrecognized),
        }
    }
}

impl From<TileType> for Color {
    fn from(ty: TileType) -> Self {
        match ty {
            TileType::Fog => Color::GRAY,

            TileType::Grass => Color::GREEN,
            TileType::Forest => Color::DARK_GREEN,
            TileType::Desert => Color::YELLOW,

            TileType::Ocean => Color::NAVY,
            TileType::River => Color::BLUE,
            TileType::DesertOasis => Color::TEAL,

            TileType::ClearSky => Color::NONE,
            TileType::WindySky => Color::WHITE,
            TileType::StormySky => Color::GRAY,
        }
    }
}

#[derive(Copy, Debug, Eq)]
#[derive_serde]
pub enum TileStructure {
    None,
    City,
    GenomeFacility,
}

impl TryFrom<char> for TileStructure {
    type Error = MapCharacterUnrecognized;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '_' => Ok(TileStructure::None),

            'c' => Ok(TileStructure::City),
            'g' => Ok(TileStructure::GenomeFacility),

            _ => Err(MapCharacterUnrecognized),
        }
    }
}

impl From<TileStructure> for Color {
    fn from(ty: TileStructure) -> Self {
        match ty {
            TileStructure::None => Color::NONE,
            TileStructure::City => Color::DARK_GRAY,
            TileStructure::GenomeFacility => Color::SILVER,
        }
    }
}

pub fn tile_qrz_to_index(map_conf: &MapConfig, q: i32, r: i32, z: i32) -> usize {
    (z as usize * map_conf.size_width as usize * map_conf.size_height as usize)
        + (r as usize * map_conf.size_width as usize)
        + q as usize
}

pub fn index_to_tile_qrz(map_conf: &MapConfig, index: usize) -> (u16, u16, u16) {
    let mut index = index as u16;

    let z = index / (map_conf.size_width * map_conf.size_height);

    index -= z * map_conf.size_width * map_conf.size_height;

    let r = index / map_conf.size_width;
    let q = index % map_conf.size_width;

    (q, r, z)
}

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
/// The synchronization of a tile on the map
pub struct MapSync {
    pub position: Property<AxialCoordinates>,
    pub layer: Property<u16>,
    pub tile_type: Property<TileType>,
    pub structure: Property<TileStructure>,
}
