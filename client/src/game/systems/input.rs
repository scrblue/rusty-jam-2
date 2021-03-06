use std::str::Split;

use leafwing_input_manager::prelude::*;

use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};
use bevy_egui::EguiContext;
use bevy_prototype_lyon::prelude::*;

use rgj_shared::{
    behavior::{AxialCoordinates, HEXAGON_SIZE},
    protocol::{player_input::PlayerInputVariant, MapSync},
};

use super::Player;

use crate::game::resources::{Map, TileSelectedEvent, TurnTracker};

// This is the list of "things in the game I want to be able to do based on input"
#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum Action {
    Pan,
    Select,
    Zoom,
}
pub fn default_input_map() -> InputMap<Action> {
    let mut input_map = InputMap::default();
    input_map.insert(Action::Select, MouseButton::Left);
    input_map.insert(Action::Pan, MouseButton::Middle);
    input_map.insert(Action::Pan, KeyCode::LShift);
    input_map
}

pub fn pan_camera_system(
    mut ev_motion: EventReader<MouseMotion>,
    mut camera_query: Query<(&mut OrthographicProjection, &mut GlobalTransform)>,
    action_query: Query<&ActionState<Action>, With<Player>>,
) {
    let mut pan = Vec2::ZERO;
    let action_state = action_query.single();

    if action_state.pressed(Action::Pan) {
        for event in ev_motion.iter() {
            pan += event.delta;
        }
    }

    let (camera_proj, mut camera_trans) = camera_query.get_single_mut().unwrap();
    camera_trans.translation.x -= pan.x * camera_proj.scale;
    camera_trans.translation.y += pan.y * camera_proj.scale;
}

pub fn zoom_camera_system(
    mut ev_scroll: EventReader<MouseWheel>,
    mut camera_query: Query<(&mut OrthographicProjection, &mut GlobalTransform)>,
) {
    let mut scroll = 0.0;

    for event in ev_scroll.iter() {
        scroll += event.y;
    }

    let (mut camera_proj, _) = camera_query.get_single_mut().unwrap();

    if camera_proj.scale > 0.0 {
        camera_proj.scale -= scroll * 0.01;
        if camera_proj.scale < 0.0 {
            camera_proj.scale = 0.1;
        }
    }
}

pub fn select_entity(
    mut entity_selected: EventWriter<TileSelectedEvent>,
    action_query: Query<&ActionState<Action>, With<Player>>,
    camera_transform_query: Query<(&GlobalTransform, &OrthographicProjection)>,
    windows: Res<Windows>,
    mut egui_context: ResMut<EguiContext>,
) {
    if !egui_context.ctx_mut().is_pointer_over_area() {
        let action_state = action_query.single();
        if action_state.pressed(Action::Select) {
            let window = windows.get_primary().unwrap();

            if let Some(position) = window.cursor_position() {
                let (camera_trans, camera_proj) = camera_transform_query.get_single().unwrap();

                let camera_x = camera_trans.translation.x;
                let camera_y = camera_trans.translation.y;
                let camera_scale = camera_proj.scale;

                let x = position.x * camera_scale + camera_x - window.width() * camera_scale / 2.0;
                let y = position.y * camera_scale + camera_y - window.height() * camera_scale / 2.0;

                let mut q = (f32::sqrt(3.0) / 3.0 * x - 1.0 / 3.0 * y) / HEXAGON_SIZE;
                let mut r = (2.0 / 3.0 * y) / HEXAGON_SIZE;

                q = q.round();
                r = r.round();

                if q >= 0.0 && r >= 0.0 && q <= i32::MAX as f32 && r <= i32::MAX as f32 {
                    let q = q as i32;
                    let r = r as i32;

                    let qr = AxialCoordinates::new(q, r);

                    info!("Clicked {} {}", q, r);

                    entity_selected.send(TileSelectedEvent(qr));
                }
            }
        }
    }
}
