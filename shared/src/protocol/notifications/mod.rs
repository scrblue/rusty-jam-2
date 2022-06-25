use naia_shared::{derive_serde, serde};

use crate::components::players::PlayerId;

pub mod client_connected;
pub mod game_start;
pub mod genome_status_change;
pub mod turn_change;

// TODO: move to components mod (and probably rename components)
#[derive(Debug)]
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

impl WhoseTurn {
    pub fn turn_number(&self) -> u16 {
        match self {
            WhoseTurn::Yours { turn_number } => *turn_number,
            WhoseTurn::Player { turn_number, .. } => *turn_number,
        }
    }
}
