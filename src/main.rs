mod commands;
mod handler;

use serenity::{framework::standard, prelude};

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN missing");

    let frame = standard::StandardFramework::new()
        .configure(|it| it.prefix("-").allow_dm(false))
        .bucket("ratelimit", |it| it.limit(2).time_span(1))
        .await
        .group(&commands::GENERAL_GROUP);

    let mut client = prelude::Client::builder(&token)
        .event_handler(handler::Handler)
        .framework(frame)
        .await
        .expect("Client::builder failed");

    if let Err(why) = client.start().await {
        println!("{:?}", why);
    }
}
