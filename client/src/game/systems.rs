use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use naia_bevy_client::events::MessageEvent;

use rgj_shared::{
    protocol::{notifications::WhoseTurn, Protocol},
    Channels,
};

use super::resources::TurnTracker;

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
