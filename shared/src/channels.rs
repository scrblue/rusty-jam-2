use naia_shared::{
    derive_channels, Channel, ChannelDirection, ChannelMode, ReliableSettings, TickBufferSettings,
};

#[derive_channels]
pub enum Channels {
    WaitingOnPlayers,
    Countdown,
    MapSync,
}

pub const CHANNEL_CONFIG: &[Channel<Channels>] = &[
	Channel{
    	index: Channels::WaitingOnPlayers,
    	direction: ChannelDirection::ServerToClient,
    	mode: ChannelMode::TickBuffered(TickBufferSettings::default()),
	},
    Channel {
        index: Channels::Countdown,
        direction: ChannelDirection::ServerToClient,
        mode: ChannelMode::TickBuffered(TickBufferSettings::default()),
    },
    Channel {
        index: Channels::MapSync,
        direction: ChannelDirection::ServerToClient,
        mode: ChannelMode::UnorderedReliable(ReliableSettings::default()),
    },
];
