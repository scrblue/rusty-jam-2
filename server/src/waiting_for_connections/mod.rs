//! A module defining components, resources, and systems specific to the WaitingForConnections GameState.

use std::time::Duration;

use bevy::prelude::*;
use iyes_loopless::prelude::*;
use naia_bevy_server::{Server, ServerAddrs};

use rgj_shared::{
    behavior::AxialCoordinates,
    protocol::{
        map_sync::{MapSync, TileType, MAP_HEIGHT},
        Protocol, WaitingOnPlayers,
    },
    resources::MapConfig,
    Channels,
};

use crate::{
    components::{AuthoritativeTileMap, TileMap},
    countdown::resources::{Countdown, TimeSinceLastCount},
    resources::{KeyMapAssociation, MainRoom, UsernameKeyAssociation},
    Args, GameState,
};

pub mod events;

/// Initialization system
pub fn init(mut commands: Commands, mut server: Server<Protocol, Channels>, args: Res<Args>) {
    info!("Server running -- awaiting connections");

    // Build AuthoritativeTileMap
    let mut auth_map_entities =
        Vec::with_capacity(args.map_size_x as usize * args.map_size_y as usize * 2);

    // TODO: Procedurally generate
    for z in 0..MAP_HEIGHT {
        for r in 0..args.map_size_y {
            for q in 0..args.map_size_x {
                if z == 1 {
                    auth_map_entities.push(
                        commands
                            .spawn()
                            .insert(MapSync::new_complete(
                                AxialCoordinates::new(q, r),
                                z,
                                TileType::ClearSky,
                            ))
                            .id(),
                    );
                } else {
                    auth_map_entities.push(
                        commands
                            .spawn()
                            .insert(MapSync::new_complete(
                                AxialCoordinates::new(q, r),
                                z,
                                TileType::Ocean,
                            ))
                            .id(),
                    );
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

    let server_addresses = ServerAddrs::new(
        args.bind_udp,
        args.bind_web_rtc,
        &format!("http://{}", args.bind_web_rtc),
    );
    server.listen(&server_addresses);

    let main_room_key = server.make_room().key();
    commands.insert_resource(MainRoom {
        key: main_room_key,
        map_entity: auth_map,
    });

    commands.insert_resource(UsernameKeyAssociation::new());
}

/// The tick fn will simply wait for the number of players to equal the configured, then enter the
/// countdown state
pub fn tick(mut commands: Commands, mut server: Server<Protocol, Channels>, args: Res<Args>) {
    // If there are exactly enough players, start the countdown
    if server.users_count() == args.num_players as usize {
        info!("Transitioning to countdown phase");
        commands.insert_resource(NextState(GameState::Countdown));

        // Insert resources needed for next state
        commands.insert_resource(KeyMapAssociation::new());
        commands.insert_resource(Countdown(10));
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
