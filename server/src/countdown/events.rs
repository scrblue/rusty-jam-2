use bevy::prelude::*;
use naia_bevy_server::{
    events::{AuthorizationEvent, ConnectionEvent, DisconnectionEvent, MessageEvent},
    Server,
};

use rgj_shared::{
    protocol::{ClientKeepAlive, Protocol, WaitingOnPlayers},
    Channels,
};

use crate::resources::{KeyMapAssociation, UsernameKeyAssociation};

pub fn authorization_event(
    mut event_reader: EventReader<AuthorizationEvent<Protocol>>,
    mut server: Server<Protocol, Channels>,
) {
    for event in event_reader.iter() {
        if let AuthorizationEvent(user_key, Protocol::Identification(_auth)) = event {
            // FIXME: Allow reconnection with reconnect password, otherwise deny
            server.reject_connection(user_key);
        }
    }
}

pub fn disconnection_event(
    mut event_reader: EventReader<DisconnectionEvent>,
    mut user_key_assoc: ResMut<UsernameKeyAssociation>,
    mut key_map_assoc: ResMut<KeyMapAssociation>,
) {
    for DisconnectionEvent(user_key, user) in event_reader.iter() {
        let username = user_key_assoc.get_from_key(user_key).unwrap();
        info!("Disconnecting from {} on {}", username, user.address);
        user_key_assoc.delete_from_key(user_key);

        // FIXME: Allow bot takeover or reconnection
        key_map_assoc.delete_from_key(user_key);
    }
}

pub fn receive_message_event(mut event_reader: EventReader<MessageEvent<Protocol, Channels>>) {
    for _ in event_reader.iter() {
        // Simply reads and discards ClientKeepAlive events
    }
}
