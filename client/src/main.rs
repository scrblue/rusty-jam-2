// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::net::SocketAddr;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use iyes_loopless::prelude::*;
use naia_bevy_client::{ClientConfig, Plugin as ClientPlugin, Stage};

use rgj_shared::{protocol::Protocol, shared_config, Channels};

use rgj_client::{connect_menu, waiting_for_more_connections_menu, GameState};

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
        .add_loopless_state(GameState::ConnectMenu)
        // ConnectMenu state
        .add_enter_system(GameState::ConnectMenu, connect_menu::connect_menu_init)
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::ConnectMenu)
                .with_system(connect_menu::connect_menu)
                .into(),
        )
        // Waiting
        .add_enter_system(
            GameState::WaitingForMoreConnectionsMenu,
            waiting_for_more_connections_menu::systems::init,
        )
        .add_system_set_to_stage(
            Stage::Connection,
            ConditionSet::new()
                .run_in_state(GameState::WaitingForMoreConnectionsMenu)
                .with_system(waiting_for_more_connections_menu::systems::connection_event)
                .into(),
        )
        .add_system_set_to_stage(
            Stage::Disconnection,
            ConditionSet::new()
                .run_in_state(GameState::WaitingForMoreConnectionsMenu)
                .with_system(waiting_for_more_connections_menu::systems::disconnection_event)
                .into(),
        )
        .add_system_set_to_stage(
            Stage::ReceiveEvents,
            ConditionSet::new()
                .run_in_state(GameState::WaitingForMoreConnectionsMenu)
                .with_system(waiting_for_more_connections_menu::systems::spawn_entity_event)
                .with_system(waiting_for_more_connections_menu::systems::insert_component_event)
                .with_system(waiting_for_more_connections_menu::systems::update_component_event)
                .with_system(
                    waiting_for_more_connections_menu::systems::receive_waiting_on_players_message,
                )
                .into(),
        )
        .add_system_set_to_stage(
            Stage::Frame,
            ConditionSet::new()
                .run_in_state(GameState::WaitingForMoreConnectionsMenu)
                .with_system(
                    waiting_for_more_connections_menu::systems::waiting_for_more_connections_menu,
                )
                .into(),
        )
        .add_system_set_to_stage(
            Stage::Tick,
            ConditionSet::new()
                .run_in_state(GameState::WaitingForMoreConnectionsMenu)
                .with_system(waiting_for_more_connections_menu::systems::tick)
                .into(),
        )
        .run();
}
