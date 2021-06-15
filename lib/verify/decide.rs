use crate::config::server;
use serenity::{
    model::{guild::Member, misc::Mention},
    prelude::Context,
    Result,
};

pub async fn accept(ctx: &Context, author: Member, config: server::Config) -> Result<()> {
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

pub async fn reject(ctx: &Context, author: Member, config: server::Config) -> Result<()> {
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
