use serenity::{
    model::{
        guild::Member,
        id::{ChannelId, RoleId, UserId},
    },
    prelude::TypeMapKey,
};
use std::{collections::HashMap, convert::Into, default::Default};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Channels {
    pub introduction: Option<ChannelId>,
    pub jail: Option<ChannelId>,
    pub log: Option<ChannelId>,
    pub role: Option<ChannelId>,
    pub verify: Option<ChannelId>,
    pub welcome: Option<ChannelId>,
}

impl Channels {
    pub fn new() -> Self {
        Channels {
            introduction: None,
            jail: None,
            log: None,
            role: None,
            verify: None,
            welcome: None,
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ServerConfig {
    pub channels: Channels,
    pub kept_roles: HashMap<UserId, Vec<RoleId>>,
}

impl ServerConfig {
    pub fn new() -> Self {
        ServerConfig {
            channels: Channels::new(),
            kept_roles: HashMap::new(),
        }
    }

    pub fn have_channels(&mut self, channels: Channels) -> Channels {
        self.channels = channels;
        channels
    }

    pub fn keep_roles(&mut self, who: Member) {
        self.kept_roles.insert(who.user.id, who.roles);
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ServerConfigTable {
    cache: HashMap<u64, ServerConfig>,
}

impl ServerConfigTable {
    pub fn new() -> Self {
        ServerConfigTable {
            cache: HashMap::new(),
        }
    }

    pub fn get<T: Into<u64>>(&self, key: T) -> Option<&ServerConfig> {
        self.cache.get(&key.into())
    }

    pub fn set<T: Into<u64>, V: Into<ServerConfig>>(
        &mut self,
        key: T,
        value: V,
    ) -> Option<ServerConfig> {
        self.cache.insert(key.into(), value.into())
    }

    pub fn provision<T: Into<u64>>(&mut self, key: T) -> Option<ServerConfig> {
        self.cache.insert(key.into(), ServerConfig::new())
    }
}

pub struct ServerConfigKey;

impl TypeMapKey for ServerConfigKey {
    type Value = ServerConfigTable;
}
