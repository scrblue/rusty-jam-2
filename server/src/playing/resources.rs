use std::{collections::VecDeque, time::Duration};

use bevy::prelude::*;
use naia_bevy_server::{Server, UserKey};

use rgj_shared::{
    behavior::AxialCoordinates,
    protocol::{notifications::WhoseTurn, GameStartNotification, Protocol, TurnChangeNotification},
    Channels,
};

use crate::resources::{KeyIdAssociation, UsernameKeyAssociation};

/// Tracks the current moving unit so that it walks along the given path
/// NOTE: This assumes the path has been verified as valid
pub struct UnitMoveInformation(pub Option<(Entity, VecDeque<AxialCoordinates>)>);

pub struct TurnTracker {
    pub player: UserKey,
    pub turn_number: u16,
    pub time_left: Option<Duration>,

    players: VecDeque<UserKey>,
}

impl TurnTracker {
    pub fn new(
        mut server: Server<Protocol, Channels>,

        user_key_assoc: &UsernameKeyAssociation,
        key_id_assoc: &KeyIdAssociation,

        mut players: VecDeque<UserKey>,
        turn_timers: Option<Duration>,
    ) -> TurnTracker {
        let player = players.pop_front().unwrap();
        players.push_back(player);

        for key in server.user_keys() {
            if key == player {
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
                        user_key_assoc.get_from_key(&player).unwrap().to_owned(),
                        *key_id_assoc.get_from_key(&player).unwrap(),
                    )),
                );
            }
        }

        server.send_all_updates();

        TurnTracker {
            player: player,
            turn_number: 0,
            time_left: turn_timers,
            players,
        }
    }

    pub fn next(
        &mut self,
        server: &mut Server<Protocol, Channels>,
        user_key_assoc: &UsernameKeyAssociation,
        key_id_assoc: &KeyIdAssociation,
    ) {
        let player = self.players.pop_front().unwrap();
        self.players.push_back(player);

        self.player = player;

        for key in server.user_keys() {
            if key == player {
                server.send_message(
                    &key,
                    Channels::GameNotification,
                    &TurnChangeNotification::new_complete(WhoseTurn::Yours),
                );
            } else {
                server.send_message(
                    &key,
                    Channels::GameNotification,
                    &TurnChangeNotification::new_complete(WhoseTurn::Player(
                        user_key_assoc.get_from_key(&player).unwrap().to_owned(),
                        *key_id_assoc.get_from_key(&player).unwrap(),
                    )),
                );
            }
        }
    }
}
