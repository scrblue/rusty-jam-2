use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};
use bevy_prototype_lyon::prelude::*;

use rgj_shared::{
    behavior::{AxialCoordinates, HEXAGON_SIZE},
    protocol::{
        player_input::{ClaimTile, PlayerInputVariant},
        MapSync,
    },
};

use crate::game::resources::{Map, TurnTracker};

pub fn select_tile_monitor(
    camera_transform_query: Query<(&GlobalTransform, &OrthographicProjection)>,

    mut query_draw: Query<&mut DrawMode>,
    query_auth: Query<&MapSync>,

    input_mouse_button: Res<Input<MouseButton>>,
    windows: Res<Windows>,

    mut turn_tracker: ResMut<TurnTracker>,
    map: Res<Map>,
) {
    if input_mouse_button.just_pressed(MouseButton::Left) {
        let window = windows.get_primary().unwrap();

        if let Some(position) = window.cursor_position() {
            let (camera_trans, camera_proj) = camera_transform_query.get_single().unwrap();

            let camera_x = camera_trans.translation.x;
            let camera_y = camera_trans.translation.y;
            let camera_scale = camera_proj.scale;

            // TODO: Get actual width and hegith from window
            let x = position.x + (camera_x - 400.0) / camera_scale;
            let y = position.y + (camera_y - 300.0) / camera_scale;

            let q = (f32::sqrt(3.0) / 3.0 * x - 1.0 / 3.0 * y) / HEXAGON_SIZE;
            let r = (2.0 / 3.0 * y) / HEXAGON_SIZE;

            if q > 0.0 && r > 0.0 {
                let q = q.round() as u16;
                let r = r.round() as u16;

                let qr = AxialCoordinates::new(q, r);

                // Insert the recorded command into the turn_tracker so they can be sent as a batch
                // at the end of the turn

                turn_tracker
                    .recorded_commands
                    .push(PlayerInputVariant::ClaimTile(ClaimTile { qr, layer: 0 }));

                // TODO: Trait or something for applying and removing predicted commands
                if let Some(entity) = map.coords_to_entity.get(&(q, r, 0)) {
                    if let Ok(mut draw_mode) = query_draw.get_mut(*entity) {
                        if let Ok(auth_state) = query_auth.get(*entity) {
                            let color: Color = (*auth_state.tile_type).into();

                            *draw_mode = DrawMode::Outlined {
                                fill_mode: FillMode::color(color),
                                outline_mode: StrokeMode::new(Color::ORANGE, 5.0),
                            };
                        }
                    }
                }
            }
        }
    }
}

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