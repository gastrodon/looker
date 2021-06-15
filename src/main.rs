mod commands;
mod handler;

use lib::{config::server, config_for};
use serenity::{
    framework::standard::StandardFramework,
    model::id::{ChannelId, GuildId, RoleId},
    prelude::Client,
};
use std::collections::HashMap;

async fn populate_config_table(client: &mut Client) {
    client
        .data
        .write()
        .await
        .insert::<server::Key>(server::Table::new());
}

// TODO this is going away when we are database backed
async fn trans_default_config(client: &mut Client) {
    let trans_id = GuildId(527575883603509248);
    let mut handle = client.data.write().await;
    let mut config = config_for!(trans_id, handle);

    config.channels = server::Channels {
        introduction: Some(ChannelId(527581457279877131)),
        jail: Some(ChannelId(796061859659776041)),
        log: Some(ChannelId(796242938769571890)),
        role: Some(ChannelId(611966830034026496)),
        verify: Some(ChannelId(718170301682417754)),
        welcome: Some(ChannelId(807718099235241994)),

        quarantine: HashMap::new(),
    };

    config.verify = server::Verify::new(
        Some(RoleId(528098139044053002)),
        Some(RoleId(836752745665921044)),
        None,
    );

    let mut table = handle.get::<server::Key>().unwrap().clone();
    table.set(trans_id, config);
    handle.insert::<server::Key>(table);
    println!("trans server config set");
}

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN missing");

    let frame = StandardFramework::new()
        .configure(|it| it.prefix("-").allow_dm(false))
        .bucket("ratelimit", |it| it.limit(2).time_span(1))
        .await
        .group(&commands::GENERAL_GROUP)
        .group(&commands::CONFIGURE_GROUP);

    let mut client = Client::builder(&token)
        .event_handler(handler::Handler)
        .framework(frame)
        .await
        .expect("Client::builder failed");

    populate_config_table(&mut client).await;
    trans_default_config(&mut client).await;

    if let Err(why) = client.start().await {
        println!("{:?}", why);
    }
}
