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
    pub fn tile_xyz_to_index(map_conf: &MapConfig, x: u16, y: u16, z: u16) -> usize {
        (z as usize * map_conf.size_width as usize * map_conf.size_height as usize)
            + (y as usize * map_conf.size_width as usize)
            + x as usize
    }

    /// Utility function for turning an index of the 1d [`Vec`] used to represent the map into xyz
    /// coordinates
    pub fn index_to_tile_xyz(map_conf: &MapConfig, index: usize) -> (u16, u16, u16) {
        let mut index = index as u16;

        let z = index / (map_conf.size_width * map_conf.size_height);

        index -= z * map_conf.size_width * map_conf.size_height;

        let y = index / map_conf.size_width;
        let x = index % map_conf.size_width;

        (x, y, z)
    }
}

/// Component tagging a tilemap entity as the authoritative state of the server
#[derive(Component)]
pub struct AuthoritativeTileMap;

/// Component tagging a tilemap entity as the subjective perspective of a given player identified
/// by [`UserKey`]
#[derive(Component)]
pub struct PerspectiveTileMap(pub UserKey);
