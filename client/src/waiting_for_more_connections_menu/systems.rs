use std::collections::HashMap;

use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use iyes_loopless::state::NextState;
use naia_bevy_client::{
    events::{InsertComponentEvent, MessageEvent, SpawnEntityEvent, UpdateComponentEvent},
    Client,
};

use crate::{
    game::resources::Map, waiting_for_more_connections_menu::resources::WaitingFor,
    ConnectionInformation, GameState, TileSprites, UnitSprites,
};
use rgj_shared::{
    protocol::{ClientKeepAlive, Identification, Protocol, ProtocolKind},
    Channels,
};

pub fn init(
    mut commands: Commands,
    mut client: Client<Protocol, Channels>,

    mut conn_info: ResMut<ConnectionInformation>,
    assets: Res<AssetServer>,
) {
    client.auth(Identification::new_complete(
        std::mem::take(&mut conn_info.username),
        std::mem::take(&mut conn_info.room_password),
    ));
    client.connect(&format!("http://{}", conn_info.socket_addr.unwrap()));

    commands.insert_resource(WaitingFor(0));
    commands.insert_resource(Map {
        coords_to_tile: HashMap::new(),
        coords_to_unit: HashMap::new(),
    });

    let beach = assets.load("tiles/BeachHex.png");
    let clear_sky = assets.load("tiles/ClearSkyHex.png");
    let desert = assets.load("tiles/DesertHex.png");
    let fog = assets.load("tiles/FogHex.png");
    let forest = assets.load("tiles/ForestHex.png");
    let grass = assets.load("tiles/GrassHex.png");
    let island = assets.load("tiles/IslandHex.png");
    let oasis = assets.load("tiles/OasisHex.png");
    let ocean = assets.load("tiles/OceanHex.png");
    let stormy_sky = assets.load("tiles/StormySkyHex.png");
    let windy_sky = assets.load("tiles/WindySkyHex.png");

    commands.insert_resource(TileSprites {
        beach,
        clear_sky,
        desert,
        fog,
        forest,
        grass,
        island,
        oasis,
        ocean,
        stormy_sky,
        windy_sky,
    });

    let bg_red = assets.load("unit_backgrounds/RedTeam.png");
    let bg_orange = assets.load("unit_backgrounds/OrangeTeam.png");
    let bg_yellow = assets.load("unit_backgrounds/YellowTeam.png");
    let bg_green = assets.load("unit_backgrounds/GreenTeam.png");
    let bg_blue = assets.load("unit_backgrounds/BlueTeam.png");
    let bg_purple = assets.load("unit_backgrounds/PurpleTeam.png");

    let fg_bat = assets.load("unit_foregrounds/Bat.png");
    let fg_chicken = assets.load("unit_foregrounds/Chicken.png");
    let fg_deer = assets.load("unit_foregrounds/Deer.png");
    let fg_eel = assets.load("unit_foregrounds/Eel.png");
    let fg_elephant = assets.load("unit_foregrounds/Elephant.png");
    let fg_rattlesnake = assets.load("unit_foregrounds/Rattlesnake.png");
    let fg_sailfish = assets.load("unit_foregrounds/Sailfish.png");
    let fg_vulture = assets.load("unit_foregrounds/Vulture.png");
    let fg_whale = assets.load("unit_foregrounds/Whale.png");

    commands.insert_resource(UnitSprites {
        bg_red,
        bg_orange,
        bg_yellow,
        bg_green,
        bg_blue,
        bg_purple,
        fg_bat,
        fg_chicken,
        fg_deer,
        fg_eel,
        fg_elephant,
        fg_rattlesnake,
        fg_sailfish,
        fg_vulture,
        fg_whale,
    });
}

pub fn connection_event(client: Client<Protocol, Channels>) {
    info!("Client connected to: {}", client.server_address());
}

pub fn disconnection_event(client: Client<Protocol, Channels>) {
    info!("Client disconnected from: {}", client.server_address());
}

pub fn spawn_entity_event(mut event_reader: EventReader<SpawnEntityEvent>) {
    for SpawnEntityEvent(entity) in event_reader.iter() {
        info!("Spawned entity {:?}", entity);
    }
}

pub fn insert_component_event(mut event_reader: EventReader<InsertComponentEvent<ProtocolKind>>) {
    for InsertComponentEvent(..) in event_reader.iter() {}
}

pub fn update_component_event(mut event_reader: EventReader<UpdateComponentEvent<ProtocolKind>>) {
    for UpdateComponentEvent(..) in event_reader.iter() {}
}

pub fn receive_waiting_on_players_message(
    mut event_reader: EventReader<MessageEvent<Protocol, Channels>>,
    mut waiting_for: ResMut<WaitingFor>,
) {
    for event in event_reader.iter() {
        if let MessageEvent(Channels::WaitingOnPlayers, Protocol::WaitingOnPlayers(waiting)) = event
        {
            waiting_for.0 = *waiting.num_waiting_for;
        }
    }
}

pub fn receive_countdown_message(
    mut event_reader: EventReader<MessageEvent<Protocol, Channels>>,
    mut commands: Commands,
) {
    for event in event_reader.iter() {
        if let MessageEvent(Channels::Countdown, Protocol::Countdown(cd)) = event {
            commands.insert_resource(crate::countdown_menu::resources::SecondsLeft(*cd.secs_left));
            commands.insert_resource(NextState(GameState::CountdownMenu));
        }
    }
}

pub fn waiting_for_more_connections_menu(
    waiting_for: Res<WaitingFor>,
    mut egui_context: ResMut<EguiContext>,
) {
    let label = format!("Waiting on {} players to connect", waiting_for.0);

    egui::Window::new("Waiting for other players").show(egui_context.ctx_mut(), |ui| {
        ui.label(label);
    });
}

pub fn tick(mut client: Client<Protocol, Channels>) {
    client.send_message(Channels::ClientKeepAlive, &ClientKeepAlive);
}
