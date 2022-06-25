use std::{
    collections::{HashMap, HashSet, VecDeque},
    time::Duration,
};

use bevy::prelude::*;
use naia_bevy_server::{Server, UserKey};

use rgj_shared::{
    behavior::AxialCoordinates,
    components::genome::AnimalType,
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

    first_player: UserKey,
    players: VecDeque<UserKey>,
}

impl TurnTracker {
    pub fn new(
        server: &mut Server<Protocol, Channels>,

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
                    &GameStartNotification::new_complete(WhoseTurn::Yours { turn_number: 1 }),
                );
            } else {
                server.send_message(
                    &key,
                    Channels::GameNotification,
                    &GameStartNotification::new_complete(WhoseTurn::Player {
                        username: user_key_assoc.get_from_key(&player).unwrap().to_owned(),
                        id: *key_id_assoc.get_from_key(&player).unwrap(),
                        turn_number: 1,
                    }),
                );
            }
        }

        server.send_all_updates();

        TurnTracker {
            player: player,
            turn_number: 1,
            time_left: turn_timers,
            first_player: player,
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

        if player == self.first_player {
            self.turn_number += 1;
        }

        for key in server.user_keys() {
            if key == player {
                server.send_message(
                    &key,
                    Channels::GameNotification,
                    &TurnChangeNotification::new_complete(WhoseTurn::Yours {
                        turn_number: self.turn_number,
                    }),
                );
            } else {
                server.send_message(
                    &key,
                    Channels::GameNotification,
                    &TurnChangeNotification::new_complete(WhoseTurn::Player {
                        username: user_key_assoc.get_from_key(&player).unwrap().to_owned(),
                        id: *key_id_assoc.get_from_key(&player).unwrap(),
                        turn_number: self.turn_number,
                    }),
                );
            }
        }
    }
}

pub struct KeyToUnlockedGenomesMap {
    pub key_to_genomes: HashMap<UserKey, Vec<AnimalType>>,
}
