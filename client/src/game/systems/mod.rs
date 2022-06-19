use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use bevy_prototype_lyon::prelude::*;
use naia_bevy_client::{
    events::{MessageEvent, UpdateComponentEvent},
    Client,
};

use rgj_shared::{
    protocol::{
        notifications::WhoseTurn, player_input::PlayerInputVariant, MapSync, PlayerInput, Protocol,
        ProtocolKind,
    },
    Channels,
};

use super::resources::{Map, TurnTracker};

pub mod input;

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

pub fn game_menu(
    mut client: Client<Protocol, Channels>,

    mut query_draw: Query<&mut DrawMode>,
    query_auth: Query<&MapSync>,

    map: Res<Map>,

    mut turn_tracker: ResMut<TurnTracker>,
    mut egui_context: ResMut<EguiContext>,
) {
    let label = match &turn_tracker.whose_turn {
        WhoseTurn::Yours => "It is your turn".to_owned(),
        WhoseTurn::Player(string) => format!("It is {}'s turn", string),
    };

    let mut commit_turn = false;

    if turn_tracker.whose_turn == WhoseTurn::Yours {
        egui::Window::new("Turn Tracker").show(egui_context.ctx_mut(), |ui| {
            ui.label(label);
            commit_turn = ui.button("End Turn").clicked();
        });
    } else {
        egui::Window::new("Turn Tracker").show(egui_context.ctx_mut(), |ui| {
            ui.label(label);
        });
    }

    if commit_turn {
        for cmd in &turn_tracker.recorded_commands {
            match cmd {
                PlayerInputVariant::ClaimTile(ct) => {
                    // Reset the queued actions' indicators so invalid commands don't stay on screen forever
                    // TODO: Trait or something for applying and removing predicted commands
                    if let Some(entity) =
                        map.coords_to_entity
                            .get(&(ct.qr.column_q, ct.qr.row_r, ct.layer))
                    {
                        if let Ok(mut draw_mode) = query_draw.get_mut(*entity) {
                            if let Ok(auth_state) = query_auth.get(*entity) {
                                let color: Color = (*auth_state.tile_type).into();

                                *draw_mode = DrawMode::Outlined {
                                    fill_mode: FillMode::color(color),
                                    outline_mode: StrokeMode::new(Color::BLACK, 5.0),
                                };
                            }
                        }
                    }
                }
            }
        }

        client.send_message(
            Channels::PlayerInput,
            &PlayerInput::new_complete(std::mem::take(&mut turn_tracker.recorded_commands)),
        );
    }
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
