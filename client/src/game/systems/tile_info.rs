use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use bevy_prototype_lyon::prelude::tess::geom::euclid::num::Round;
use naia_bevy_client::{shared::EntityHandleConverter, Client};

use rgj_shared::{
    behavior::AxialCoordinates,
    components::genome::TerrainType,
    protocol::{
        map_sync::{MapSync, TileStructure, TileType},
        player_input::PlayerInputVariant,
        PlayerInput, Protocol, UnitSync,
    },
    Channels,
};

use crate::game::resources::{Map, TileSelectedEvent, TileSelectedState};

// TODO: Only run on state for performance
pub fn display_info(
    mut client: Client<Protocol, Channels>,

    mut tile_selected: EventReader<TileSelectedEvent>,

    map_sync_query: Query<&MapSync>,
    unit_sync_query: Query<&UnitSync>,

    map: Res<Map>,

    mut state: ResMut<TileSelectedState>,

    mut egui_context: ResMut<EguiContext>,
) {
    if let Some(tile) = tile_selected.iter().last() {
        if let Some(entity) = state.moving_unit {
            if let Ok(unit_sync) = unit_sync_query.get(entity) {
                let entity_pos = *unit_sync.position;
                let desired_pos = tile.0;

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
                    let step_q =
                        desired_pos.column_q as f32 + diff_q as f32 * 1.0 / dist as f32 * i as f32;
                    let step_r =
                        desired_pos.row_r as f32 + diff_r as f32 * 1.0 / dist as f32 * i as f32;

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
                for point in travels_through {
                    if let Some(tile) = map.coords_to_tile.get(&(point.column_q, point.row_r, 0)) {
                        let tile_state = map_sync_query.get(*tile).unwrap();
                        // TODO: More complex movement rules here -- traversing different terrains
                        // differently
                        // TODO: Queueing movement that cancels or pauses if cannont be completed
                        // ever or immediately
                        match *tile_state.tile_type {
                            TileType::Fog => {
                                can_travel = false;
                                state.error = "You must be able to see through the fog to cross it -- try getting closer firt".to_owned();
                                break;
                            }
                            TileType::Grass | TileType::Forest | TileType::Desert => {
                                if terrain_types.contains(&TerrainType::Ground) {
                                    if entity_stamina > 0 {
                                        entity_stamina -= 1;
                                    } else {
                                        can_travel = false;
                                        state.error =
                                            "You do not have enough energy to walk this far"
                                                .to_owned();
                                        break;
                                    }
                                } else {
                                    can_travel = false;
                                    state.error =
                                        "This creature cannot walk on the ground".to_owned();
                                    break;
                                }
                            }
                            TileType::Ocean | TileType::River | TileType::DesertOasis => {
                                if terrain_types.contains(&TerrainType::Water) {
                                    if entity_stamina > 0 {
                                        entity_stamina -= 1;
                                    } else {
                                        can_travel = false;
                                        state.error =
                                            "You do not have enough energy to swim this far"
                                                .to_owned();
                                        break;
                                    }
                                } else {
                                    can_travel = false;
                                    state.error =
                                        "This creature cannot swim in the water".to_owned();
                                    break;
                                }
                            }
                            TileType::ClearSky | TileType::WindySky | TileType::StormySky => {
                                if terrain_types.contains(&TerrainType::Air) {
                                    if entity_stamina > 0 {
                                        entity_stamina -= 1;
                                    } else {
                                        can_travel = false;
                                        state.error =
                                            "You do not have enough energy to fly this far"
                                                .to_owned();
                                        break;
                                    }
                                } else {
                                    can_travel = false;
                                    state.error = "This creature cannot fly in the air".to_owned();
                                    break;
                                }
                            }
                        }
                    } else {
                        state.error = "Walking through tile that does not exist".to_owned();
                    }
                }

                // Finally if the travel is valid, send the message
                if can_travel {
                    let handle = client.entity_to_handle(&entity);
                    client.send_message(
                        Channels::PlayerInput,
                        &PlayerInput::new_complete(
                            vec![PlayerInputVariant::MoveEntity(handle, desired_pos)],
                            false,
                        ),
                    );
                }
            } else {
                state.error = "Fatal internal error in UnitSync-less unit".to_owned();
            }
        } else {
            state.tile = map
                .coords_to_tile
                .get(&(tile.0.column_q, tile.0.row_r, 0))
                .map(|e| map_sync_query.get(*e).map(|s| s.clone()).ok())
                .flatten();

            state.unit = map
                .coords_to_unit
                .get(&(tile.0.column_q, tile.0.row_r, 0))
                .map(|e| unit_sync_query.get(*e).map(|s| s.clone()).ok())
                .flatten();
        }
    }

    let change = match (&state.tile, &state.unit) {
        (
            Some(MapSync {
                position,
                layer,
                tile_type,
                structure,
            }),
            Some(UnitSync {
                hybrid_type,
                current_health,
                stamina_remaining,
                ..
            }),
        ) => {
            let mut toggle_move = false;
            egui::Window::new("Unit View").show(egui_context.ctx_mut(), |ui| {
                if !state.error.is_empty() {
                    ui.label(&state.error);
                }

                ui.horizontal(|ui| {
                    ui.label("Unit:");
                    ui.label(hybrid_type.name());
                });

                ui.horizontal(|ui| {
                    ui.label("Health:");
                    ui.label(format!(
                        "{} of {}",
                        **current_health,
                        hybrid_type.body().health
                    ));
                });

                ui.horizontal(|ui| {
                    ui.label("Remaining Stamina:");
                    ui.label(format!("{}", **stamina_remaining));
                });

                ui.horizontal(|ui| {
                    ui.label("Tile Type:");
                    ui.label((*tile_type).to_string());
                });

                match **structure {
                    TileStructure::None => {}
                    TileStructure::City => {
                        ui.label("Guarding a city");
                    }
                    TileStructure::GenomeFacility => {
                        ui.label("Guarding a genome facility");
                    }
                }

                if state.moving_unit.is_none() {
                    if ui.button("Move").clicked() {
                        toggle_move = true;
                    }
                } else {
                    if ui.button("Cancel Move").clicked() {
                        toggle_move = true;
                    }
                }
            });

            if toggle_move {
                if state.moving_unit.is_none() {
                    Change::MoveUnit(**position, **layer)
                } else {
                    Change::CancelMoveUnit
                }
            } else {
                Change::None
            }
        }
        (
            Some(MapSync {
                tile_type,
                structure,
                ..
            }),
            None,
        ) => {
            egui::Window::new("Tile View").show(egui_context.ctx_mut(), |ui| {
                if !state.error.is_empty() {
                    ui.label(&state.error);
                }
                ui.horizontal(|ui| {
                    ui.label("Tile Type:");
                    ui.label((*tile_type).to_string());
                });

                match **structure {
                    TileStructure::None => {}
                    TileStructure::City => {
                        ui.label("With a city");
                    }
                    TileStructure::GenomeFacility => {
                        ui.label("With a genome facility");
                    }
                }
            });
            Change::None
        }

        (None, None) => Change::None,
        (None, Some(..)) => {
            error!("Some unit but no tile on clicked location");
            Change::None
        }
    };

    match change {
        Change::MoveUnit(coord, layer) => {
            state.moving_unit = Some(
                *map.coords_to_unit
                    .get(&(coord.column_q, coord.row_r, layer))
                    .unwrap(),
            );
        }
        Change::CancelMoveUnit => {
            state.moving_unit = None;
            state.error = String::new();
        }
        Change::None => {}
    }
}

pub enum Change {
    None,
    MoveUnit(AxialCoordinates, u16),
    CancelMoveUnit,
}
