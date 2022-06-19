use std::time::Duration;

use bevy::prelude::*;
use clap::command;
use iyes_loopless::state::NextState;
use naia_bevy_server::Server;

use rgj_shared::{
    protocol::{
        countdown,
        map_sync::{MapSync, TileType, MAP_HEIGHT},
        Countdown as CountdownPacket, Protocol,
    },
    resources::MapConfig,
    Channels,
};

use crate::{
    components::{PerspectiveTileMap, TileMap},
    resources::{KeyMapAssociation, MainRoom},
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
    mut key_map_assoc: ResMut<KeyMapAssociation>,
) {
    info!("In countdown state -- preparing maps for players");
    let map_config = MapConfig {
        size_width: args.map_size_x,
        size_height: args.map_size_y,
    };

    let auth_map = &query_tilemap.get(main_room.map_entity).unwrap().children;

    for (index, key) in server.user_keys().into_iter().enumerate() {
        // FIXME: Support more than four players
        let mut sub_map_entities =
            Vec::with_capacity(args.map_size_x as usize * args.map_size_y as usize * 2);

        let (valid_xs, valid_ys, valid_zs) = if index == 0 {
            // Spawn player on left side
            let valid_xs = [0, 1, 2];

            let mid_y = args.map_size_y / 2;
            let valid_ys = [mid_y - 1, mid_y, mid_y + 1];

            let valid_zs = [0, 1];

            (valid_xs, valid_ys, valid_zs)
        } else if index == 1 {
            // Spawn player on right side
            let max_x = args.map_size_x;
            let valid_xs = [max_x - 2, max_x - 1, max_x];

            let mid_y = args.map_size_y / 2;
            let valid_ys = [mid_y - 1, mid_y, mid_y + 1];

            let valid_zs = [0, 1];

            (valid_xs, valid_ys, valid_zs)
        } else if index == 2 {
            // Spawn player on top
            let mid_x = args.map_size_x / 2;
            let valid_xs = [mid_x - 1, mid_x, mid_x + 1];

            let max_y = args.map_size_y;
            let valid_ys = [max_y - 2, max_y - 1, max_y];

            let valid_zs = [0, 1];

            (valid_xs, valid_ys, valid_zs)
        } else {
            // Spawn player on bottom
            let mid_x = args.map_size_x / 2;
            let valid_xs = [mid_x - 1, mid_x, mid_x + 1];

            let valid_ys = [0, 1, 2];

            let valid_zs = [0, 1];

            (valid_xs, valid_ys, valid_zs)
        };

        for z in 0..MAP_HEIGHT {
            for y in 0..args.map_size_y {
                for x in 0..args.map_size_x {
                    // If in valid_xs, valid_ys, and valid_zs, insert auth state into perceived
                    // state, otherwise insert a fog tile

                    if valid_zs.contains(&z) && valid_ys.contains(&y) && valid_xs.contains(&x) {
                        // Get the authoritative map sync
                        let map_sync = query_tile
                            .get(auth_map[TileMap::tile_xyz_to_index(&map_config, x, y, z)])
                            .unwrap()
                            .clone();

                        sub_map_entities.push(server.spawn().insert(map_sync).id());
                    } else {
                        sub_map_entities.push(
                            server
                                .spawn()
                                .insert(MapSync::new_complete(x, y, z, TileType::Fog))
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

    time.0 += clock.delta();

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
