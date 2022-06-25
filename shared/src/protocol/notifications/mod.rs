use naia_shared::{derive_serde, serde};

use crate::components::players::PlayerId;

pub mod client_connected;
pub mod game_start;
pub mod turn_change;

// TODO: move to components mod (and probably rename components)
#[derive_serde]
pub enum WhoseTurn {
    Yours,
    Player(String, PlayerId),
}
