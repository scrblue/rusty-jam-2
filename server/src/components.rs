use bevy::prelude::*;
use naia_bevy_server::UserKey;

use rgj_shared::resources::MapConfig;

/// Component defining an entity as an entire tile map with MapSync entities as children.
#[derive(Component)]
pub struct TileMap {
    pub children: Vec<Entity>,
}
impl TileMap {
    /// Utility function for turning xyz coordinates into the index of the 1d [`Vec`] used to
    /// represent the map
    pub fn tile_qrz_to_index(map_conf: &MapConfig, q: i32, r: i32, z: i32) -> usize {
        (z as usize * map_conf.size_width as usize * map_conf.size_height as usize)
            + (r as usize * map_conf.size_width as usize)
            + q as usize
    }

    /// Utility function for turning an index of the 1d [`Vec`] used to represent the map into xyz
    /// coordinates
    pub fn index_to_tile_qrz(map_conf: &MapConfig, index: usize) -> (i32, i32, i32) {
        let mut index = index as i32;

        let z = index / (map_conf.size_width * map_conf.size_height) as i32;

        index -= z * (map_conf.size_width * map_conf.size_height) as i32;

        let r = index / map_conf.size_width as i32;
        let q = index % map_conf.size_width as i32;

        (q, r, z)
    }
}

/// Component tagging a tilemap entity as the authoritative state of the server
#[derive(Component)]
pub struct AuthoritativeTileMap;

/// Component tagging a tilemap entity as the subjective perspective of a given player identified
/// by [`UserKey`]
#[derive(Component)]
pub struct PerspectiveTileMap(pub UserKey);
