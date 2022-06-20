use naia_shared::Protocolize;

pub mod identification;
pub use identification::Identification;

pub mod client_keep_alive;
pub use client_keep_alive::ClientKeepAlive;

pub mod player_input;
pub use player_input::PlayerInput;

pub mod waiting_on_players;
pub use waiting_on_players::WaitingOnPlayers;

pub mod countdown;
pub use countdown::Countdown;

pub mod notifications;
pub use notifications::{game_start::GameStartNotification, turn_change::TurnChangeNotification};

pub mod map_sync;
pub use map_sync::MapSync;

pub mod unit_sync;
pub use unit_sync::UnitSync;

#[derive(Protocolize)]
pub enum Protocol {
    Identification(Identification),
    ClientKeepAlive(ClientKeepAlive),
    PlayerInput(PlayerInput),

    WaitingOnPlayers(WaitingOnPlayers),
    Countdown(Countdown),

    GameStartNotification(GameStartNotification),
    TurnChangeNotification(TurnChangeNotification),

    MapSync(MapSync),
    UnitSync(UnitSync),
}
