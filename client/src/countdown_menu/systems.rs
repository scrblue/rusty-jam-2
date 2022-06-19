use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use bevy_prototype_lyon::prelude::*;
use iyes_loopless::state::NextState;
use naia_bevy_client::{
    events::{InsertComponentEvent, MessageEvent, SpawnEntityEvent},
    Client,
};

use rgj_shared::{
    behavior::HEXAGON_SIZE,
    protocol::{ClientKeepAlive, MapSync, Protocol, ProtocolKind},
    Channels,
};

use super::resources::SecondsLeft;
use crate::{game::resources::TurnTracker, GameState};

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

                let q = map_sync.position.column_q;
                let r = map_sync.position.row_r;
                let z = *map_sync.layer;

                // TODO: Will moving this so it's not created a thousand times improve performance?
                let shape = shapes::RegularPolygon {
                    sides: 6,
                    feature: shapes::RegularPolygonFeature::Radius(HEXAGON_SIZE),
                    ..Default::default()
                };

                let mut transform = Transform::from_xyz(
                    HEXAGON_SIZE * (q as f32 * f32::sqrt(3.0) + (f32::sqrt(3.0) / 2.0 * r as f32)),
                    HEXAGON_SIZE * (r as f32 * 3.0 / 2.0),
                    z as f32 * -1.0,
                );
                transform.rotate(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2));

                commands.spawn_bundle(GeometryBuilder::build_as(
                    &shape,
                    DrawMode::Outlined {
                        fill_mode: FillMode::color(color),
                        outline_mode: StrokeMode::new(Color::BLACK, 5.0),
                    },
                    transform,
                ));
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

pub fn receive_game_start_notification(
    mut event_reader: EventReader<MessageEvent<Protocol, Channels>>,
    mut commands: Commands,
) {
    for event in event_reader.iter() {
        if let MessageEvent(Channels::GameNotification, Protocol::GameStartNotification(gsn)) =
            event
        {
            commands.insert_resource(TurnTracker::new(&gsn.whose_turn));
            commands.insert_resource(NextState(GameState::Game));
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