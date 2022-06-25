use bevy::{ecs::query, prelude::*};
use naia_bevy_server::Server;

use rgj_shared::{
    behavior::AxialCoordinates,
    protocol::{
        game_sync::map_sync::{
            index_to_tile_qrz, tile_qrz_to_index, TileStructure, TileType, MAP_HEIGHT,
        },
        notifications::{game_start::GameStartNotification, WhoseTurn},
        MapSync, Protocol, UnitSync,
    },
    resources::MapConfig,
    Channels,
};

use crate::{
    components::TileMap,
    resources::{
        KeyIdAssociation, KeyMapAssociation, KeyUnitsAssociation, MainRoom, UsernameKeyAssociation,
    },
};

pub mod events;

pub mod resources;
use resources::{TurnTracker, UnitMoveInformation};

pub fn init(
    mut commands: Commands,
    server: Server<Protocol, Channels>,

    user_key_assoc: Res<UsernameKeyAssociation>,
    key_id_assoc: Res<KeyIdAssociation>,
) {
    let keys = server.user_keys().into_iter().collect();
    let turn_tracker = TurnTracker::new(server, &user_key_assoc, &key_id_assoc, keys, None);
    commands.insert_resource(turn_tracker);
    commands.insert_resource(UnitMoveInformation(None));
}

pub fn tick(
    mut server: Server<Protocol, Channels>,

    query_tilemap: Query<&TileMap>,
    mut query_tile: Query<&mut MapSync>,
    mut query_units: Query<&mut UnitSync>,

    mut move_info: ResMut<UnitMoveInformation>,

    map_config: Res<MapConfig>,
    main_room: Res<MainRoom>,
    key_map_assoc: Res<KeyMapAssociation>,
    key_units_assoc: Res<KeyUnitsAssociation>,
) {
    if let Some((entity, ref mut path)) = &mut move_info.0 {
        let mut unit_sync = query_units.get_mut(*entity).unwrap();

        let run_updates = match path.pop_front() {
            Some(next_stop) => {
                // Process the move making sure to update
                *unit_sync.position = next_stop;
                move_info.0 = None;
                true
            }
            None => {
                // Clear an empty path
                move_info.0 = None;
                false
            }
        };

        std::mem::drop(unit_sync);

        if run_updates {
            let auth_map = &query_tilemap.get(main_room.map_entity).unwrap().children;
            // Updates tiles in view for every player
            for user_key in server.user_keys() {
                if let Some(units) = key_units_assoc.get_from_key(user_key) {
                    // Build a list of hexes visible to any unit belonging to this player
                    let mut valid_qrs = Vec::new();
                    for unit in units {
                        if let Ok(unit_sync) = query_units.get(*unit) {
                            let viewing_distance =
                                unit_sync.hybrid_type.head().viewing_distance as i32;
                            let pos = *unit_sync.position;

                            // Finds all tiles within viewing_distance
                            for q_offset in -viewing_distance..=viewing_distance {
                                for r_offset in
                                    std::cmp::max(-viewing_distance, -q_offset - viewing_distance)
                                        ..=std::cmp::min(
                                            viewing_distance,
                                            -q_offset + viewing_distance,
                                        )
                                {
                                    let q = pos.column_q as i32 + q_offset;
                                    let r = pos.row_r as i32 + r_offset;

                                    if q >= 0
                                        && r >= 0
                                        && q <= i32::MAX.into()
                                        && r <= i32::MAX.into()
                                    {
                                        valid_qrs.push(AxialCoordinates::new(q as i32, r as i32));
                                    }
                                }
                            }
                        }
                    }

                    // With the tiles in range of all units, update the subjective map
                    let subjective_map = &query_tilemap
                        .get(*key_map_assoc.get_from_key(&user_key).unwrap())
                        .unwrap()
                        .children;

                    for z in 0..MAP_HEIGHT as i32 {
                        for r in 0..map_config.size_height as i32 {
                            for q in 0..map_config.size_width as i32 {
                                let qr = AxialCoordinates::new(q, r);

                                let [mut subj_tile, auth_tile] = query_tile
                                    .get_many_mut([
                                        subjective_map
                                            [TileMap::tile_qrz_to_index(&map_config, q, r, z)],
                                        auth_map[TileMap::tile_qrz_to_index(&map_config, q, r, z)],
                                    ])
                                    .unwrap();

                                if valid_qrs.contains(&qr) {
                                    *subj_tile.tile_type = *auth_tile.tile_type.clone();
                                    *subj_tile.structure = *auth_tile.structure.clone();
                                } else {
                                    if *subj_tile.tile_type != TileType::Fog {
                                        *subj_tile.tile_type = TileType::Fog;
                                        *subj_tile.structure = TileStructure::None;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    for (_, user_key, entity) in server.scope_checks() {
        // Only send updates from tiles in a user's perceived map
        let tilemap = &query_tilemap
            .get(*key_map_assoc.get_from_key(&user_key).unwrap())
            .unwrap()
            .children;
        let units = key_units_assoc.get_from_key(user_key);

        let mut in_scope = false;

        // If the unit belongs to a player, it should be in scope
        if let Some(units) = units {
            if units.contains(&entity) {
                in_scope = true;
                server.user_scope(&user_key).include(&entity);
            }
        }

        // If the tilemap is a part of that player's subjective map, it should be in scope
        if tilemap.contains(&entity) {
            in_scope = true;
            server.user_scope(&user_key).include(&entity);
        }

        // If the unit is simply in view, it should be in scope
        if let Ok(unit_any_player) = query_units.get(entity) {
            let pos = *unit_any_player.position;

            // So check if the tile a given unit is on is in view, if it is, the unit is also in view
            if let Ok(tile) = query_tile.get(
                tilemap[tile_qrz_to_index(&map_config, pos.column_q.into(), pos.row_r.into(), 0)],
            ) {
                if *tile.tile_type == TileType::Fog {
                    server.user_scope(&user_key).exclude(&entity);
                } else {
                    in_scope = true;
                    server.user_scope(&user_key).include(&entity);
                }
            }
        }

        if !in_scope {
            server.user_scope(&user_key).exclude(&entity);
        }
    }

    server.send_all_updates();
}
