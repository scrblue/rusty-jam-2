use naia_shared::{
    derive_channels, Channel, ChannelDirection, ChannelMode, ReliableSettings, TickBufferSettings,
};

#[derive_channels]
pub enum Channels {
    ClientKeepAlive,

    WaitingOnPlayers,
    Countdown,
    MapSync,
}

pub const CHANNEL_CONFIG: &[Channel<Channels>] = &[
    Channel {
        index: Channels::ClientKeepAlive,
        direction: ChannelDirection::ClientToServer,
        mode: ChannelMode::TickBuffered(TickBufferSettings::default()),
    },
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
];
