// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_prototype_lyon::prelude::ShapePlugin;
use iyes_loopless::prelude::*;

use leafwing_input_manager::prelude::*;

use naia_bevy_client::{ClientConfig, Plugin as ClientPlugin, Stage};

use rgj_shared::{protocol::Protocol, shared_config, Channels};

use rgj_client::{
    connect_menu,
    countdown_menu::systems as countdown_systems,
    game::{resources::TileSelectedEvent, systems as game_systems},
    waiting_for_more_connections_menu::systems as waiting_systems,
    GameState,
};

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 1 })
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .insert_resource(WindowDescriptor {
            width: 800.,
            height: 600.,
            title: "Rusty Science".to_string(), // ToDo
            ..Default::default()
        })
        .add_event::<TileSelectedEvent>()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        // .insert_resource(WorldInspectorParams {
        //     despawnable_entities: true,
        //     highlight_changes: true,
        //     ..Default::default()
        // })
        // .add_plugin(WorldInspectorPlugin::new())
        // .add_plugin(InspectorPlugin::<Resources>::new())
        // // registers the type in the `bevy_reflect` machinery,
        // // so that even without implementing `Inspectable` we can display the struct fields
        // .register_type::<OrthographicProjection>()
        // This plugin maps inputs to an input-type agnostic action-state
        // We need to provide it with an enum which stores the possible actions a player could take
        .add_plugin(InputManagerPlugin::<game_systems::input::Action>::default())
        // XXX The InputMap and ActionState components will be added to any entity with the Player component
        .add_plugin(ClientPlugin::<Protocol, Channels>::new(
            ClientConfig::default(),
            shared_config(),
        ))
        .add_plugin(ShapePlugin)
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
            waiting_systems::init,
        )
        .add_system_set_to_stage(
            Stage::Connection,
            ConditionSet::new()
                .run_in_state(GameState::WaitingForMoreConnectionsMenu)
                .with_system(waiting_systems::connection_event)
                .into(),
        )
        .add_system_set_to_stage(
            Stage::Disconnection,
            ConditionSet::new()
                .run_in_state(GameState::WaitingForMoreConnectionsMenu)
                .with_system(waiting_systems::disconnection_event)
                .into(),
        )
        .add_system_set_to_stage(
            Stage::ReceiveEvents,
            ConditionSet::new()
                .run_in_state(GameState::WaitingForMoreConnectionsMenu)
                // On the server it sends spawn_entity_events and insert_component_events from the
                // Countdown state before sending the first countdown. This is a workaround to make
                // sure that entities still spawn while this happens
                .with_system(countdown_systems::spawn_entity_event)
                .with_system(countdown_systems::insert_map_sync_event)
                .with_system(countdown_systems::insert_unit_sync_event)
                .with_system(waiting_systems::receive_waiting_on_players_message)
                .with_system(waiting_systems::receive_countdown_message)
                .into(),
        )
        .add_system_set_to_stage(
            Stage::Frame,
            ConditionSet::new()
                .run_in_state(GameState::WaitingForMoreConnectionsMenu)
                .with_system(waiting_systems::waiting_for_more_connections_menu)
                .into(),
        )
        .add_system_set_to_stage(
            Stage::Tick,
            ConditionSet::new()
                .run_in_state(GameState::WaitingForMoreConnectionsMenu)
                .with_system(waiting_systems::tick)
                .into(),
        )
        // Countdown
        .add_enter_system(GameState::CountdownMenu, countdown_systems::init)
        .add_system_set_to_stage(
            Stage::ReceiveEvents,
            ConditionSet::new()
                .run_in_state(GameState::CountdownMenu)
                .with_system(countdown_systems::spawn_entity_event)
                .with_system(countdown_systems::insert_map_sync_event)
                .with_system(countdown_systems::insert_unit_sync_event)
                .with_system(countdown_systems::receive_countdown_message)
                .with_system(countdown_systems::receive_game_start_notification)
                .into(),
        )
        .add_system_set_to_stage(
            Stage::Frame,
            ConditionSet::new()
                .run_in_state(GameState::CountdownMenu)
                .with_system(countdown_systems::countdown_menu)
                .into(),
        )
        .add_system_set_to_stage(
            Stage::Tick,
            ConditionSet::new()
                .run_in_state(GameState::CountdownMenu)
                .with_system(countdown_systems::tick)
                .into(),
        )
        .add_system_set_to_stage(
            Stage::ReceiveEvents,
            ConditionSet::new()
                .run_in_state(GameState::Game)
                .with_system(game_systems::update_map_component_event)
                .with_system(game_systems::update_unit_component_event)
                .with_system(game_systems::insert_unit_sync_event)
                .with_system(game_systems::receive_turn_change_notification)
                .into(),
        )
        .add_system_set_to_stage(
            Stage::Frame,
            ConditionSet::new()
                .run_in_state(GameState::Game)
                .with_system(game_systems::game_menu)
                .with_system(game_systems::input::pan_camera_system)
                .with_system(game_systems::input::zoom_camera_system)
                .with_system(game_systems::input::select_entity)
                .with_system(game_systems::tile_info::display_info)
                .into(),
        )
        .run();
}
