use bevy::prelude::*;
use naia_bevy_server::{
    events::{AuthorizationEvent, ConnectionEvent, DisconnectionEvent, MessageEvent},
    Server,
};

use rgj_shared::{
    components::players::PlayerId,
    protocol::{
        ClientConnected, ClientKeepAlive, Protocol, ReceiveChat, SendChat, WaitingOnPlayers,
    },
    Channels,
};

use crate::{
    resources::{KeyIdAssociation, MainRoom, UsernameKeyAssociation},
    Args,
};

pub const ID_ORDER: &[PlayerId] = &[
    PlayerId::Red,
    PlayerId::Blue,
    PlayerId::Yellow,
    PlayerId::Green,
    PlayerId::Purple,
    PlayerId::Orange,
];

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

            if server.users_count() > config.num_players as usize {
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

    main_room: Res<MainRoom>,
    username_key_assoc: Res<UsernameKeyAssociation>,
    mut key_id_assoc: ResMut<KeyIdAssociation>,
) {
    for ConnectionEvent(user_key) in event_reader.iter() {
        let address = server
            .user_mut(user_key)
            .enter_room(&main_room.key)
            .address();

        let username = username_key_assoc.get_from_key(user_key).unwrap();
        info!("Formed connection with {} on {}", username, address);

        let _entity = server
            .spawn()
            .enter_room(&main_room.key)
            .insert(ClientKeepAlive)
            .id();

        server.send_message(
            user_key,
            Channels::WaitingOnPlayers,
            &WaitingOnPlayers::new_complete(0),
        );

        // TODO: Ensure that PlayerId's don't mess up if multiple players connect on the same tick
        for key in server.user_keys() {
            server.send_message(
                &key,
                Channels::GameNotification,
                &ClientConnected::new(username.clone(), ID_ORDER[server.users_count() - 1]),
            );

            key_id_assoc.insert(key, ID_ORDER[server.users_count() - 1]);
        }
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

pub fn receive_message_event(
    mut server: Server<Protocol, Channels>,

    mut event_reader: EventReader<MessageEvent<Protocol, Channels>>,

    key_id_assoc: Res<KeyIdAssociation>,
) {
    for event in event_reader.iter() {
        if let MessageEvent(user_key, Channels::Chat, Protocol::SendChat(SendChat { message })) =
            event
        {
            let id = key_id_assoc.get_from_key(&user_key).unwrap();

            for key in server.user_keys() {
                server.send_message(
                    &key,
                    Channels::Chat,
                    &ReceiveChat::new(Some(*id), message.to_string()),
                );
            }
        }
    }
}
