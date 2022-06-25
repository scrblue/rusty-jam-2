use bevy::prelude::*;
use naia_bevy_client::events::InsertComponentEvent;

use rgj_shared::{
    behavior::HEXAGON_SIZE,
    components::{genome::Hybrid, players::PlayerId},
    protocol::{ProtocolKind, UnitSync},
};

use crate::{game::resources::Map, UnitSprites};

pub mod chat;

fn insert_unit(
    commands: &mut Commands,
    transform: Transform,

    main_entity: Entity,

    id: PlayerId,
    hybrid: &Hybrid,

    sprites: &UnitSprites,
) {
    let head = match hybrid.head_name() {
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
    let body = match hybrid.body_name() {
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
    let limbs = match hybrid.limbs_name() {
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
