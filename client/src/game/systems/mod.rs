use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use bevy_prototype_lyon::prelude::*;
use naia_bevy_client::{
    events::{MessageEvent, UpdateComponentEvent},
    Client,
};

use rgj_shared::{
    protocol::{
        notifications::WhoseTurn, player_input::PlayerInputVariant, map_sync::{MapSync, TileType}, PlayerInput, Protocol,
        ProtocolKind,
    },
    Channels,
};

use crate::TileSprites;

use super::resources::{Map, TurnTracker};

pub mod input;
pub mod tile_info;

pub fn update_component_event(
    mut event_reader: EventReader<UpdateComponentEvent<ProtocolKind>>,

    query_auth: Query<&MapSync>,
    mut query_local: Query<&mut Handle<Image>>,

    assets: Res<TileSprites>,
) {
    for event in event_reader.iter() {
        if let UpdateComponentEvent(_tick, entity, ProtocolKind::MapSync) = event {
            if let Ok(map_sync) = query_auth.get(*entity) {
                let mut handle = query_local.get_mut(*entity).unwrap();
                let texture = match *map_sync.tile_type {
                    // FIXME: Fog should be fog
                    TileType::Fog => &assets.ocean,

                    TileType::Grass => &assets.grass,
                    TileType::Forest => &assets.forest,
                    TileType::Desert => &assets.desert,

                    TileType::Ocean => &assets.ocean,
                    // FIXME: River should be river
                    TileType::River => &assets.ocean,
                    TileType::DesertOasis => &assets.oasis,

                    TileType::ClearSky => &assets.clear_sky,
                    TileType::WindySky => &assets.windy_sky,
                    TileType::StormySky => &assets.stormy_sky,
                };

                *handle = texture.clone();
            }
        }
    }
}

pub fn game_menu(
    mut client: Client<Protocol, Channels>,

    mut query_draw: Query<&mut DrawMode>,
    query_auth: Query<&MapSync>,

    map: Res<Map>,

    mut turn_tracker: ResMut<TurnTracker>,
    mut egui_context: ResMut<EguiContext>,
) {
    let label = match &turn_tracker.whose_turn {
        WhoseTurn::Yours => "It is your turn".to_owned(),
        WhoseTurn::Player(string) => format!("It is {}'s turn", string),
    };

    let mut commit_turn = false;

    if turn_tracker.whose_turn == WhoseTurn::Yours {
        egui::Window::new("Turn Tracker").show(egui_context.ctx_mut(), |ui| {
            ui.label(label);
            commit_turn = ui.button("End Turn").clicked();
        });
    } else {
        egui::Window::new("Turn Tracker").show(egui_context.ctx_mut(), |ui| {
            ui.label(label);
        });
    }

    if commit_turn {
        for cmd in &turn_tracker.recorded_commands {
            // match cmd {}
        }

        client.send_message(
            Channels::PlayerInput,
            &PlayerInput::new_complete(std::mem::take(&mut turn_tracker.recorded_commands)),
        );
    }
}

pub fn receive_turn_change_notification(
    mut event_reader: EventReader<MessageEvent<Protocol, Channels>>,
    mut commands: Commands,
) {
    for event in event_reader.iter() {
        if let MessageEvent(Channels::GameNotification, Protocol::TurnChangeNotification(gsn)) =
            event
        {
            commands.insert_resource(TurnTracker::new(&gsn.whose_turn));
        }
    }
}
