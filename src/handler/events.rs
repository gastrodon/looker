use crate::client_data::{ChannelsKey, Verify, VerifyKey};
use serenity::{
    async_trait,
    model::channel::{Message, Reaction, ReactionType},
    model::guild::Member,
    model::misc::Mention,
    prelude::{Context, EventHandler},
    Error, Result,
};

const EMOJI_CHECK: &str = "\u{2705}";
const EMOJI_CROSS: &str = "\u{274C}";

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn reaction_add(&self, ctx: Context, react: Reaction) {
        let code = match &react.emoji {
            ReactionType::Unicode(code) => code.as_str(),
            _ => return,
        };

        let readable = &ctx.data.read().await;

        let result = match code {
            EMOJI_CHECK | EMOJI_CROSS => {
                let verify = *readable.get::<VerifyKey>().unwrap();
                if react.channel_id == verify.channel_id {
                    handle_verify(&ctx, &react, code, verify).await
                } else {
                    Ok(())
                }
            }
            _ => Ok(()),
        };

        if let Err(why) = result {
            println!("{:?}", why);
        }
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

async fn handle_verify(ctx: &Context, react: &Reaction, code: &str, verify: Verify) -> Result<()> {
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

    match code {
        EMOJI_CHECK => do_verify(ctx, message, verify).await,
        EMOJI_CROSS => no_verify(ctx, message, verify).await,
        _ => unreachable!(),
    }
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

async fn do_verify(ctx: &Context, message: Message, verify: Verify) -> Result<()> {
    let mut author = match message.channel(&ctx.cache).await {
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

    author.add_role(&ctx.http, verify.role_id).await?;
    message.delete(&ctx.http).await?;
    welcome(ctx, &author).await?;
    Ok(())
}

async fn no_verify(ctx: &Context, message: Message, _: Verify) -> Result<()> {
    message
        .channel_id
        .say(&ctx.http, "Should be not verified")
        .await?;
    Ok(())
}
