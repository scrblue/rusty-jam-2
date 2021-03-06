use std::{net::SocketAddr, path::PathBuf};

use bevy::{log::LogPlugin, prelude::*};
use clap::{Parser, Subcommand};
use iyes_loopless::prelude::*;
use naia_bevy_server::{Plugin as ServerPlugin, ServerConfig, Stage};

use rgj_shared::{protocol::Protocol, shared_config, Channels};

mod components;
mod resources;

mod waiting_for_connections;
use waiting_for_connections::{
    events as waiting_events, init as waiting_init, tick as waiting_tick,
};

mod countdown;
use countdown::{events as countdown_events, init as countdown_init, tick as countdown_tick};

mod playing;
use playing::{events as playing_events, init as playing_init, tick as playing_tick};

const MAX_PLAYERS: u8 = 6;
const MIN_PLAYERS: u8 = 2;

#[derive(Parser)]
pub struct Args {
    bind_udp: SocketAddr,
    bind_web_rtc: SocketAddr,

    num_players: u8,
    room_password: String,

    #[clap(subcommand)]
    map_option: MapOption,
}

#[derive(Subcommand)]
pub enum MapOption {
    Generate { size_x: u16, size_y: u16 },
    Load { file_path: PathBuf },
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum GameState {
    WaitingForConnections,
    Countdown,
    Playing,
}

pub fn main() {
    let args = Args::parse();

    if args.num_players > MAX_PLAYERS || args.num_players < MIN_PLAYERS {
        panic!(
            "Number of players must be between {} and {}",
            MIN_PLAYERS, MAX_PLAYERS
        );
    }

    App::default()
        // Basic ECS stuff
        .add_plugins(MinimalPlugins)
        // Logging
        .add_plugin(LogPlugin::default())
        // Entity hierarchies and positions in space
        .add_plugin(HierarchyPlugin::default())
        .add_plugin(TransformPlugin::default())
        // naia server plugin
        .add_plugin(ServerPlugin::<Protocol, Channels>::new(
            ServerConfig::default(),
            shared_config(),
        ))
        // Insert resources
        .insert_resource(args)
        .add_loopless_state(GameState::WaitingForConnections)
        // WaitingForConnections state
        .add_enter_system(GameState::WaitingForConnections, waiting_init)
        .add_system_set_to_stage(
            Stage::ReceiveEvents,
            ConditionSet::new()
                .run_in_state(GameState::WaitingForConnections)
                .with_system(waiting_events::authorization_event)
                .with_system(waiting_events::connection_event)
                .with_system(waiting_events::disconnection_event)
                .with_system(waiting_events::receive_message_event)
                .into(),
        )
        .add_system_set_to_stage(
            Stage::Tick,
            ConditionSet::new()
                .run_in_state(GameState::WaitingForConnections)
                .with_system(waiting_tick)
                .into(),
        )
        // Countdown state
        .add_enter_system(GameState::Countdown, countdown_init)
        .add_system_set_to_stage(
            Stage::ReceiveEvents,
            ConditionSet::new()
                .run_in_state(GameState::Countdown)
                .with_system(countdown_events::authorization_event)
                .with_system(countdown_events::disconnection_event)
                .with_system(countdown_events::receive_message_event)
                .into(),
        )
        .add_system_set_to_stage(
            Stage::Tick,
            ConditionSet::new()
                .run_in_state(GameState::Countdown)
                .with_system(countdown_tick)
                .into(),
        )
        // Playing state
        .add_enter_system(GameState::Playing, playing_init)
        .add_system_set_to_stage(
            Stage::ReceiveEvents,
            ConditionSet::new()
                .run_in_state(GameState::Playing)
                .with_system(playing_events::receive_input_event)
                .into(),
        )
        .add_system_set_to_stage(
            Stage::Tick,
            ConditionSet::new()
                .run_in_state(GameState::Playing)
                .with_system(playing_tick)
                .into(),
        )
        .run()
}
