use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use rgj_shared::{
    behavior::AxialCoordinates,
    protocol::{
        map_sync::{MapSync, TileStructure},
        UnitSync,
    },
};

use crate::game::resources::{Map, TileSelectedEvent, TileSelectedState};

// TODO: Only run on state for performance
pub fn display_info(
    mut tile_selected: EventReader<TileSelectedEvent>,

    map_sync_query: Query<&MapSync>,
    unit_sync_query: Query<&UnitSync>,

    map: Res<Map>,

    mut state: ResMut<TileSelectedState>,

    mut egui_context: ResMut<EguiContext>,
) {
    let mut error = String::new();
    if let Some(tile) = tile_selected.iter().last() {
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

        if let Some(entity) = state.moving_unit {
            error = format!(
                "Attempting to move unit to ({}, {}), but feature is unimplemented",
                tile.0.column_q, tile.0.row_r
            );
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
                if !error.is_empty() {
                    ui.label(error);
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
                if !error.is_empty() {
                    ui.label(error);
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
        }
        Change::None => {}
    }
}

pub enum Change {
    None,
    MoveUnit(AxialCoordinates, u16),
    CancelMoveUnit,
}
