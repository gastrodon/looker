use serenity::{
    model::id::{ChannelId, RoleId},
    model::permissions::Permissions,
    prelude::TypeMapKey,
};

#[derive(Clone, Copy)]
pub struct Verify {
    pub channel_id: ChannelId,
    pub role_id: RoleId,
    pub permissions: Permissions,
}

pub struct VerifyKey;

impl TypeMapKey for VerifyKey {
    type Value = Verify;
}
