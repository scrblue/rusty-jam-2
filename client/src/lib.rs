use std::net::SocketAddr;

pub mod connect_menu;
pub mod countdown_menu;
pub mod game;
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
