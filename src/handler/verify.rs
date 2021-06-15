use super::{EMOJI_CHECK, EMOJI_CROSS, EMOJI_QUESTION};
use lib::{
    config::server,
    verify::{accept, open_quarantine, reject},
};
use serenity::{
    futures::StreamExt,
    model::{
        channel::{Message, Reaction},
        id::{ChannelId, RoleId, UserId},
        permissions::Permissions,
    },
    prelude::Context,
    Error, Result,
};

pub async fn handle_verify(
    ctx: &Context,
    react: &Reaction,
    code: &str,
    config: server::Config,
) -> Result<()> {
    if !may_verify(ctx, react, config.verify.permissions_required).await? {
        return Ok(());
    }

    let message = react.message(&ctx.http).await?;
    if already_verified(ctx, &message, config.verify.verified_id).await? {
        return Ok(());
    };

    let channel = match message.channel(&ctx.cache).await {
        Some(channel) => channel,
        None => return Err(Error::Other("message not in a channel")),
    };

    let author = match channel.clone().guild() {
        Some(guild_channel) => {
            guild_channel
                .guild_id
                .member(&ctx.http, &message.author)
                .await?
        }
        None => return Err(Error::Other("channel isn't a guild_channel")),
    };

    if let Err(why) = match code {
        EMOJI_CHECK => accept(ctx, author.clone(), config).await,
        EMOJI_CROSS => reject(ctx, author.clone(), config).await,
        EMOJI_QUESTION => open_quarantine(ctx, author.clone(), config).await,
        _ => unreachable!(),
    } {
        return Err(why);
    };

    purge_messages(ctx, author.user.id, channel.id()).await
}

async fn already_verified(
    ctx: &Context,
    message: &Message,
    role_id: Option<RoleId>,
) -> Result<bool> {
    let guild_id = match message.channel(&ctx.cache).await {
        Some(channel) => match channel.guild() {
            Some(guild_channel) => guild_channel.guild_id,
            None => return Err(Error::Other("channel isn't a guild_channel")),
        },
        None => return Err(Error::Other("message not in a channel")),
    };

    match role_id {
        Some(role_id) => message.author.has_role(&ctx.http, guild_id, role_id).await,
        None => Ok(false),
    }
}

async fn may_verify(ctx: &Context, react: &Reaction, permissions: Permissions) -> Result<bool> {
    let member = match react.guild_id {
        Some(guild_id) => {
            guild_id
                .member(&ctx.http, react.user(&ctx.http).await?)
                .await?
        }
        None => return Err(Error::Other("react not in a guild")),
    };

    Ok(member.permissions(&ctx).await?.contains(permissions))
}

async fn purge_messages(ctx: &Context, author_id: UserId, channel_id: ChannelId) -> Result<()> {
    let mut handle = channel_id.messages_iter(&ctx.http).boxed();

    while let Some(next) = handle.next().await {
        if let Ok(message) = next {
            if message.author.id == author_id {
                if let Err(why) = message.delete(&ctx.http).await {
                    return Err(why);
                }
            };
        };
    }

    Ok(())
}
