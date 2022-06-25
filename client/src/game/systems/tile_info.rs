use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use naia_bevy_client::{events::UpdateComponentEvent, shared::BigMapKey, Client};

use rgj_shared::{
    behavior::AxialCoordinates,
    components::genome::{AnimalType, Hybrid, TerrainType, DEER},
    protocol::{
        game_sync::map_sync::{ConstructionStatus, MapSync, TileStructure, TileType},
        player_input::PlayerInputVariant,
        PlayerInput, Protocol, ProtocolKind, UnitSync,
    },
    Channels,
};

use crate::game::resources::{Map, TileSelectedEvent, TileSelectedState, UnlockedGenomes};

// TODO: Only run on state for performance
pub fn display_info(
    mut client: Client<Protocol, Channels>,

    mut tile_selected: EventReader<TileSelectedEvent>,
    mut player_moves: EventReader<UpdateComponentEvent<ProtocolKind>>,

    map_sync_query: Query<&MapSync>,
    unit_sync_query: Query<&UnitSync>,

    map: Res<Map>,
    genomes: Res<UnlockedGenomes>,

    mut state: ResMut<TileSelectedState>,

    mut egui_context: ResMut<EguiContext>,
) {
    // If a unit moves
    for player_move in player_moves.iter() {
        if let UpdateComponentEvent(_tick, entity, ProtocolKind::UnitSync) = player_move {
            // And the entity equals to the moving unit
            if let Some(tracked_entity) = state.moving_unit {
                if tracked_entity == *entity {
                    // Then update the tracked  tile to the new position of the unit
                    if let Ok(unit_sync) = unit_sync_query.get(*entity) {
                        state.tile = Some(*unit_sync.position);
                    }
                }
            }
        }
    }

    // If a new tile is selected
    if let Some(tile) = tile_selected.iter().last() {
        // Then move the set moving unit to that tile if there is one
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

                    ((abs_q + abs_q_plus_r + abs_r_div_2) as i32, diff_q, diff_r)
                };

                // Draw a line through the two points
                let mut travels_through: Vec<AxialCoordinates> = Vec::new();
                for i in 0..=dist {
                    let step_q =
                        desired_pos.column_q as f32 + diff_q as f32 * 1.0 / dist as f32 * i as f32;
                    let step_r =
                        desired_pos.row_r as f32 + diff_r as f32 * 1.0 / dist as f32 * i as f32;

                    travels_through.push(AxialCoordinates::new(
                        step_q.round() as i32,
                        step_r.round() as i32,
                    ));
                }

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
                    let mut input =
                        PlayerInput::new_complete(PlayerInputVariant::MoveEntity(desired_pos));
                    input.relevant_entity.set(&client, &entity);

                    client.send_message(Channels::PlayerInput, &input);
                }
            } else {
                state.error = "Fatal internal error in UnitSync-less unit".to_owned();
            }
        }
        // If a unit isn't being tracked, then update the tracked tile
        else {
            state.tile = Some(tile.0);
        }
    }

    let tile = state
        .tile
        .map(|tile| {
            map.coords_to_tile
                .get(&(tile.column_q, tile.row_r, 0))
                .map(|e| map_sync_query.get(*e).map(|s| s.clone()).ok())
                .flatten()
        })
        .flatten();

    let unit = state
        .tile
        .map(|tile| {
            map.coords_to_unit
                .get(&(tile.column_q, tile.row_r, 0))
                .map(|e| unit_sync_query.get(*e).map(|s| s.clone()).ok())
                .flatten()
        })
        .flatten();

    let change = match (tile, unit) {
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
                        *current_health,
                        hybrid_type.body().health
                    ));
                });

                ui.horizontal(|ui| {
                    ui.label("Remaining Stamina:");
                    ui.label(format!("{}", *stamina_remaining));
                });

                ui.horizontal(|ui| {
                    ui.label("Tile Type:");
                    ui.label((*tile_type).to_string());
                });

                let mut on_facility = false;
                match &*structure {
                    TileStructure::None => {}
                    TileStructure::GenomeFacility {
                        unique_genome,
                        building,
                    } => {
                        on_facility = true;
                        let building = match building {
                            Some(ConstructionStatus {
                                building,
                                finished_on,
                            }) => format!(
                                " building a {} until turn {}",
                                building.name(),
                                finished_on.turn_number()
                            ),
                            None => "".to_owned(),
                        };

                        ui.label(format!(
                            "Guarding a genome facility containing {} genome{}",
                            unique_genome.name, building
                        ));
                    }
                }

                // TODO: Only display this button if the unit belongs to this player
                if state.moving_unit.is_none() {
                    if ui.button("Move").clicked() {
                        toggle_move = true;
                    }
                } else {
                    if ui.button("Cancel Move").clicked() {
                        toggle_move = true;
                    }
                }

                if state.build_screen {
                    if on_facility && ui.button("Close build menu").clicked() {
                        state.build_screen = false;
                    }
                } else {
                    if on_facility && ui.button("Open build menu").clicked() {
                        state.build_screen = true;
                    }
                }
            });

            let mut build_unit: Option<Hybrid> = None;
            if state.build_screen {
                egui::Window::new("Build View").show(egui_context.ctx_mut(), |ui| {
                    egui::ComboBox::from_label("Select Head")
                        .selected_text(format!(
                            "{}",
                            state.head.as_ref().unwrap_or(&DEER.clone()).name
                        ))
                        .show_ui(ui, |ui| {
                            for genome in &genomes.0 {
                                ui.selectable_value(
                                    &mut state.head,
                                    Some(genome.clone()),
                                    &genome.name,
                                );
                            }
                        });
                    egui::ComboBox::from_label("Select Body")
                        .selected_text(format!(
                            "{}",
                            state.body.as_ref().unwrap_or(&DEER.clone()).name
                        ))
                        .show_ui(ui, |ui| {
                            for genome in &genomes.0 {
                                ui.selectable_value(
                                    &mut state.body,
                                    Some(genome.clone()),
                                    &genome.name,
                                );
                            }
                        });
                    egui::ComboBox::from_label("Select Limbs")
                        .selected_text(format!(
                            "{}",
                            state.limbs.as_ref().unwrap_or(&DEER.clone()).name
                        ))
                        .show_ui(ui, |ui| {
                            for genome in &genomes.0 {
                                ui.selectable_value(
                                    &mut state.limbs,
                                    Some(genome.clone()),
                                    &genome.name,
                                );
                            }
                        });

                    if ui.button("Build").clicked() {
                        build_unit = Some(Hybrid::new(
                            state.head.take().unwrap_or_else(|| DEER.clone()),
                            state.body.take().unwrap_or_else(|| DEER.clone()),
                            state.limbs.take().unwrap_or_else(|| DEER.clone()),
                        ));
                    }
                });
            }

            if let Some(build_unit) = build_unit.take() {
                state.build_screen = false;
                Change::BuildUnit(*position, build_unit)
            } else if toggle_move {
                if state.moving_unit.is_none() {
                    Change::MoveUnit(*position, *layer)
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

                match &*structure {
                    TileStructure::None => {}
                    TileStructure::GenomeFacility {
                        unique_genome,
                        building,
                    } => {
                        let building = match building {
                            Some(ConstructionStatus {
                                building,
                                finished_on,
                            }) => format!(
                                " building a {} until turn {}",
                                building.name(),
                                finished_on.turn_number()
                            ),
                            None => "".to_owned(),
                        };

                        ui.label(format!(
                            "With a genome facility containing {} genome{}",
                            unique_genome.name, building
                        ));
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
            if let Some(unit) = map
                .coords_to_unit
                .get(&(coord.column_q, coord.row_r, layer))
            {
                state.moving_unit = Some(*unit);
            }
        }
        Change::CancelMoveUnit => {
            state.moving_unit = None;
            state.error = String::new();
        }
        Change::BuildUnit(pos, hybrid) => {
            client.send_message(
                Channels::PlayerInput,
                &PlayerInput::new_complete(PlayerInputVariant::BuildHybrid(pos, hybrid)),
            );
        }
        Change::None => {}
    }
}

pub enum Change {
    None,
    MoveUnit(AxialCoordinates, i32),
    CancelMoveUnit,
    BuildUnit(AxialCoordinates, Hybrid),
}
