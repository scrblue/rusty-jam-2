//! A module for resource definitions that apply to multiple states. Resources that only apply to an
//! individual state are defined in that state's module.

use std::collections::HashMap;

use naia_bevy_server::{RoomKey, UserKey};

/// The [`RoomKey`] of the overworld map that every player is apart of.
pub struct MainRoomKey(pub RoomKey);

/// A simple enum used with two-way associations
pub enum DeletedStatus {
    AssociatedNotFound,
    Deleted,
}

/// A two-way association between username [`strings`] and [`UserKey`]s
pub struct UsernameKeyAssocaiation {
    name_to_key: HashMap<String, UserKey>,
    key_to_name: HashMap<UserKey, String>,
}
impl UsernameKeyAssocaiation {
    pub fn new() -> Self {
        UsernameKeyAssocaiation {
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
