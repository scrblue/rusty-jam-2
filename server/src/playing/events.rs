use bevy::{ecs::query, prelude::*};
use naia_bevy_server::{events::MessageEvent, Server, UserKey};

use rgj_shared::{
    behavior::AxialCoordinates,
    components::genome::TerrainType,
    protocol::{
        map_sync::{tile_qrz_to_index, MapSync, TileType},
        player_input::PlayerInputVariant,
        PlayerInput, Protocol, TurnChangeNotification, UnitSync,
    },
    resources::MapConfig,
    Channels,
};

use super::resources::{TurnTracker, UnitMoveInformation};
use crate::{
    components::TileMap,
    resources::{KeyMapAssociation, KeyUnitsAssociation, MainRoom, UsernameKeyAssociation},
};

pub fn receive_input_event(
    mut server: Server<Protocol, Channels>,

    mut event_reader: EventReader<MessageEvent<Protocol, Channels>>,

    query_tilemap: Query<&TileMap>,
    query_tile: Query<&MapSync>,
    query_unit: Query<&UnitSync>,

    mut turn_tracker: ResMut<TurnTracker>,
    mut move_information: ResMut<UnitMoveInformation>,

    map_conf: Res<MapConfig>,
    user_key_assoc: Res<UsernameKeyAssociation>,
    key_units_assoc: Res<KeyUnitsAssociation>,
    main_room: Res<MainRoom>,
) {
    for event in event_reader.iter() {
        if let MessageEvent(user_key, Channels::PlayerInput, Protocol::PlayerInput(input)) = event {
            if turn_tracker.player == *user_key {
                match &*input.partial_turn {
                    PlayerInputVariant::MoveEntity(axial_coordinates) => {
                        // Only process if there is no active move
                        if move_information.0.is_none() {
                            match handle_move_entity(
                                &server,
                                *user_key,
                                input,
                                *axial_coordinates,
                                &query_tilemap,
                                &query_tile,
                                &query_unit,
                                &key_units_assoc,
                                &user_key_assoc,
                                &main_room,
                                &map_conf,
                            ) {
                                Ok(CanTravel::CanTravel(entity, steps_through)) => {
                                    move_information.0 =
                                        Some((entity, steps_through.into_iter().collect()));
                                }
                                Ok(CanTravel::InvalidTravel) => {}
                                Err(Error::Warn(msg)) => warn!("{}", msg),
                                Err(Error::Error(msg)) => error!("{}", msg),
                            }
                        }
                    }

                    PlayerInputVariant::EndTurn => {
                        turn_tracker.next(&mut server, &user_key_assoc);
                    }
                }
            }
        }
    }
}

pub enum Error {
    Warn(String),
    Error(String),
}

pub enum CanTravel {
    CanTravel(Entity, Vec<AxialCoordinates>),
    InvalidTravel,
}

fn handle_move_entity(
    server: &Server<Protocol, Channels>,

    senders_key: UserKey,
    input: &PlayerInput,
    axial_coordianates: AxialCoordinates,

    query_tilemap: &Query<&TileMap>,
    query_tile: &Query<&MapSync>,
    query_unit: &Query<&UnitSync>,

    key_units_assoc: &KeyUnitsAssociation,
    user_key_assoc: &UsernameKeyAssociation,
    main_room: &MainRoom,
    map_conf: &MapConfig,
) -> Result<CanTravel, Error> {
    let entity = input.relevant_entity.get(server).ok_or(Error::Warn(
        "Invalid Input: No EntityProperty with MoveEntity PlayerInput event".to_owned(),
    ))?;

    let key_which_owns_entity = key_units_assoc.get_from_entity(entity).ok_or(Error::Warn(
        "MoveEntity EntityProperty was not a valid unit Entity".to_owned(),
    ))?;

    let player_who_sent_message_name =
        user_key_assoc
            .get_from_key(&senders_key)
            .ok_or(Error::Error(
                "UserKey associated with input not in UsernameKeyAssociation".to_owned(),
            ))?;

    if senders_key != *key_which_owns_entity {
        return Err(Error::Warn(format!(
            "{} sent UserKey for entity which they do not own",
            player_who_sent_message_name
        )));
    }

    let unit_sync = query_unit
        .get(entity)
        .map_err(|_| Error::Error("Known unit Entity does not contain UnitSync".to_owned()))?;

    // TODO: Don't copy and paste this from client
    let entity_pos = *unit_sync.position;
    let desired_pos = axial_coordianates;

    // TODO: More hex utils

    // Calculate the distance between the the desired position and the current position,
    // caching what's needed
    let (dist, diff_q, diff_r) = {
        let (diff_q, diff_r) = (
            entity_pos.column_q as i32 - desired_pos.column_q as i32,
            entity_pos.row_r as i32 - desired_pos.row_r as i32,
        );

        let abs_q = i32::abs(diff_q);
        let abs_q_plus_r = i32::abs(diff_q + diff_r);
        let abs_r_div_2 = i32::abs(diff_r) / 2;

        ((abs_q + abs_q_plus_r + abs_r_div_2) as u16, diff_q, diff_r)
    };

    // Draw a line through the two points
    let mut travels_through: Vec<AxialCoordinates> = Vec::new();
    for i in 0..=dist {
        let step_q = desired_pos.column_q as f32 + diff_q as f32 * 1.0 / dist as f32 * i as f32;
        let step_r = desired_pos.row_r as f32 + diff_r as f32 * 1.0 / dist as f32 * i as f32;

        travels_through.push(AxialCoordinates::new(
            step_q.round() as u16,
            step_r.round() as u16,
        ));
    }
    info!("Travels Through: {:?}", travels_through);

    // Ensure that the player's unit has enough stamina and has the requisite terrain
    // types to cross this terrain
    let mut entity_stamina = *unit_sync.stamina_remaining;

    let mut terrain_types = Vec::with_capacity(2);
    terrain_types.push(unit_sync.hybrid_type.limbs().terrain_a.terrain_type);
    if let Some(terrain_b) = unit_sync.hybrid_type.limbs().terrain_b {
        terrain_types.push(terrain_b.terrain_type);
    }

    let mut can_travel = true;

    // Step through each point in the line
    for point in &travels_through {
        let map_entity = main_room.map_entity;

        // TODO: Eliminate unwraps
        let map = &query_tilemap.get(map_entity).unwrap().children;
        let tile = map[tile_qrz_to_index(&map_conf, point.column_q.into(), point.row_r.into(), 0)];
        let tile_state = query_tile.get(tile).unwrap();

        // TODO: More complex movement rules here -- traversing different terrains
        // differently
        // TODO: Queueing movement that cancels or pauses if cannont be completed
        // ever or immediately
        match *tile_state.tile_type {
            TileType::Fog => {
                can_travel = false;
                break;
            }
            TileType::Grass | TileType::Forest | TileType::Desert => {
                if terrain_types.contains(&TerrainType::Ground) {
                    if entity_stamina > 0 {
                        entity_stamina -= 1;
                    } else {
                        can_travel = false;
                        break;
                    }
                } else {
                    can_travel = false;
                    break;
                }
            }
            TileType::Ocean | TileType::River | TileType::DesertOasis => {
                if terrain_types.contains(&TerrainType::Water) {
                    if entity_stamina > 0 {
                        entity_stamina -= 1;
                    } else {
                        can_travel = false;
                        break;
                    }
                } else {
                    can_travel = false;
                    break;
                }
            }
            TileType::ClearSky | TileType::WindySky | TileType::StormySky => {
                if terrain_types.contains(&TerrainType::Air) {
                    if entity_stamina > 0 {
                        entity_stamina -= 1;
                    } else {
                        can_travel = false;
                        break;
                    }
                } else {
                    can_travel = false;
                    break;
                }
            }
        }
    }

    if can_travel {
        Ok(CanTravel::CanTravel(entity, travels_through))
    } else {
        Ok(CanTravel::InvalidTravel)
    }
}
