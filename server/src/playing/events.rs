use bevy::{ecs::query, prelude::*};
use naia_bevy_server::events::MessageEvent;

use rgj_shared::{
    protocol::{
        map_sync::{MapSync, TileType},
        player_input::{ClaimTile, PlayerInputVariant},
        Protocol, TurnChangeNotification,
    },
    resources::MapConfig,
    Channels,
};

use super::resources::TurnTracker;
use crate::{
    components::TileMap,
    resources::{KeyMapAssociation, MainRoom},
};

pub fn receive_input_event(
    mut event_reader: EventReader<MessageEvent<Protocol, Channels>>,

    query_tilemap: Query<&TileMap>,
    mut query_tile: Query<&mut MapSync>,

    map_conf: Res<MapConfig>,
    turn_tracker: Res<TurnTracker>,
    main_room: Res<MainRoom>,
    key_map_assoc: Res<KeyMapAssociation>,
) {
    for event in event_reader.iter() {
        if let MessageEvent(user_key, Channels::PlayerInput, Protocol::PlayerInput(input)) = event {
            if turn_tracker.player == *user_key {
                let mut claims = 0;
                for input in &*input.turn_inputs {
                    match input {
                        PlayerInputVariant::ClaimTile(ClaimTile { qr, layer: _layer }) => {
                            if claims >= 3 {
                                break;
                            }

                            let q = qr.column_q;
                            let r = qr.row_r;

                            // Get the auth state of (q, r, 0)
                            let auth_map =
                                &query_tilemap.get(main_room.map_entity).unwrap().children;
                            let auth_tile = query_tile
                                .get(auth_map[TileMap::tile_qrz_to_index(&map_conf, q, r, 0)])
                                .unwrap()
                                .clone();

                            // Get the current perceived state of (q, r, 0)
                            let current_map = &query_tilemap
                                .get(*key_map_assoc.get_from_key(user_key).unwrap())
                                .unwrap()
                                .children;
                            let mut selected_tile = query_tile
                                .get_mut(
                                    current_map[TileMap::tile_qrz_to_index(&map_conf, q, r, 0)],
                                )
                                .unwrap();

                            // If it's a fog tile, set it to the auth tile, then add one to claims
                            if *selected_tile.tile_type == TileType::Fog {
                                *selected_tile.tile_type = *auth_tile.tile_type;
                                claims += 1;
                            }
                        }
                    }
                }
            }
        }
    }
}
