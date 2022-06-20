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
