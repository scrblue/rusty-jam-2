use bevy::{input::mouse::MouseMotion, prelude::*};
use bevy_egui::{egui, EguiContext};
use bevy_prototype_lyon::prelude::*;
use naia_bevy_client::{
    events::{MessageEvent, UpdateComponentEvent},
    Client,
};

use rgj_shared::{
    behavior::{AxialCoordinates, HEXAGON_SIZE, HEXAGON_WIDTH, HEXAGON_HEIGHT},
    protocol::{
        notifications::WhoseTurn,
        player_input::{ClaimTile, PlayerInputVariant},
        MapSync, PlayerInput, Protocol, ProtocolKind,
    },
    Channels,
};

use super::resources::TurnTracker;

pub fn update_component_event(
    mut event_reader: EventReader<UpdateComponentEvent<ProtocolKind>>,

    query_auth: Query<&MapSync>,
    mut query_local: Query<&mut DrawMode>,
) {
    for event in event_reader.iter() {
        if let UpdateComponentEvent(_tick, entity, ProtocolKind::MapSync) = event {
            if let Ok(map_sync) = query_auth.get(*entity) {
                let color: Color = (*map_sync.tile_type).into();

                let mut draw_mode = query_local.get_mut(*entity).unwrap();
                *draw_mode = DrawMode::Outlined {
                    fill_mode: FillMode::color(color),
                    outline_mode: StrokeMode::new(Color::BLACK, 5.0),
                };
            }
        }
    }
}

pub fn game_menu(turn_tracker: Res<TurnTracker>, mut egui_context: ResMut<EguiContext>) {
    let label = match &turn_tracker.whose_turn {
        WhoseTurn::Yours => "It is your turn".to_owned(),
        WhoseTurn::Player(string) => format!("It is {}'s turn", string),
    };

    egui::Window::new("Turn Tracker").show(egui_context.ctx_mut(), |ui| ui.label(label));
}

pub fn receive_turn_change_notification(
    mut event_reader: EventReader<MessageEvent<Protocol, Channels>>,
    mut commands: Commands,
) {
    for event in event_reader.iter() {
        if let MessageEvent(Channels::GameNotification, Protocol::TurnChangeNotification(gsn)) =
            event
        {
            commands.insert_resource(TurnTracker::new(&gsn.whose_turn));
        }
    }
}

pub fn select_tile_monitor(
    mut client: Client<Protocol, Channels>,
    input_mouse_button: Res<Input<MouseButton>>,
    windows: Res<Windows>,
) {
    if input_mouse_button.just_pressed(MouseButton::Left) {
        let window = windows.get_primary().unwrap();

        if let Some(position) = window.cursor_position() {
            let x = position.x - 400.0 + *HEXAGON_WIDTH/2.0;
            let y = position.y - 300.0 + HEXAGON_HEIGHT/2.0;

            let q = (f32::sqrt(3.0) / 3.0 * x - 1.0 / 3.0 * y) / HEXAGON_SIZE;
            let r = (2.0 / 3.0 * y) / HEXAGON_SIZE;

            if q > 0.0 && r > 0.0 {
                let q = q.round() as u16;
                let r = r.round() as u16;

                let qr = AxialCoordinates::new(q, r);
                client.send_message(
                    Channels::PlayerInput,
                    &PlayerInput::new_complete(vec![PlayerInputVariant::ClaimTile(ClaimTile {
                        qr,
                        layer: 0,
                    })]),
                );
            }
        }
    }
}
