//! A module defining components, resources, and systems specific to the WaitingForConnections GameState.

use bevy::prelude::*;
use iyes_loopless::prelude::*;
use naia_bevy_server::{Server, ServerAddrs};

use rgj_shared::{
    protocol::{Protocol, WaitingOnPlayers},
    Channels,
};

use crate::{
    resources::{MainRoomKey, UsernameKeyAssociation},
    Args, GameState,
};

pub mod events;

#[derive(Component)]
pub struct JunkComponent;

/// Initialization system
pub fn init(mut commands: Commands, mut server: Server<Protocol, Channels>, args: Res<Args>) {
    info!("Server running -- awaiting connections");

    let server_addresses = ServerAddrs::new(
        args.bind_udp,
        args.bind_web_rtc,
        &format!("http://{}", args.bind_web_rtc),
    );
    server.listen(&server_addresses);

    let main_room_key = server.make_room().key();
    commands.insert_resource(MainRoomKey(main_room_key));

    commands.insert_resource(UsernameKeyAssociation::new());
}

/// The tick fn will simply wait for the number of players to equal the configured, then enter the
/// countdown state
pub fn tick(mut commands: Commands, mut server: Server<Protocol, Channels>, args: Res<Args>) {
    // If there are exactly enough players, start the countdown
    if server.users_count() == args.num_players as usize {
        commands.insert_resource(NextState(GameState::Countdown));
    }

    // Update players on how many new connections they're waiting on
    // XXX: Be VERY certain the user count never exceeds the num_players so that it may never exceed u8::MAX.
    let waiting_on = WaitingOnPlayers::new_complete(args.num_players - server.users_count() as u8);
    for key in server.user_keys() {
        server.send_message(&key, Channels::WaitingOnPlayers, &waiting_on);
    }
}
