use serenity::{
    model::{
        guild::Member,
        id::{ChannelId, RoleId, UserId},
        permissions::Permissions,
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

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Verify {
    pub verified_id: Option<RoleId>,
    pub unverified_id: Option<RoleId>,
    pub permissions_required: Permissions,
}

impl Verify {
    pub fn new(
        verified: Option<RoleId>,
        unverified: Option<RoleId>,
        permissions: Option<Permissions>,
    ) -> Self {
        #[rustfmt::skip]
        let default_permission =
                  Permissions::MANAGE_ROLES
                | Permissions::MANAGE_MESSAGES
                | Permissions::KICK_MEMBERS;

        Verify {
            verified_id: verified,
            unverified_id: unverified,
            permissions_required: permissions.unwrap_or(default_permission),
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ServerConfig {
    pub channels: Channels,
    pub kept_roles: HashMap<UserId, Vec<RoleId>>,
    pub verify: Verify,
}

impl ServerConfig {
    pub fn new() -> Self {
        ServerConfig {
            channels: Channels::new(),
            kept_roles: HashMap::new(),
            verify: Verify::new(None, None, None),
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
