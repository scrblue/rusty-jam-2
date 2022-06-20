use naia_shared::{derive_serde, serde};

pub mod game_start;
pub mod turn_change;

#[derive_serde]
pub enum WhoseTurn {
    Yours,
    Player(String),
}
