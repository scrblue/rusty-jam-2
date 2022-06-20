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
    resources::{KeyMapAssociation, MainRoom, UsernameKeyAssociation},
};

pub fn receive_input_event(
    mut server: Server<Protocol, Channels>,

    mut event_reader: EventReader<MessageEvent<Protocol, Channels>>,

    query_tilemap: Query<&TileMap>,
    mut query_tile: Query<&mut MapSync>,

    map_conf: Res<MapConfig>,
    mut turn_tracker: ResMut<TurnTracker>,
    user_key_assoc: Res<UsernameKeyAssociation>,
    main_room: Res<MainRoom>,
    key_map_assoc: Res<KeyMapAssociation>,
) {
    for event in event_reader.iter() {
        if let MessageEvent(user_key, Channels::PlayerInput, Protocol::PlayerInput(input)) = event {
            if turn_tracker.player == *user_key {
                let mut claims = 0;
                for input in &*input.turn_inputs {
                    // match input {}
                }

                turn_tracker.next(&mut server, &user_key_assoc);
            }
        }
    }
}
