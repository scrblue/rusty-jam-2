pub mod behavior;
pub mod resources;
pub mod protocol;

mod channels;
pub use channels::{Channels, CHANNEL_CONFIG};

mod shared;
pub use shared::shared_config;
