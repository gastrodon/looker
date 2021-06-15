use crate::{config_for, maybe};
use lib::config::server;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::{channel::Message, id::ChannelId},
    prelude::Context,
    Result,
};

async fn gate(ctx: &Context, message: &Message) -> Result<bool> {
    let channel_id = message.channel_id;
    let config = match channel_id.to_channel(&ctx.http).await?.guild() {
        Some(guild) => config_for!(guild.id, ctx.data.read().await),
        None => return Ok(false),
    };

    Ok(config.channels.quarantine.contains(&channel_id))
}

async fn cleanup(ctx: &Context, channel_id: ChannelId) -> Result<()> {
    match channel_id.delete(&ctx.http).await {
        Ok(_) => Ok(()),
        Err(why) => Err(why),
    }
}

#[command]
pub async fn accept(ctx: &Context, message: &Message, _: Args) -> CommandResult {
    if !maybe!(gate(ctx, message).await, Result) {
        return Ok(());
    }

    maybe!(cleanup(ctx, message.channel_id).await, Result);
    Ok(())
}

#[command]
pub async fn deny(ctx: &Context, message: &Message, _: Args) -> CommandResult {
    if !maybe!(gate(ctx, message).await, Result) {
        return Ok(());
    }

    maybe!(cleanup(ctx, message.channel_id).await, Result);
    Ok(())
}
