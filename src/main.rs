mod client_data;
mod commands;
mod handler;
mod macros;

use serenity::{
    framework::standard::StandardFramework,
    model::id::{ChannelId, RoleId},
    model::permissions::Permissions,
    prelude::Client,
};

static VERIFY_CHANNEL: u64 = 718170301682417754;
static VERIFIED_ROLE: u64 = 528098139044053002;
static UNVERIFIED_ROLE: u64 = 836752745665921044;

static INTRODUCTIONS_CHANNEL: u64 = 527581457279877131;
static LOGS_CHANNEL: u64 = 796242938769571890;
static ROLES_CHANNEL: u64 = 611966830034026496;
static WELCOME_CHANNEL: u64 = 807718099235241994;

async fn infil_verify(client: &mut Client) {
    let verify = client_data::Verify {
        channel_id: ChannelId(VERIFY_CHANNEL),
        verified: RoleId(VERIFIED_ROLE),
        unverified: RoleId(UNVERIFIED_ROLE),
        permissions: Permissions::MANAGE_ROLES
            | Permissions::MANAGE_MESSAGES
            | Permissions::KICK_MEMBERS,
    };

    client
        .data
        .write()
        .await
        .insert::<client_data::VerifyKey>(verify);
}

async fn infil_channels(client: &mut Client) {
    let channels = client_data::Channels {
        introductions: ChannelId(INTRODUCTIONS_CHANNEL),
        logs: ChannelId(LOGS_CHANNEL),
        roles: ChannelId(ROLES_CHANNEL),
        welcome: ChannelId(WELCOME_CHANNEL),
    };

    client
        .data
        .write()
        .await
        .insert::<client_data::ChannelsKey>(channels);
}

async fn infil_configs(client: &mut Client) {
    client
        .data
        .write()
        .await
        .insert::<client_data::ServerConfigKey>(client_data::ServerConfigTable::new());
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

    infil_verify(&mut client).await;
    infil_channels(&mut client).await;
    infil_configs(&mut client).await;

    if let Err(why) = client.start().await {
        println!("{:?}", why);
    }
}
