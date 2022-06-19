use naia_shared::Protocolize;

pub mod identification;
pub use identification::Identification;

pub mod client_keep_alive;
pub use client_keep_alive::ClientKeepAlive;

pub mod waiting_on_players;
pub use waiting_on_players::WaitingOnPlayers;

pub mod countdown;
pub use countdown::Countdown;

pub mod map_sync;
pub use map_sync::MapSync;

#[derive(Protocolize)]
pub enum Protocol {
    Identification(Identification),
    ClientKeepAlive(ClientKeepAlive),

    WaitingOnPlayers(WaitingOnPlayers),
    Countdown(Countdown),
    MapSync(MapSync),
}
