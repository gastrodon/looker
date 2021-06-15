use super::{EMOJI_CHECK, EMOJI_CROSS, EMOJI_QUESTION};
use crate::{
    client_data::{ServerConfig, ServerConfigKey},
    config_for,
};
use serenity::{
    futures::StreamExt,
    model::{
        channel::{Embed, Message, PermissionOverwrite, PermissionOverwriteType, Reaction},
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

    match code {
        EMOJI_CHECK => match do_verify(ctx, author, config).await {
            Ok(_) => message.delete(&ctx.http).await,
            Err(why) => Err(why),
        },
        EMOJI_CROSS => match no_verify(ctx, author, config).await {
            Ok(_) => message.delete(&ctx.http).await,
            Err(why) => Err(why),
        },
        EMOJI_QUESTION => do_quarantine(ctx, author, config).await,
        _ => unreachable!(),
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
                intro = Mention::from(intro)
            ),
            (Some(role), None) => format!(
                    "Welcome to the server, {who}. Feel free to grab some roles in {role}",
                who = mention,
                role = Mention::from(role)
            ),
            (None, Some(intro)) => format!(
                "Welcome to the server, {who}. Feel free to introduce yourself in {intro}",
                who = mention,
                intro = Mention::from(intro)
            ),
            (None, None) => format!("Welcome to the server, {who}.", who = mention),
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

async fn do_quarantine(ctx: &Context, author: Member, config: ServerConfig) -> Result<()> {
    let name = format!(
        "quarantine-{}-{}",
        author.user.name, author.user.discriminator
    );

    let channel = author
        .guild_id
        .create_channel(&ctx.http, |it| {
            it.name(&name).permissions(vec![PermissionOverwrite {
                allow: Permissions::empty(),
                deny: Permissions::READ_MESSAGES | Permissions::SEND_MESSAGES,
                kind: PermissionOverwriteType::Role((*author.guild_id.as_u64()).into()),
            }])
        })
        .await?;

    {
        let id = author.guild_id;

        let mut handle = ctx.data.write().await;
        let mut config = config_for!(id, handle);
        config.start_quarantine(channel.id);

        let mut table = handle.get::<ServerConfigKey>().unwrap().clone();
        table.set(id, config);
        handle.insert::<ServerConfigKey>(table);
    }

    channel
        .create_permission(
            &ctx.http,
            &PermissionOverwrite {
                allow: Permissions::READ_MESSAGES | Permissions::SEND_MESSAGES,
                deny: Permissions::empty(),
                kind: PermissionOverwriteType::Member(author.user.id),
            },
        )
        .await
        .ok();

    channel
        .say(
            &ctx.http,
            format!(
                "Hello {who}, we've moved you in here to chat before you're verified",
                who = Mention::from(author.user.id)
            ),
        )
        .await
        .ok();

    if let Some(channel_id) = config.channels.verify {
        let mut buffer = Vec::<Message>::new();
        let mut handle = channel_id.messages_iter(&ctx.http).boxed();

        while let Some(next) = handle.next().await {
            if let Ok(message) = next {
                if message.author.id == author.user.id {
                    buffer.push(message)
                };
            };
        }

        let webhook = channel.create_webhook(&ctx.http, &name).await?;
        let avatar_url = author
            .user
            .avatar_url()
            .unwrap_or_else(|| author.user.default_avatar_url());

        for message in buffer.iter().rev() {
            webhook
                .execute(&ctx.http, true, |it| {
                    it.embeds(vec![Embed::fake(|embed| {
                        embed.description(&message.content)
                    })])
                    .avatar_url(&avatar_url)
                    .username(&author.user.name)
                })
                .await
                .ok();

            message.delete(&ctx.http).await.ok();
        }
    };

    Ok(())
}
