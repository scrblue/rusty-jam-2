// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::net::SocketAddr;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use iyes_loopless::prelude::*;
use naia_bevy_client::{ClientConfig, Plugin as ClientPlugin, Stage};

use rgj_shared::{protocol::Protocol, shared_config, Channels};

use rgj_client::connect_menu;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum GameState {
    ConnectMenu,
    WaitingForMoreConnectionsMenu,
    CountdownMenu,
    Game,
}

pub struct ConnectionInformation {
    socket_addr: SocketAddr,
    username: String,
    room_password: String,
}

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 1 })
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .insert_resource(WindowDescriptor {
            width: 800.,
            height: 600.,
            title: "Bevy game".to_string(), // ToDo
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(ClientPlugin::<Protocol, Channels>::new(
            ClientConfig::default(),
            shared_config(),
        ))
        .add_event::<connect_menu::ConnectEvent>()
        .add_loopless_state(GameState::ConnectMenu)
        // ConnectMenu state
        .add_enter_system(GameState::ConnectMenu, connect_menu::connect_menu_init)
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::ConnectMenu)
                .with_system(connect_menu::connect_menu)
                .with_system(monitor_for_addr)
                .into(),
        )
        //.add_exit_system(GameState::ConnectMenu, destroy_ui)
        .run();
}

/// Monitors for an event from the [`connect_menu::connect_menu`] so the game knows to begin
/// connection
fn monitor_for_addr(mut commands: Commands, mut events: EventReader<connect_menu::ConnectEvent>) {
    if let Some(connect_menu::ConnectEvent {
        socket_addr,
        username,
        room_password,
    }) = events.iter().next()
    {
        commands.insert_resource(ConnectionInformation {
            socket_addr: *socket_addr,
            username: username.clone(),
            room_password: room_password.clone(),
        });
        commands.insert_resource(NextState(GameState::WaitingForMoreConnectionsMenu));
        info!("HERE!");
    }
}
