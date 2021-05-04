use super::{EMOJI_CHECK, EMOJI_CROSS};
use crate::client_data::{ChannelsKey, Verify};
use serenity::{
    model::channel::{Message, Reaction},
    model::guild::Member,
    model::misc::Mention,
    prelude::Context,
    Error, Result,
};

pub async fn handle_verify(
    ctx: &Context,
    react: &Reaction,
    code: &str,
    verify: Verify,
) -> Result<()> {
    let message = react.message(&ctx.http).await?;
    let channel = message.channel_id;

    if !may_verify(ctx, react, verify).await? {
        channel.say(&ctx.http, "Not allowed").await?;
        return Ok(());
    }

    if already_verified(ctx, &message, verify).await? {
        channel.say(&ctx.http, "Already verify").await?;
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
        EMOJI_CHECK => do_verify(ctx, author, verify).await,
        EMOJI_CROSS => no_verify(ctx, author, verify).await,
        _ => unreachable!(),
    };

    match result {
        Ok(()) => message.delete(&ctx.http).await,
        Err(why) => Err(why),
    }
}

async fn already_verified(ctx: &Context, message: &Message, verify: Verify) -> Result<bool> {
    let guild_id = match message.channel(&ctx.cache).await {
        Some(channel) => match channel.guild() {
            Some(guild_channel) => guild_channel.guild_id,
            None => return Err(Error::Other("channel isn't a guild_channel")),
        },
        None => return Err(Error::Other("message not in a channel")),
    };

    Ok(message
        .author
        .has_role(&ctx.http, guild_id, verify.role_id)
        .await?)
}

async fn may_verify(ctx: &Context, react: &Reaction, verify: Verify) -> Result<bool> {
    let member = match react.guild_id {
        Some(guild_id) => {
            guild_id
                .member(&ctx.http, react.user(&ctx.http).await?)
                .await?
        }
        None => return Err(Error::Other("react not in a guild")),
    };

    Ok(member.permissions(&ctx).await?.contains(verify.permissions))
}

async fn welcome(ctx: &Context, member: &Member) -> Result<()> {
    let readable = ctx.data.read().await;
    let channels = readable.get::<ChannelsKey>().unwrap();

    channels
        .welcome
        .say(
            &ctx.http,
            format!(
                "Welcome to the server {who}. Feel free to add roles in {roles}, and introduce yourself in {introductions}. Don't forget to familiaraize yourself with the rules, and enjoy the server!",
                who = Mention::from(member),
                roles = Mention::from(channels.roles),
                introductions = Mention::from(channels.introductions),
            ),
        )
        .await?;

    Ok(())
}

async fn do_verify(ctx: &Context, author: Member, verify: Verify) -> Result<()> {
    let mut author_mut = author;
    author_mut.add_role(&ctx.http, verify.role_id).await?;
    welcome(ctx, &author_mut).await?;
    Ok(())
}

async fn no_verify(_: &Context, _: Member, _: Verify) -> Result<()> {
    Ok(())
}