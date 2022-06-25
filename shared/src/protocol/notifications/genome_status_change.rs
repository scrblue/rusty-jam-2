use bevy::prelude::Component;
use naia_shared::{derive_serde, serde, Property, Replicate};

use crate::components::genome::AnimalType;

#[derive(Copy, Debug, Eq, Hash)]
#[derive_serde]
pub enum LockedStatus {
    Unlocked,
    Locked,
}

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct GenomeStatusChange {
    pub species: Property<AnimalType>,
    pub status: Property<LockedStatus>,
}

impl GenomeStatusChange {
    pub fn new(species: AnimalType, status: LockedStatus) -> GenomeStatusChange {
        GenomeStatusChange::new_complete(species, status)
    }
}
