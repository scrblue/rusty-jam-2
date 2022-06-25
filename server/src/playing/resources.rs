use std::{
    collections::{HashMap, HashSet, VecDeque},
    time::Duration,
};

use bevy::prelude::*;
use naia_bevy_server::{Server, UserKey};

use rgj_shared::{
    behavior::AxialCoordinates,
    components::genome::AnimalType,
    protocol::{
        game_sync::{
            map_sync::{tile_qrz_to_index, ConstructionStatus, MapSync, TileStructure},
            unit_sync::UnitSync,
        },
        notifications::WhoseTurn,
        GameStartNotification, Protocol, TurnChangeNotification,
    },
    resources::MapConfig,
    Channels,
};

use crate::{
    components::TileMap,
    resources::{KeyIdAssociation, KeyUnitsAssociation, MainRoom, UsernameKeyAssociation},
};

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

    // TODO: OnTurn event
    pub fn next(
        &mut self,
        server: &mut Server<Protocol, Channels>,
        user_key_assoc: &UsernameKeyAssociation,
        key_id_assoc: &KeyIdAssociation,

        query_tilemap: &Query<&TileMap>,
        // TODO: Query<&mut MapSync, With<Authoritative>>,
        query_tile: &mut Query<(Entity, &mut MapSync)>,

        map_config: MapConfig,
        main_room: &MainRoom,
        key_units_assoc: &mut KeyUnitsAssociation,
        should_update: &mut ShouldUpdate,
    ) {
        let player = self.players.pop_front().unwrap();
        self.players.push_back(player);

        self.player = player;

        if player == self.first_player {
            self.turn_number += 1;
        }

        let mut current_turn = WhoseTurn::Player {
            turn_number: self.turn_number,
            id: *key_id_assoc.get_from_key(&self.player).unwrap(),
            username: user_key_assoc.get_from_key(&self.player).unwrap().clone(),
        };

        let auth_map = &query_tilemap.get(main_room.map_entity).unwrap().children;

        let mut tiles_to_change = Vec::new();

        for (entity, tile) in query_tile.iter() {
            let MapSync {
                ref position,
                layer,
                ref structure,
                ..
            } = &*tile;

            if auth_map.contains(&entity) {
                if let TileStructure::GenomeFacility { ref building, .. } = **structure {
                    if let Some(ConstructionStatus {
                        building,
                        finished_on,
                    }) = building
                    {
                        if finished_on == &mut current_turn {
                            let id = server
                                .spawn()
                                .enter_room(&main_room.key)
                                .insert(UnitSync::new_complete(
                                    **position,
                                    **layer,
                                    *key_id_assoc.get_from_key(&self.player).unwrap(),
                                    building.clone(),
                                    building.body().health,
                                    building.limbs().terrain_a.tiles_per_turn.into(),
                                ))
                                .id();

                            should_update.0 = true;
                            key_units_assoc.insert(self.player, id);
                            tiles_to_change.push((**position, **layer));
                        }
                    }
                }
            }
        }
        for (qr, z) in &tiles_to_change {
            let (_entitiy, mut tile) = query_tile
                .get_mut(auth_map[tile_qrz_to_index(&map_config, qr.column_q, qr.row_r, *z)])
                .unwrap();

            if let TileStructure::GenomeFacility {
                ref mut building, ..
            } = *tile.structure
            {
                *building = None;
            }
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

pub struct ShouldUpdate(pub bool);
