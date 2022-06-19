use std::convert::TryFrom;

use bevy::prelude::{Color, Component};
use naia_shared::{derive_serde, serde, Property, Replicate};
use thiserror::Error;

use crate::resources::MapConfig;

/// Represents the two layers, ground and air levels
pub const MAP_HEIGHT: u16 = 2;

#[derive(Copy)]
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

#[derive(Debug, Error)]
#[error("tile number given is out of bounds")]
pub struct TileIdOutOfBounds;

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

pub fn tile_xyz_to_index(map_conf: &MapConfig, x: i32, y: i32, z: i32) -> usize {
    (z as usize * map_conf.size_width as usize * map_conf.size_height as usize)
        + (y as usize * map_conf.size_width as usize)
        + x as usize
}

pub fn index_to_tile_xyz(map_conf: &MapConfig, index: usize) -> (u16, u16, u16) {
    let mut index = index as u16;

    let z = index / (map_conf.size_width * map_conf.size_height);

    index -= z * map_conf.size_width * map_conf.size_height;

    let y = index / map_conf.size_width;
    let x = index % map_conf.size_width;

    (x, y, z)
}

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
/// The synchronization of a tile on the map
pub struct MapSync {
    pub x: Property<u16>,
    pub y: Property<u16>,
    pub z: Property<u16>,
    pub tile_type: Property<TileType>,
}
