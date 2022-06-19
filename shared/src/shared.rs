use std::time::Duration;

use naia_shared::{SharedConfig, SocketConfig};

use crate::{Channels, CHANNEL_CONFIG};

#[cfg(debug_assertions)]
use naia_shared::LinkConditionerConfig;

pub fn shared_config() -> SharedConfig<Channels> {
    let tick_interval = Some(Duration::from_millis(20));

    #[cfg(debug_assertions)]
    let link_condition = Some(LinkConditionerConfig::average_condition());
    #[cfg(not(debug_assertions))]
    let link_condition = None;

    SharedConfig::new(
        SocketConfig::new(link_condition, None),
        CHANNEL_CONFIG,
        tick_interval,
        None,
    )
}
