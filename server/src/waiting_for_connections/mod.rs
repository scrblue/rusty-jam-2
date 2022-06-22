//! A module defining components, resources, and systems specific to the WaitingForConnections GameState.

use std::time::Duration;

use bevy::prelude::*;
use iyes_loopless::prelude::*;
use naia_bevy_server::{Server, ServerAddrs};

use rgj_shared::{
    behavior::AxialCoordinates,
    protocol::{
        map_sync::{MapSync, TileStructure, TileType, MAP_HEIGHT},
        Protocol, WaitingOnPlayers,
    },
    resources::MapConfig,
    Channels,
};

use crate::{
    components::{AuthoritativeTileMap, TileMap},
    countdown::resources::{Countdown, TimeSinceLastCount},
    resources::{KeyMapAssociation, KeyUnitsAssociation, MainRoom, UsernameKeyAssociation},
    Args, GameState, MapOption,
};

pub mod events;

fn init_tile(
    commands: &mut Commands,
    q: u16,
    r: u16,
    z: u16,
    tile: TileType,
    structure: TileStructure,
) -> Entity {
    commands
        .spawn()
        .insert(MapSync::new_complete(
            AxialCoordinates::new(q, r),
            z,
            tile,
            structure,
        ))
        .id()
}

/// Initialization system
pub fn init(mut commands: Commands, mut server: Server<Protocol, Channels>, args: Res<Args>) {
    info!("Server running -- awaiting connections");

    let main_room_key = server.make_room().key();
    match &args.map_option {
        MapOption::Generate { size_x, size_y } => {
            let size_x = *size_x;
            let size_y = *size_y;

            // Build AuthoritativeTileMap
            let mut auth_map_entities = Vec::with_capacity(size_x as usize * size_y as usize * 2);

            // TODO: Procedurally generate
            for z in 0..MAP_HEIGHT {
                for r in 0..size_y {
                    for q in 0..size_x {
                        if z == 1 {
                            auth_map_entities.push(init_tile(
                                &mut commands,
                                q,
                                r,
                                z,
                                TileType::ClearSky,
                                TileStructure::None,
                            ));
                        } else {
                            auth_map_entities.push(init_tile(
                                &mut commands,
                                q,
                                r,
                                z,
                                TileType::Ocean,
                                TileStructure::None,
                            ));
                        }
                    }
                }
            }

            let auth_map = commands
                .spawn()
                .insert(AuthoritativeTileMap)
                .insert(TileMap {
                    children: auth_map_entities,
                })
                .id();

            commands.insert_resource(MainRoom {
                key: main_room_key,
                map_entity: auth_map,
            });

            commands.insert_resource(MapConfig {
                size_width: size_x,
                size_height: size_y,
            });
        }

        MapOption::Load { file_path } => {
            let file_string = std::fs::read_to_string(file_path).expect("Given map invalid");

            let mut auth_map_entities = Vec::with_capacity(file_string.chars().count() * 2 / 3);

            // Will be the length of each line
            let mut x_size = 0;

            // Scan through for validity
            let mut char_count_for_line = 0;
            let mut line_count = 0;
            for c in file_string.chars() {
                match c {
                    '\n' => {
                        // If X_size hasn't been set, set it to the char_count_for_line for the
                        // first nonempty line, otherwise ensure lines are equal in length and panic
                        // otherwise
                        if x_size == 0 {
                            x_size = char_count_for_line;
                        } else if x_size != char_count_for_line {
                            panic!("Given map's lines must be equal in length")
                        }

                        char_count_for_line = 0;
                        line_count += 1;
                    }

                    _ => {
                        char_count_for_line += 1;
                    }
                }
            }

            if line_count % 3 != 0 {
                panic!("Given map must havea number of lines divisible by three")
            }

            let y_size = line_count / 3;

            // Read the tiles from the ground and air layers
            let tiles = file_string
                .chars()
                .filter(|c| *c != '\n')
                .take(y_size * 2 * x_size)
                .map(TryInto::try_into)
                .collect::<Result<Vec<TileType>, _>>()
                .expect("Unrecognized character");

            // Then read structures from the remaining layer
            let structures = file_string
                .chars()
                .filter(|c| *c != '\n')
                .skip(y_size * 2 * x_size)
                .map(TryInto::try_into)
                .collect::<Result<Vec<TileStructure>, _>>()
                .expect("Unrecognized character");

            // Consume them one by one filling the Vec
            let mut tiles = tiles.into_iter();
            let mut structures = structures.into_iter();
            for z in 0..MAP_HEIGHT {
                for r in 0..y_size {
                    for q in 0..x_size {
                        auth_map_entities.push(init_tile(
                            &mut commands,
                            q as u16,
                            r as u16,
                            z,
                            tiles.next().unwrap(),
                            structures.next().unwrap_or(TileStructure::None),
                        ));
                    }
                }
            }

            let auth_map = commands
                .spawn()
                .insert(AuthoritativeTileMap)
                .insert(TileMap {
                    children: auth_map_entities,
                })
                .id();

            commands.insert_resource(MainRoom {
                key: main_room_key,
                map_entity: auth_map,
            });

            commands.insert_resource(MapConfig {
                size_width: x_size as u16,
                size_height: y_size as u16,
            });
        }
    }

    let server_addresses = ServerAddrs::new(
        args.bind_udp,
        args.bind_web_rtc,
        &format!("http://{}", args.bind_web_rtc),
    );
    server.listen(&server_addresses);

    commands.insert_resource(UsernameKeyAssociation::new());
    commands.insert_resource(KeyMapAssociation::new());
    commands.insert_resource(KeyUnitsAssociation::new());
}

/// The tick fn will simply wait for the number of players to equal the configured, then enter the
/// countdown state
pub fn tick(mut commands: Commands, mut server: Server<Protocol, Channels>, args: Res<Args>) {
    // If there are exactly enough players, start the countdown
    if server.users_count() == args.num_players as usize {
        info!("Transitioning to countdown phase");
        commands.insert_resource(NextState(GameState::Countdown));

        // Insert resources needed for next state
        commands.insert_resource(Countdown(3));
        commands.insert_resource(TimeSinceLastCount(Duration::from_secs(0)));
    }

    for (_, user_key, entity) in server.scope_checks() {
        server.user_scope(&user_key).include(&entity);
    }

    // Update players on how many new connections they're waiting on
    // XXX: Be VERY certain the user count never exceeds the num_players so that it may never exceed u8::MAX.
    let waiting_on = WaitingOnPlayers::new_complete(args.num_players - server.users_count() as u8);
    for key in server.user_keys() {
        server.send_message(&key, Channels::WaitingOnPlayers, &waiting_on);
    }

    server.send_all_updates();
}
