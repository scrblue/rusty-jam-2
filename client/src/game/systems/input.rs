use std::str::Split;

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

use crate::game::resources::{Map, TileSelectedEvent, TurnTracker};

pub fn camera_system(
    mut ev_motion: EventReader<MouseMotion>,
    mut ev_scroll: EventReader<MouseWheel>,

    mut camera_query: Query<(&mut OrthographicProjection, &mut GlobalTransform)>,

    input_mouse_button: Res<Input<MouseButton>>,
) {
    let mut pan = Vec2::ZERO;
    let mut scroll = 0.0;

    if input_mouse_button.pressed(MouseButton::Middle) {
        for event in ev_motion.iter() {
            pan += event.delta;
        }
    }

    for event in ev_scroll.iter() {
        scroll += event.y;
    }

    let (mut camera_proj, mut camera_trans) = camera_query.get_single_mut().unwrap();

    if camera_proj.scale > 0.0 {
        camera_proj.scale -= scroll * 0.01;
        if camera_proj.scale < 0.0 {
            camera_proj.scale = 0.1;
        }
    }
    camera_trans.translation.x -= pan.x * camera_proj.scale;
    camera_trans.translation.y += pan.y * camera_proj.scale;
}

pub fn select_entity(
    mut entity_selected: EventWriter<TileSelectedEvent>,

    camera_transform_query: Query<(&GlobalTransform, &OrthographicProjection)>,

    input_mouse_button: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    egui_context: Res<EguiContext>,
) {
    if !egui_context.ctx().wants_pointer_input() {
        if input_mouse_button.just_pressed(MouseButton::Left) {
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

                if q >= 0.0 && r >= 0.0 && q <= u16::MAX as f32 && r <= u16::MAX as f32 {
                    let q = q as u16;
                    let r = r as u16;

                    let qr = AxialCoordinates::new(q, r);

                    info!("Clicked {} {}", q, r);

                    entity_selected.send(TileSelectedEvent(qr));
                }
            }
        }
    }
}
