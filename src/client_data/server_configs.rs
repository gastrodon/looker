use serenity::{model::id::ChannelId, prelude::TypeMapKey};
use std::{collections::HashMap, convert::Into, default::Default};

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Channels {
    pub introduction: Option<ChannelId>,
    pub log: Option<ChannelId>,
    pub role: Option<ChannelId>,
    pub welcome: Option<ChannelId>,
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct ServerConfig {
    channels: Option<Channels>,
}

impl ServerConfig {
    pub fn new() -> Self {
        ServerConfig { channels: None }
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct ServerConfigTable {
    cache: HashMap<u64, ServerConfig>,
}

impl ServerConfigTable {
    pub fn new() -> Self {
        ServerConfigTable {
            cache: HashMap::new(),
        }
    }

    pub fn get<'a, T: Into<&'a u64>>(&self, key: T) -> Option<&ServerConfig> {
        self.cache.get(key.into())
    }

    pub fn set<'a, T: Into<&'a u64>, V: Into<ServerConfig>>(
        &mut self,
        key: T,
        value: V,
    ) -> Option<ServerConfig> {
        self.cache.insert(*key.into(), value.into())
    }

    pub fn provision<'a, T: Into<&'a u64>>(&mut self, key: T) -> Option<ServerConfig> {
        self.cache.insert(*key.into(), ServerConfig::new())
    }
}

impl Default for ServerConfigTable {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ServerConfigKey;

impl TypeMapKey for ServerConfigKey {
    type Value = ServerConfigTable;
}
