use serenity::{
    framework::standard::macros::command,
    framework::standard::{Args, CommandResult},
    model::channel::Message,
    prelude::Context,
};

#[command]
pub async fn configure(_: &Context, _: &Message, _: Args) -> CommandResult {
    Ok(())
}
