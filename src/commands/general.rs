use serenity::{framework::standard, framework::standard::macros, model::channel, prelude};

#[macros::group]
#[commands(about)]
struct General;

#[macros::command]
async fn about(
    context: &prelude::Context,
    message: &channel::Message,
    _: standard::Args,
) -> standard::CommandResult {
    message
        .channel_id
        .say(&context.http, "Yes I am alive")
        .await?;
    Ok(())
}
