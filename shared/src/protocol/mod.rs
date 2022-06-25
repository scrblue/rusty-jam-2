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

pub mod game_sync;
pub use game_sync::{map_sync::MapSync, unit_sync::UnitSync};

pub mod chat;
pub use chat::{receive_chat::ReceiveChat, send_chat::SendChat};

#[derive(Protocolize)]
pub enum Protocol {
    Identification(Identification),
    ClientKeepAlive(ClientKeepAlive),
    SendChat(SendChat),
    PlayerInput(PlayerInput),

    WaitingOnPlayers(WaitingOnPlayers),
    Countdown(Countdown),

	ReceiveChat(ReceiveChat),

    GameStartNotification(GameStartNotification),
    TurnChangeNotification(TurnChangeNotification),

    MapSync(MapSync),
    UnitSync(UnitSync),
}
