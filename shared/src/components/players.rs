use bevy::prelude::*;
use naia_shared::{derive_serde, serde};

pub const MAX_NUM_PLAYERS: usize = 6;

#[derive_serde]
pub enum PlayerId {
	Red,
	Orange,
	Yellow,
	Green,
	Blue,
	Purple,
}

// TODO: Colorblind mode -- in sprites too
impl From<PlayerId> for Color {
	fn from(id: PlayerId) -> Self {
		match id {
			PlayerId::Red => Color::rgb_u8(175, 0, 0),
			PlayerId::Orange => Color::rgb_u8(175, 70, 0),
			PlayerId::Yellow => Color::rgb_u8(175, 160, 0),
			PlayerId::Green => Color::rgb_u8(37, 175,0),
			PlayerId::Blue => Color::rgb_u8(0, 112,175),
			PlayerId::Purple => Color::rgb_u8(64, 0, 175),
		}
	}
}
