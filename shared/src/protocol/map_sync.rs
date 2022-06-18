use std::convert::TryFrom;

use bevy::prelude::{Color, Component};
use naia_shared::{derive_serde, serde, Property, Replicate};
use thiserror::Error;

use crate::components::MapConfig;

#[derive(Copy)]
#[derive_serde]
pub enum TileType {
    Fog,
    Grass,
    Dirt,
    Water,
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
            2 => Ok(TileType::Dirt),
            3 => Ok(TileType::Water),
            _ => Err(TileIdOutOfBounds),
        }
    }
}

impl From<TileType> for u8 {
    fn from(ty: TileType) -> Self {
        match ty {
            TileType::Fog => 0,
            TileType::Grass => 1,
            TileType::Dirt => 2,
            TileType::Water => 3,
        }
    }
}

impl From<TileType> for Color {
    fn from(ty: TileType) -> Self {
        match ty {
            TileType::Fog => Color::GRAY,
            TileType::Grass => Color::DARK_GREEN,
            TileType::Dirt => Color::rgb_u8(116, 102, 59),
            TileType::Water => Color::TEAL,
        }
    }
}

pub fn tile_xy_to_index(map_conf: &MapConfig, x: i32, y: i32) -> usize {
    (y as usize * map_conf.size_width as usize) + x as usize
}

pub fn index_to_tile_xy(map_conf: &MapConfig, index: usize) -> (i32, i32) {
    (
        index as i32 % map_conf.size_width,
        index as i32 / map_conf.size_width,
    )
}

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
/// The synchronization of the map revealing the types of tiles of what's visible to the player
pub struct MapSync {
    pub map: Property<Vec<TileType>>,
}

impl MapSync {
    pub fn new_fog(map_conf: &MapConfig) -> MapSync {
        MapSync::new_complete(vec![
            TileType::Fog;
            map_conf.size_width as usize
                * map_conf.size_height as usize
        ])
    }

    pub fn new_ocean(map_conf: &MapConfig) -> MapSync {
        MapSync::new_complete(vec![
            TileType::Water;
            map_conf.size_width as usize
                * map_conf.size_height as usize
        ])
    }

    pub fn change_tile(&mut self, map_conf: &MapConfig, x: i32, y: i32, tile: TileType) {
        self.map[tile_xy_to_index(map_conf, x, y)] = tile;
    }
}
