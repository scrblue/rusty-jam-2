use bevy::{ecs::query, prelude::*};
use naia_bevy_server::{events::MessageEvent, Server};

use rgj_shared::{
    protocol::{
        map_sync::{MapSync, TileType},
        player_input::PlayerInputVariant,
        Protocol, TurnChangeNotification,
    },
    resources::MapConfig,
    Channels,
};

use super::resources::TurnTracker;
use crate::{
    components::TileMap,
    resources::{KeyMapAssociation, KeyUnitsAssociation, MainRoom, UsernameKeyAssociation},
};

pub fn receive_input_event(
    mut server: Server<Protocol, Channels>,

    mut event_reader: EventReader<MessageEvent<Protocol, Channels>>,

    query_tilemap: Query<&TileMap>,
    mut query_tile: Query<&mut MapSync>,

    map_conf: Res<MapConfig>,
    mut turn_tracker: ResMut<TurnTracker>,
    user_key_assoc: Res<UsernameKeyAssociation>,
    key_units_assoc: Res<KeyUnitsAssociation>,
    main_room: Res<MainRoom>,
    key_map_assoc: Res<KeyMapAssociation>,
) {
    for event in event_reader.iter() {
        if let MessageEvent(user_key, Channels::PlayerInput, Protocol::PlayerInput(input)) = event {
            if turn_tracker.player == *user_key {
                match &*input.partial_turn {
                    // XXX: Pickup here
                    PlayerInputVariant::MoveEntity(axial_coordinates) => {
                        if let Some(entity) = input.relevant_entity.get(&server) {
                            if let Some(entity_belongs_to) = key_units_assoc.get_from_entity(entity)
                            {
                                let player_sent_name =
                                    user_key_assoc.get_from_key(user_key).unwrap();
                                if entity_belongs_to == user_key {
                                    warn!(
                                        "Player {} moving entity to {:?}",
                                        player_sent_name, axial_coordinates
                                    );
                                } else {
                                    error!("Player {} sent MoveEntity with EntityProperty of other player's unit", player_sent_name);
                                }
                            } else {
                                error!("MoveEntity EntityProperty was not a valid unit Entity")
                            }
                        } else {
                            error!("MoveEntity input was not sent with associated EntityProperty");
                        }
                    }

                    PlayerInputVariant::EndTurn => {
						turn_tracker.next(&mut server, &user_key_assoc);
                    }
                }

            }
        }
    }
}
