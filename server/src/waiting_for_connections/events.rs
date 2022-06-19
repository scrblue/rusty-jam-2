use bevy::prelude::*;
use naia_bevy_server::{
    events::{AuthorizationEvent, ConnectionEvent, DisconnectionEvent, MessageEvent},
    Server,
};

use rgj_shared::{
    protocol::{Protocol, WaitingOnPlayers, ClientKeepAlive},
    Channels,
};

use crate::{
    resources::{KeyEntityAssociation, MainRoomKey, UsernameKeyAssociation},
    Args,
};

pub fn authorization_event(
    mut event_reader: EventReader<AuthorizationEvent<Protocol>>,
    mut server: Server<Protocol, Channels>,

    mut association: ResMut<UsernameKeyAssociation>,
    config: Res<Args>,
) {
    for event in event_reader.iter() {
        if let AuthorizationEvent(user_key, Protocol::Identification(auth)) = event {
            if *auth.room_password != config.room_password {
                info!("Rejecting connection: password invalid");
                server.reject_connection(user_key);
            }

            if association.get_from_name(&*auth.username).is_some() {
                info!("Rejecting connection: {} already connected", *auth.username);
                server.reject_connection(user_key);
            }

            if server.users_count() >= config.num_players as usize {
                info!("Rejecting connection, num_players exceeded");
                server.reject_connection(user_key);
            }

            info!("Accepting connection");
            association.insert(auth.username.to_string(), *user_key);
            server.accept_connection(user_key);
        }
    }
}

pub fn connection_event<'world, 'state>(
    mut event_reader: EventReader<ConnectionEvent>,
    mut server: Server<'world, 'state, Protocol, Channels>,

    main_room_key: Res<MainRoomKey>,
    association: Res<UsernameKeyAssociation>,
) {
    for ConnectionEvent(user_key) in event_reader.iter() {
        let address = server
            .user_mut(user_key)
            .enter_room(&main_room_key.0)
            .address();

        let username = association.get_from_key(user_key).unwrap();
        info!("Formed connection with {} on {}", username, address);

        let _entity = server
            .spawn()
            .enter_room(&main_room_key.0)
            .insert(ClientKeepAlive)
            .id();

        server.send_message(
            user_key,
            Channels::WaitingOnPlayers,
            &WaitingOnPlayers::new_complete(0),
        );
    }
}

pub fn disconnection_event(
    mut event_reader: EventReader<DisconnectionEvent>,
    mut association: ResMut<UsernameKeyAssociation>,
) {
    for DisconnectionEvent(user_key, user) in event_reader.iter() {
        let username = association.get_from_key(user_key).unwrap();
        info!("Disconnecting from {} on {}", username, user.address);
        association.delete_from_key(user_key);
    }
}

pub fn receive_message_event(mut event_reader: EventReader<MessageEvent<Protocol, Channels>>) {
    // NOOP in this state
    for _ in event_reader.iter() {
        info!("KEEP ALIVE");
    }
}
