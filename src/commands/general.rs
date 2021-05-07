use super::UNVERIFIED_COMMAND;
use serenity::{
    framework::standard::macros::{command, group},
    framework::standard::{Args, CommandResult},
    model::channel::Message,
    prelude::Context,
};

#[group]
#[commands(about, unverified)]
struct General;

#[command]
async fn about(context: &Context, message: &Message, _: Args) -> CommandResult {
    message
        .channel_id
        .say(&context.http, "Yes I am alive")
        .await?;
    Ok(())
}
