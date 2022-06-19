use std::{net::SocketAddr, str::FromStr};

use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use iyes_loopless::prelude::*;

use crate::{ConnectionInformation, GameState};

#[derive(Default)]
pub struct UiState {
    error_msg: String,

    socket_addr_s: String,

    username: String,
    password: String,
}

pub fn connect_menu_init(mut commands: Commands) {
    commands.insert_resource(UiState::default());
    commands.insert_resource(ConnectionInformation::default());
}

pub fn connect_menu(
    mut commands: Commands,

    mut egui_context: ResMut<EguiContext>,
    mut ui_state: ResMut<UiState>,
) {
    let mut clicked = false;

    egui::Window::new("Connect").show(egui_context.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            ui.label("Socket Address");
            ui.text_edit_singleline(&mut ui_state.socket_addr_s);
        });

        ui.horizontal(|ui| {
            ui.label("Username");
            ui.text_edit_singleline(&mut ui_state.username);
        });

        ui.horizontal(|ui| {
            ui.label("Room Password");
            ui.text_edit_singleline(&mut ui_state.password);
        });

        clicked = ui.button("Connect").clicked();

        ui.label(&ui_state.error_msg);
    });

    if clicked {
        let socket_addr = SocketAddr::from_str(&ui_state.socket_addr_s);
        if ui_state.username.is_empty() {
            ui_state.error_msg = "You must enter a username".to_owned();
        }

        match socket_addr {
            Ok(socket_addr) => {
                commands.insert_resource(ConnectionInformation {
                    socket_addr: Some(socket_addr),
                    username: ui_state.username.clone(),
                    room_password: ui_state.password.clone(),
                });
                commands.insert_resource(NextState(GameState::WaitingForMoreConnectionsMenu));
            }
            Err(e) => ui_state.error_msg = e.to_string(),
        }
    }
}
