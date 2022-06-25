use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use iyes_loopless::state::NextState;
use naia_bevy_client::{
    events::{InsertComponentEvent, MessageEvent, SpawnEntityEvent},
    Client,
};

use rgj_shared::{
    behavior::{HEXAGON_HEIGHT, HEXAGON_SIZE, HEXAGON_WIDTH},
    components::{genome::Hybrid, players::PlayerId},
    protocol::{
        game_sync::map_sync::{MapSync, TileStructure, TileType},
        ClientKeepAlive, Protocol, ProtocolKind, UnitSync,
    },
    Channels,
};

use super::resources::SecondsLeft;
use crate::{
    game::{
        components::TileWithBuilding,
        resources::{Map, TileSelectedState, TurnTracker},
    },
    GameState, TileSprites, UnitSprites,
};

pub fn init(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

pub fn spawn_entity_event(mut event_reader: EventReader<SpawnEntityEvent>) {
    for _event in event_reader.iter() {
        debug!("Entity spawned");
    }
}

fn insert_unit(
    commands: &mut Commands,
    transform: Transform,

    main_entity: Entity,

    id: PlayerId,
    hybrid: &Hybrid,

    sprites: &UnitSprites,
) {
    let head = match hybrid.head_type() {
        "Vampire-Bat" => sprites.fg_bat.clone(),
        "Chicken" => sprites.fg_chicken.clone(),
        "Deer" => sprites.fg_deer.clone(),
        "Electric-Eel" => sprites.fg_eel.clone(),
        "Elephant" => sprites.fg_elephant.clone(),
        "Rattlesnake" => sprites.fg_rattlesnake.clone(),
        "Sailfish" => sprites.fg_sailfish.clone(),
        "Vulture" => sprites.fg_vulture.clone(),
        "Whale" => sprites.fg_whale.clone(),
        _ => unreachable!(),
    };
    let body = match hybrid.body_type() {
        "Vampire-Bat" => sprites.fg_bat.clone(),
        "Chicken" => sprites.fg_chicken.clone(),
        "Deer" => sprites.fg_deer.clone(),
        "Electric-Eel" => sprites.fg_eel.clone(),
        "Elephant" => sprites.fg_elephant.clone(),
        "Rattlesnake" => sprites.fg_rattlesnake.clone(),
        "Sailfish" => sprites.fg_sailfish.clone(),
        "Vulture" => sprites.fg_vulture.clone(),
        "Whale" => sprites.fg_whale.clone(),
        _ => unreachable!(),
    };
    let limbs = match hybrid.limbs_type() {
        "Vampire-Bat" => sprites.fg_bat.clone(),
        "Chicken" => sprites.fg_chicken.clone(),
        "Deer" => sprites.fg_deer.clone(),
        "Electric-Eel" => sprites.fg_eel.clone(),
        "Elephant" => sprites.fg_elephant.clone(),
        "Rattlesnake" => sprites.fg_rattlesnake.clone(),
        "Sailfish" => sprites.fg_sailfish.clone(),
        "Vulture" => sprites.fg_vulture.clone(),
        "Whale" => sprites.fg_whale.clone(),
        _ => unreachable!(),
    };

    let bg = match id {
        PlayerId::Red => sprites.bg_red.clone(),
        PlayerId::Orange => sprites.bg_orange.clone(),
        PlayerId::Yellow => sprites.bg_yellow.clone(),
        PlayerId::Green => sprites.bg_green.clone(),
        PlayerId::Blue => sprites.bg_blue.clone(),
        PlayerId::Purple => sprites.bg_purple.clone(),
    };

    let top = commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(25.0, 25.0)),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, 15.0, 0.1),
            texture: head,
            ..Default::default()
        })
        .id();
    let left = commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(25.0, 25.0)),
                ..Default::default()
            },
            transform: Transform::from_xyz(-15.0, -15.0, 0.1),
            texture: body,
            ..Default::default()
        })
        .id();
    let right = commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(25.0, 25.0)),
                ..Default::default()
            },
            transform: Transform::from_xyz(15.0, -15.0, 0.1),
            texture: limbs,
            ..Default::default()
        })
        .id();

    commands
        .entity(main_entity)
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(65.0, 65.0)),
                ..Default::default()
            },
            transform,
            texture: bg,
            ..Default::default()
        })
        .push_children(&[top, left, right]);
}

pub fn insert_unit_sync_event(
    mut event_reader: EventReader<InsertComponentEvent<ProtocolKind>>,
    mut commands: Commands,

    query: Query<&UnitSync>,

    mut map: ResMut<Map>,
    unit_sprites: Res<UnitSprites>,
) {
    for event in event_reader.iter() {
        if let InsertComponentEvent(entity, ProtocolKind::UnitSync) = event {
            if let Ok(unit_sync) = query.get(*entity) {
                let q = unit_sync.position.column_q;
                let r = unit_sync.position.row_r;
                let z = *unit_sync.layer as i32;

                let transform = Transform::from_xyz(
                    HEXAGON_SIZE * (q as f32 * f32::sqrt(3.0) + (f32::sqrt(3.0) / 2.0 * r as f32)),
                    HEXAGON_SIZE * (r as f32 * 3.0 / 2.0),
                    z as f32 * -1.0 + 0.9,
                );

                insert_unit(
                    &mut commands,
                    transform,
                    *entity,
                    *unit_sync.player_id,
                    &unit_sync.hybrid_type,
                    &unit_sprites,
                );

                map.coords_to_unit.insert((q, r, z), *entity);
            }
        }
    }
}

pub fn insert_map_sync_event(
    mut event_reader: EventReader<InsertComponentEvent<ProtocolKind>>,
    mut commands: Commands,

    query: Query<&MapSync>,

    mut map: ResMut<Map>,
    assets: Res<TileSprites>,
) {
    for event in event_reader.iter() {
        if let InsertComponentEvent(entity, ProtocolKind::MapSync) = event {
            if let Ok(map_sync) = query.get(*entity) {
                let q = map_sync.position.column_q;
                let r = map_sync.position.row_r;
                let z = *map_sync.layer;

                let mut transform = Transform::from_xyz(
                    HEXAGON_SIZE * (q as f32 * f32::sqrt(3.0) + (f32::sqrt(3.0) / 2.0 * r as f32)),
                    HEXAGON_SIZE * (r as f32 * 3.0 / 2.0),
                    z as f32 * -1.0,
                );

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

                commands.entity(*entity).insert_bundle(SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(*HEXAGON_WIDTH, HEXAGON_HEIGHT)),
                        ..Default::default()
                    },
                    transform,
                    texture: texture.clone(),
                    ..Default::default()
                });

                // Insert the building if there is one
                if *map_sync.structure != TileStructure::None {
                    let color: Color = (*map_sync.structure).into();
                    transform.translation.z += 0.1;

                    let structure_entity = commands
                        .spawn_bundle(SpriteBundle {
                            sprite: Sprite {
                                color,
                                custom_size: Some(Vec2::new(65.0, 65.0)),
                                ..Default::default()
                            },
                            transform,
                            ..Default::default()
                        })
                        .id();

                    commands
                        .entity(*entity)
                        .insert(TileWithBuilding { structure_entity });
                }

                map.coords_to_tile.insert((q, r, z), *entity);
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
            commands.insert_resource(TileSelectedState::default());
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
