use serenity::{model::id::ChannelId, prelude::TypeMapKey};

#[derive(Clone, Copy)]
pub struct Channels {
    pub introductions: ChannelId,
    pub roles: ChannelId,
    pub welcome: ChannelId,
}

pub struct ChannelsKey;

impl TypeMapKey for ChannelsKey {
    type Value = Channels;
}
