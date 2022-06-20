use std::time::Duration;

use bevy::{prelude::*, render::texture::DEFAULT_IMAGE_HANDLE};
use iyes_loopless::state::NextState;
use naia_bevy_server::{shared::Random, Server};

use rgj_shared::{
    behavior::AxialCoordinates,
    components::genome::{Hybrid, CHICKEN, DEER},
    protocol::{
        map_sync::{MapSync, TileStructure, TileType, MAP_HEIGHT},
        unit_sync::UnitSync,
        Countdown as CountdownPacket, Protocol,
    },
    resources::MapConfig,
    Channels,
};

use crate::{
    components::{PerspectiveTileMap, TileMap},
    resources::{KeyMapAssociation, KeyUnitsAssociation, MainRoom},
    Args, GameState,
};

pub mod events;

pub mod resources;
use resources::{Countdown, TimeSinceLastCount};

/// Initializes the countdown state by inserting necessary resources, and loading maps
pub fn init(
    mut commands: Commands,
    mut server: Server<Protocol, Channels>,

    query_tilemap: Query<&TileMap>,
    query_tile: Query<&MapSync>,

    args: Res<Args>,
    main_room: Res<MainRoom>,
    map_config: Res<MapConfig>,
    mut key_map_assoc: ResMut<KeyMapAssociation>,
    mut key_units_assoc: ResMut<KeyUnitsAssociation>,
) {
    info!("In countdown state -- preparing maps for players");

    let auth_map = &query_tilemap.get(main_room.map_entity).unwrap().children;

    // TODO: Map-aware spawning of intitial units
    // TODO: Don't spawn players too close
    let user_count = server.users_count();
    let mut starting_positions = Vec::with_capacity(user_count);
    for i in 0..user_count {
        let q = Random::gen_range_u32(0, map_config.size_width.into()) as u16;
        let r = Random::gen_range_u32(0, map_config.size_height.into()) as u16;

        starting_positions.push(AxialCoordinates::new(q, r));
    }

    for (index, key) in server.user_keys().into_iter().enumerate() {
        // FIXME: Support more than four players
        let mut sub_map_entities = Vec::with_capacity(
            map_config.size_width as usize * map_config.size_height as usize * 2,
        );

        let unit = server
            .spawn()
            .enter_room(&main_room.key)
            .insert(UnitSync::new_complete(
                starting_positions[index],
                0,
                Hybrid::new(DEER.clone(), DEER.clone(), DEER.clone()),
                DEER.body.health,
                DEER.limbs.terrain_a.tiles_per_turn.into(),
            ))
            .id();

        key_units_assoc.insert(key, unit);

        let viewing_distance = DEER.head.viewing_distance as i32;

        let mut valid_qrs = Vec::new();
        for q in -viewing_distance..viewing_distance {
            for r in std::cmp::max(-viewing_distance, -q - viewing_distance)
                ..std::cmp::min(viewing_distance, -q + viewing_distance)
            {
                if q >= 0 && r >= 0 && q <= u16::MAX.into() && r <= u16::MAX.into() {
                    valid_qrs.push(AxialCoordinates::new(q as u16, r as u16));
                }
            }
        }

        for z in 0..MAP_HEIGHT {
            for r in 0..map_config.size_height {
                for q in 0..map_config.size_width {
                    let qr = AxialCoordinates::new(q, r);

                    // If the tile is in view of the initial deer entity, send the authoritative
                    // state, otherwise send fog
                    if valid_qrs.contains(&qr) {
                        let map_sync = query_tile
                            .get(
                                auth_map[TileMap::tile_qrz_to_index(
                                    &map_config,
                                    qr.column_q,
                                    qr.row_r,
                                    z,
                                )],
                            )
                            .unwrap()
                            .clone();

                        sub_map_entities.push(
                            server
                                .spawn()
                                .enter_room(&main_room.key)
                                .insert(map_sync)
                                .id(),
                        );
                    } else {
                        sub_map_entities.push(
                            server
                                .spawn()
                                .enter_room(&main_room.key)
                                .insert(MapSync::new_complete(
                                    AxialCoordinates::new(q, r),
                                    z,
                                    TileType::Fog,
                                    TileStructure::None,
                                ))
                                .id(),
                        );
                    }
                }
            }
        }

        let subj_map = commands
            .spawn()
            .insert(PerspectiveTileMap(key))
            .insert(TileMap {
                children: sub_map_entities,
            })
            .id();

        key_map_assoc.insert(key, subj_map);
    }

    info!("Done preparing perspectives");
}

/// Simply does the countdown and handles scoping of all the components inserted above
pub fn tick(
    mut commands: Commands,
    mut server: Server<Protocol, Channels>,

    // Scoping stuff
    key_map_assoc: Res<KeyMapAssociation>,
    query_tilemap: Query<&TileMap>,

    // Countdown stuff
    mut countdown: ResMut<Countdown>,
    mut time: ResMut<TimeSinceLastCount>,
    clock: Res<Time>,
) {
    for (_, user_key, entity) in server.scope_checks() {
        // Only send updates from tiles in a user's perceived map
        let tilemap = &query_tilemap
            .get(*key_map_assoc.get_from_key(&user_key).unwrap())
            .unwrap()
            .children;

        if tilemap.contains(&entity) {
            server.user_scope(&user_key).include(&entity);
        } else {
            server.user_scope(&user_key).exclude(&entity);
        }
    }

    time.0 += Duration::from_secs_f32(clock.delta_seconds() * 1000.0);

    if time.0 > Duration::from_secs(1) {
        time.0 = Duration::from_secs(0);

        countdown.0 -= 1;

        if countdown.0 == 0 {
            commands.insert_resource(NextState(GameState::Playing));
        }

        for key in server.user_keys() {
            server.send_message(
                &key,
                Channels::Countdown,
                &CountdownPacket::new_complete(countdown.0),
            );
        }
    }

    server.send_all_updates();
}
