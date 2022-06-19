use bevy::prelude::*;
use naia_bevy_server::Server;

use rgj_shared::{
    protocol::{
        notifications::{game_start::GameStartNotification, WhoseTurn},
        Protocol,
    },
    Channels,
};

use crate::resources::UsernameKeyAssociation;

pub mod resources;
use resources::TurnTracker;

pub fn init(
    mut commands: Commands,
    mut server: Server<Protocol, Channels>,
    user_key_assoc: Res<UsernameKeyAssociation>,
) {
    let key_first_turn = server.user_keys()[0];
    commands.insert_resource(TurnTracker {
        player: key_first_turn,
        turn_number: 1,
        // TODO: Allow timeouts
        time_left: None,
    });

    for key in server.user_keys() {
        if key == key_first_turn {
            server.send_message(
                &key,
                Channels::GameNotification,
                &GameStartNotification::new_complete(WhoseTurn::Yours),
            );
        } else {
            server.send_message(
                &key,
                Channels::GameNotification,
                &GameStartNotification::new_complete(WhoseTurn::Player(
                    user_key_assoc
                        .get_from_key(&key_first_turn)
                        .unwrap()
                        .to_owned(),
                )),
            );
        }
    }
}
