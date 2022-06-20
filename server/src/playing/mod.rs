use bevy::{ecs::query, prelude::*};
use naia_bevy_server::Server;

use rgj_shared::{
    protocol::{
        notifications::{game_start::GameStartNotification, WhoseTurn},
        Protocol,
    },
    Channels,
};

use crate::{
    components::TileMap,
    resources::{KeyMapAssociation, KeyUnitsAssociation, UsernameKeyAssociation},
};

pub mod events;

pub mod resources;
use resources::TurnTracker;

pub fn init(
    mut commands: Commands,
    server: Server<Protocol, Channels>,
    user_key_assoc: Res<UsernameKeyAssociation>,
) {
    let keys = server.user_keys().into_iter().collect();
    let turn_tracker = TurnTracker::new(server, &user_key_assoc, keys, None);
    commands.insert_resource(turn_tracker);
}

pub fn tick(
    mut server: Server<Protocol, Channels>,
    query_tilemap: Query<&TileMap>,

    key_map_assoc: Res<KeyMapAssociation>,
    key_units_assoc: Res<KeyUnitsAssociation>,
) {
    for (_, user_key, entity) in server.scope_checks() {
        // Only send updates from tiles in a user's perceived map
        let tilemap = &query_tilemap
            .get(*key_map_assoc.get_from_key(&user_key).unwrap())
            .unwrap()
            .children;

        let units = key_units_assoc.get_from_key(user_key);

        let mut in_scope = false;

        if let Some(units) = units {
            if units.contains(&entity) {
                in_scope = true;
                server.user_scope(&user_key).include(&entity);
            }
        }

        if tilemap.contains(&entity) {
            in_scope = true;
            server.user_scope(&user_key).include(&entity);
        }

        if !in_scope {
            server.user_scope(&user_key).exclude(&entity);
        }
    }

    server.send_all_updates();
}
