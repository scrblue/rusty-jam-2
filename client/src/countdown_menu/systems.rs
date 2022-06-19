use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use iyes_loopless::state::NextState;
use naia_bevy_client::{
    events::{InsertComponentEvent, MessageEvent, SpawnEntityEvent},
    Client,
};

use crate::GameState;
use rgj_shared::{
    protocol::{ClientKeepAlive, MapSync, Protocol, ProtocolKind},
    Channels,
};

use super::resources::SecondsLeft;

pub fn init(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

pub fn spawn_entity_event(mut event_reader: EventReader<SpawnEntityEvent>) {
    for _event in event_reader.iter() {
        debug!("Entity spawned");
    }
}

pub fn insert_component_event(
    mut event_reader: EventReader<InsertComponentEvent<ProtocolKind>>,
    mut commands: Commands,

    query: Query<&MapSync>,
) {
    for event in event_reader.iter() {
        if let InsertComponentEvent(entity, ProtocolKind::MapSync) = event {
            if let Ok(map_sync) = query.get(*entity) {
                let color: Color = (*map_sync.tile_type).into();

                commands.entity(*entity).insert_bundle(SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(15.0, 15.0)),
                        color,
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(
                        *map_sync.x as f32 * 15.0,
                        *map_sync.y as f32 * 15.0,
                        *map_sync.z as f32 * -1.0,
                    ),
                    ..Default::default()
                });
            }
        }
    }
}

pub fn receive_countdown_message(
    mut event_reader: EventReader<MessageEvent<Protocol, Channels>>,
    mut seconds_left: ResMut<SecondsLeft>,
) {
    for event in event_reader.iter() {
        if let MessageEvent(Channels::Countdown, Protocol::Countdown(cd)) = event {
            seconds_left.0 = *cd.secs_left;
        }
    }
}

pub fn countdown_menu(seconds_left: Res<SecondsLeft>, mut egui_context: ResMut<EguiContext>) {
    let label = format!("Starting in {} seconds", seconds_left.0);

    egui::Window::new("Countdown").show(egui_context.ctx_mut(), |ui| ui.label(label));
}

pub fn tick(mut client: Client<Protocol, Channels>) {
    client.send_message(Channels::ClientKeepAlive, &ClientKeepAlive);
}
