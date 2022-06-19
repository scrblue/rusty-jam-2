use naia_shared::{
    derive_channels, Channel, ChannelDirection, ChannelMode, ReliableSettings, TickBufferSettings,
};

#[derive_channels]
pub enum Channels {
    ClientKeepAlive,
    PlayerInput,

    WaitingOnPlayers,
    Countdown,
    GameNotification,
}

pub const CHANNEL_CONFIG: &[Channel<Channels>] = &[
    // Client-to-sever
    Channel {
        index: Channels::ClientKeepAlive,
        direction: ChannelDirection::ClientToServer,
        mode: ChannelMode::TickBuffered(TickBufferSettings::default()),
    },
    Channel {
        index: Channels::PlayerInput,
        direction: ChannelDirection::ClientToServer,
        mode: ChannelMode::OrderedReliable(ReliableSettings::default()),
    },
    // Server-to-client
    Channel {
        index: Channels::WaitingOnPlayers,
        direction: ChannelDirection::ServerToClient,
        mode: ChannelMode::OrderedReliable(ReliableSettings::default()),
    },
    Channel {
        index: Channels::Countdown,
        direction: ChannelDirection::ServerToClient,
        mode: ChannelMode::OrderedReliable(ReliableSettings::default()),
    },
    Channel {
		index: Channels::GameNotification,
		direction: ChannelDirection::ServerToClient,
		mode: ChannelMode::OrderedReliable(ReliableSettings::default()),
    }
];
