use serenity::{
    framework::standard::macros::command,
    framework::standard::{Args, CommandResult},
    model::channel::Message,
    prelude::Context,
};

#[command]
pub async fn about(context: &Context, message: &Message, _: Args) -> CommandResult {
    message
        .channel_id
        .say(&context.http, "Yes I am alive")
        .await?;
    Ok(())
}
