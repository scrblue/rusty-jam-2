use std::time::Duration;

use naia_bevy_server::UserKey;

pub struct TurnTracker {
	pub player: UserKey,
	pub turn_number: u16,
	pub time_left: Option<Duration>,
}
