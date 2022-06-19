use std::net::SocketAddr;

use bevy::prelude::*;

pub mod connect_menu;
pub mod countdown_menu;
pub mod game;
pub mod input;
pub mod waiting_for_more_connections_menu;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum GameState {
    ConnectMenu,
    WaitingForMoreConnectionsMenu,
    CountdownMenu,
    Game,
}

#[derive(Default)]
pub struct ConnectionInformation {
    pub socket_addr: Option<SocketAddr>,
    pub username: String,
    pub room_password: String,
}

pub struct TileSprites {
    pub beach: Handle<Image>,
    pub clear_sky: Handle<Image>,
    pub desert: Handle<Image>,
    pub fog: Handle<Image>,
    pub forest: Handle<Image>,
    pub grass: Handle<Image>,
    pub island: Handle<Image>,
    pub oasis: Handle<Image>,
    pub ocean: Handle<Image>,
    pub stormy_sky: Handle<Image>,
    pub windy_sky: Handle<Image>,
}
