use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use leafwing_input_manager::prelude::*;

use naia_bevy_client::{
    events::{InsertComponentEvent, MessageEvent, UpdateComponentEvent},
    Client,
};

use rgj_shared::{
    behavior::HEXAGON_SIZE,
    protocol::{
        map_sync::{MapSync, TileStructure, TileType},
        notifications::WhoseTurn,
        player_input::PlayerInputVariant,
        PlayerInput, Protocol, ProtocolKind, UnitSync,
    },
    Channels,
};

use crate::TileSprites;

use super::{
    components::TileWithBuilding,
    resources::{Map, TurnTracker},
};

pub mod input;
pub mod tile_info;

// TODO: Extract and don't copy paste from version in countdown
pub fn insert_unit_sync_event(
    mut event_reader: EventReader<InsertComponentEvent<ProtocolKind>>,
    mut commands: Commands,

    query: Query<&UnitSync>,

    mut map: ResMut<Map>,
) {
    for event in event_reader.iter() {
        if let InsertComponentEvent(entity, ProtocolKind::UnitSync) = event {
            if let Ok(unit_sync) = query.get(*entity) {
                let q = unit_sync.position.column_q;
                let r = unit_sync.position.row_r;
                let z = *unit_sync.layer;

                let transform = Transform::from_xyz(
                    HEXAGON_SIZE * (q as f32 * f32::sqrt(3.0) + (f32::sqrt(3.0) / 2.0 * r as f32)),
                    HEXAGON_SIZE * (r as f32 * 3.0 / 2.0),
                    z as f32 * -1.0 + 0.9,
                );

                commands.entity(*entity).insert_bundle(SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(32.0, 32.0)),
                        color: Color::ORANGE,
                        ..Default::default()
                    },
                    transform,
                    ..Default::default()
                });

                map.coords_to_unit.insert((q, r, z), *entity);
            }
        }
    }
}

#[derive(Component)]
pub struct Player;

pub fn spawn_player(mut commands: Commands) {
    let mut input_map = InputMap::default();
    //    input_map.insert([(input::Action::Select, KeyCode::P)]);
    input_map.insert(input::Action::Select, MouseButton::Left);
    input_map.insert_chord(input::Action::Pan, [KeyCode::LShift, KeyCode::P]);

    commands
        .spawn()
        .insert(Player)
        .insert_bundle(InputManagerBundle::<input::Action> {
            // Stores "which actions are currently pressed"
            action_state: ActionState::default(),
            // Describes how to convert from player inputs into those actions
            input_map,
        });
}

pub fn update_map_component_event(
    mut commands: Commands,

    mut event_reader: EventReader<UpdateComponentEvent<ProtocolKind>>,

    query_auth: Query<&MapSync>,
    query_translate: Query<&Transform>,
    mut query_handle: Query<&mut Handle<Image>>,
    // mut query_w_building: Query<&mut TileWithBuilding>,
    assets: Res<TileSprites>,
) {
    for event in event_reader.iter() {
        if let UpdateComponentEvent(_tick, entity, ProtocolKind::MapSync) = event {
            if let Ok(map_sync) = query_auth.get(*entity) {
                let mut handle = query_handle.get_mut(*entity).unwrap();
                let texture = match *map_sync.tile_type {
                    TileType::Fog => &assets.fog,

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

                if *map_sync.structure != TileStructure::None {
                    if let Ok(transform) = query_translate.get(*entity) {
                        let mut transform = transform.clone();

                        let color: Color = (*map_sync.structure).into();
                        transform.translation.z += 0.1;

                        let structure_entity = commands
                            .spawn_bundle(SpriteBundle {
                                sprite: Sprite {
                                    color,
                                    custom_size: Some(Vec2::new(65.0, 65.0)),
                                    ..Default::default()
                                },
                                transform: transform,
                                ..Default::default()
                            })
                            .id();

                        commands.entity(*entity).insert(TileWithBuilding {
                            structure_entity: structure_entity,
                        });
                    }
                }
                // Clean up if the entity has a TileWithBuilding
                else {
                    // TODO: Not sure why this doesn't work, but it doesn't, so it's a TODO
                    // if let Ok(twb) = query_w_building.get_mut(*entity) {
                    //     let struct_entity = twb.structure_entity;

                    //     commands.entity(struct_entity).despawn();
                    //     commands.entity(*entity).remove::<TileWithBuilding>();
                    // }
                }
            }
        }
    }
}

pub fn update_unit_component_event(
    mut event_reader: EventReader<UpdateComponentEvent<ProtocolKind>>,

    query_unit: Query<&UnitSync>,
    mut query_local: Query<&mut GlobalTransform>,

    mut map: ResMut<Map>,
) {
    for event in event_reader.iter() {
        if let UpdateComponentEvent(_tick, entity, ProtocolKind::UnitSync) = event {
            if let Ok(unit_sync) = query_unit.get(*entity) {
                let (q, r) = (unit_sync.position.column_q, unit_sync.position.row_r);

                let mut transform = query_local.get_mut(*entity).unwrap();
                let (old_q, old_r) = {
                    let mut q = (f32::sqrt(3.0) / 3.0 * transform.translation.x
                        - 1.0 / 3.0 * transform.translation.y)
                        / HEXAGON_SIZE;
                    let mut r = (2.0 / 3.0 * transform.translation.y) / HEXAGON_SIZE;

                    q = q.round();
                    r = r.round();

                    if q >= 0.0 && r >= 0.0 && q <= u16::MAX as f32 && r <= u16::MAX as f32 {
                        let q = q as u16;
                        let r = r as u16;

                        (q, r)
                    } else {
                        panic!("Could not fit in u16");
                    }
                };

                let new = GlobalTransform::from_xyz(
                    HEXAGON_SIZE * (q as f32 * f32::sqrt(3.0) + (f32::sqrt(3.0) / 2.0 * r as f32)),
                    HEXAGON_SIZE * (r as f32 * 3.0 / 2.0),
                    0.9,
                );
                *transform = new;

                map.coords_to_unit.remove(&(old_q, old_r, 0));
                map.coords_to_unit.insert((q, r, 0), *entity);
            }
        }
    }
}

pub fn game_menu(
    mut client: Client<Protocol, Channels>,

    turn_tracker: Res<TurnTracker>,
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
        client.send_message(
            Channels::PlayerInput,
            &PlayerInput::new_complete(PlayerInputVariant::EndTurn),
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
