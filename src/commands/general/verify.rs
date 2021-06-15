use lib::{config::server, config_for, maybe, verify};
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::Context,
};

enum Kind {
    Accept,
    Reject,
}

async fn handle(kind: Kind, ctx: &Context, message: &Message) -> CommandResult {
    let guild_id = maybe!(message.guild_id, Option);
    let config = config_for!(guild_id, ctx.data.read().await);
    let member = match config.channels.quarantine.get(&message.channel_id) {
        Some(description) => maybe!(
            guild_id.member(&ctx.http, description.candidate).await,
            Result
        ),
        None => return Ok(()),
    };

    maybe!(
        match kind {
            Kind::Accept => verify::accept(ctx, member, config).await,
            Kind::Reject => verify::reject(ctx, member, config).await,
        },
        Result
    );

    maybe!(message.channel_id.delete(&ctx.http).await, Result);
    Ok(())
}

#[command]
pub async fn accept(ctx: &Context, message: &Message, _: Args) -> CommandResult {
    handle(Kind::Accept, ctx, message).await
}

#[command]
#[aliases("deny")]
pub async fn reject(ctx: &Context, message: &Message, _: Args) -> CommandResult {
    handle(Kind::Reject, ctx, message).await
}
