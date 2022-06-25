pub mod behavior;
pub mod components;
pub mod protocol;
pub mod resources;

mod channels;
pub use channels::{Channels, CHANNEL_CONFIG};

mod shared;
pub use shared::shared_config;
