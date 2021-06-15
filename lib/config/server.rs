use serenity::{
    model::{
        guild::Member,
        id::{ChannelId, RoleId, UserId},
        permissions::Permissions,
    },
    prelude::TypeMapKey,
};
use std::{collections::HashMap, convert::Into, default::Default};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Quarantine {
    pub channel: ChannelId,
    pub candidate: UserId,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Channels {
    pub introduction: Option<ChannelId>,
    pub jail: Option<ChannelId>,
    pub log: Option<ChannelId>,
    pub role: Option<ChannelId>,
    pub verify: Option<ChannelId>,
    pub welcome: Option<ChannelId>,

    pub quarantine: HashMap<ChannelId, Quarantine>,
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

            quarantine: HashMap::new(),
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
pub struct Config {
    pub channels: Channels,
    pub kept_roles: HashMap<UserId, Vec<RoleId>>,
    pub verify: Verify,
}

impl Config {
    pub fn new() -> Self {
        Config {
            channels: Channels::new(),
            kept_roles: HashMap::new(),
            verify: Verify::new(None, None, None),
        }
    }

    pub fn have_channels(&mut self, channels: Channels) -> Channels {
        self.channels = channels.clone();
        channels
    }

    pub fn keep_roles(&mut self, who: Member) {
        self.kept_roles.insert(who.user.id, who.roles);
    }

    pub fn start_quarantine(&mut self, channel: ChannelId, candidate: UserId) {
        self.channels
            .quarantine
            .insert(channel, Quarantine { channel, candidate });
    }

    pub fn drop_quarantine(&mut self, channel: &ChannelId) {
        self.channels.quarantine.remove(channel);
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Table {
    cache: HashMap<u64, Config>,
}

impl Table {
    pub fn new() -> Self {
        Table {
            cache: HashMap::new(),
        }
    }

    pub fn get<T: Into<u64>>(&self, key: T) -> Option<&Config> {
        self.cache.get(&key.into())
    }

    pub fn set<T: Into<u64>, V: Into<Config>>(&mut self, key: T, value: V) -> Option<Config> {
        self.cache.insert(key.into(), value.into())
    }

    pub fn provision<T: Into<u64>>(&mut self, key: T) -> Option<Config> {
        self.cache.insert(key.into(), Config::new())
    }
}

pub struct Key;

impl TypeMapKey for Key {
    type Value = Table;
}
