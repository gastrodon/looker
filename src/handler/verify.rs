use super::{EMOJI_CHECK, EMOJI_CROSS};
use crate::client_data::ServerConfig;
use serenity::{
    model::{
        channel::{Message, Reaction},
        guild::Member,
        id::RoleId,
        misc::Mention,
        permissions::Permissions,
    },
    prelude::Context,
    Error, Result,
};

pub async fn handle_verify(
    ctx: &Context,
    react: &Reaction,
    code: &str,
    config: ServerConfig,
) -> Result<()> {
    if !may_verify(ctx, react, config.verify.permissions_required).await? {
        return Ok(());
    }

    let message = react.message(&ctx.http).await?;
    if already_verified(ctx, &message, config.verify.verified_id).await? {
        return Ok(());
    };

    let author = match message.channel(&ctx.cache).await {
        Some(channel) => match channel.guild() {
            Some(guild_channel) => {
                guild_channel
                    .guild_id
                    .member(&ctx.http, &message.author)
                    .await?
            }
            None => return Err(Error::Other("channel isn't a guild_channel")),
        },
        None => return Err(Error::Other("message not in a channel")),
    };

    let result = match code {
        EMOJI_CHECK => do_verify(ctx, author, config).await,
        EMOJI_CROSS => no_verify(ctx, author, config).await,
        _ => unreachable!(),
    };

    match result {
        Ok(()) => message.delete(&ctx.http).await,
        Err(why) => Err(why),
    }
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

async fn do_verify(ctx: &Context, author: Member, config: ServerConfig) -> Result<()> {
    let mut author_mut = author;
    if let Some(role) = config.verify.verified_id {
        author_mut.add_role(&ctx.http, role).await?
    }

    if let Some(role) = config.verify.unverified_id {
        author_mut.remove_role(&ctx.http, role).await?;
    }

    if let Some(welcome) = config.channels.welcome {
        let mention = Mention::from(&author_mut);
        let message = match (config.channels.role, config.channels.introduction) {
            (Some(role), Some(intro)) => format!(
                    "Welcome to the server, {who}. Feel free to grab some roles in {role}, or introduce yourself in {intro}",
                who = mention,
                role = Mention::from(role),
                intro = Mention::from(intro),
            ),
            (Some(role), None) => format!(
                    "Welcome to the server, {who}. Feel free to grab some roles in {role}",
                who = mention,
                role = Mention::from(role),
            ),
            (None, Some(intro)) => format!(
                "Welcome to the server, {who}. Feel free to introduce yourself in {intro}",
                who = mention,
                intro = Mention::from(intro),
            ),
            (None, None) => format!("Welcome to the server, {who}.", who = mention,),
        };

        welcome.say(&ctx.http, message).await?;
    }

    Ok(())
}

async fn no_verify(ctx: &Context, author: Member, config: ServerConfig) -> Result<()> {
    author.kick(&ctx.http).await?;

    if let Some(log) = config.channels.log {
        log.say(
            &ctx.http,
            format!(
                "{who} was rejected and kicked",
                who = Mention::from(&author)
            ),
        )
        .await?;
    };

    Ok(())
}
