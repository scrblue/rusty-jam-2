//! A module for resource definitions that apply to multiple states. Resources that only apply to an
//! individual state are defined in that state's module.

use std::collections::HashMap;

use bevy::prelude::*;
use naia_bevy_server::{RoomKey, UserKey};

/// The [`RoomKey`] of the overworld map that every player is apart of.
pub struct MainRoom {
    pub key: RoomKey,
    pub map_entity: Entity,
}

/// A simple enum used with two-way associations
pub enum DeletedStatus {
    AssociatedNotFound,
    Deleted,
}

/// A two-way association between username [`Strings`] and [`UserKey`]s
pub struct UsernameKeyAssociation {
    name_to_key: HashMap<String, UserKey>,
    key_to_name: HashMap<UserKey, String>,
}
impl UsernameKeyAssociation {
    pub fn new() -> Self {
        UsernameKeyAssociation {
            name_to_key: HashMap::new(),
            key_to_name: HashMap::new(),
        }
    }

    pub fn insert(&mut self, name: String, key: UserKey) {
        self.name_to_key.insert(name.clone(), key);
        self.key_to_name.insert(key, name);
    }

    pub fn get_from_name(&self, name: &str) -> Option<&UserKey> {
        self.name_to_key.get(name)
    }

    pub fn get_from_key(&self, key: &UserKey) -> Option<&String> {
        self.key_to_name.get(key)
    }

    pub fn delete_from_name(&mut self, name: &str) -> DeletedStatus {
        if let Some(key) = self.name_to_key.remove(name) {
            self.key_to_name.remove(&key);
            DeletedStatus::Deleted
        } else {
            DeletedStatus::AssociatedNotFound
        }
    }

    pub fn delete_from_key(&mut self, key: &UserKey) -> DeletedStatus {
        if let Some(name) = self.key_to_name.remove(key) {
            self.name_to_key.remove(&name);
            DeletedStatus::Deleted
        } else {
            DeletedStatus::AssociatedNotFound
        }
    }
}

/// A two-way association between [`UserKey`]s and the [`Entity`]s represeting the entire perceived
/// map for a player
pub struct KeyMapAssociation {
    key_to_map_entity: HashMap<UserKey, Entity>,
    map_entity_to_key: HashMap<Entity, UserKey>,
}
impl KeyMapAssociation {
    pub fn new() -> Self {
        KeyMapAssociation {
            key_to_map_entity: HashMap::new(),
            map_entity_to_key: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: UserKey, entity: Entity) {
        self.key_to_map_entity.insert(key.clone(), entity);
        self.map_entity_to_key.insert(entity, key);
    }

    pub fn get_from_key(&self, key: &UserKey) -> Option<&Entity> {
        self.key_to_map_entity.get(key)
    }

    pub fn get_from_entity(&self, entity: &Entity) -> Option<&UserKey> {
        self.map_entity_to_key.get(entity)
    }

    pub fn delete_from_key(&mut self, key: &UserKey) -> DeletedStatus {
        if let Some(entity) = self.key_to_map_entity.remove(key) {
            self.map_entity_to_key.remove(&entity);
            DeletedStatus::Deleted
        } else {
            DeletedStatus::AssociatedNotFound
        }
    }

    pub fn delete_from_entity(&mut self, entity: &Entity) -> DeletedStatus {
        if let Some(key) = self.map_entity_to_key.remove(entity) {
            self.key_to_map_entity.remove(&key);
            DeletedStatus::Deleted
        } else {
            DeletedStatus::AssociatedNotFound
        }
    }
}

pub struct MapAsset(pub HandleUntyped);
