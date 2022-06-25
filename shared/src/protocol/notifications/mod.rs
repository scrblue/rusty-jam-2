use naia_shared::{derive_serde, serde};

use crate::components::players::PlayerId;

pub mod client_connected;
pub mod game_start;
pub mod turn_change;

// TODO: move to components mod (and probably rename components)
#[derive_serde]
pub enum WhoseTurn {
    Yours {
        turn_number: u16,
    },
    Player {
        username: String,
        id: PlayerId,
        turn_number: u16,
    },
}
