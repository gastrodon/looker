use lib::{config::server, edit, maybe};
use serenity::{
    framework::standard::macros::command,
    framework::standard::{Args, CommandResult},
    model::channel::Message,
    model::id::ChannelId,
    prelude::Context,
};

static CONFIG_CHANNELS_HELP: &str = r#"```
-config channel <kind #channel> [kind #channel]

Used to configure what channels should be used for what.


kind may be one of
    introduction:   Where users should send introduction messages
    jail:           Where users go when they're icky
    log:            Where event logs should be sent to
    role:           Where users self assign roles
    verify:         Where incoming users are verified
    welcome:        Where welcome messages should be sent
```"#;

#[command]
#[aliases("channel")]
#[only_in("guild")]
#[required_permissions("MANAGE_CHANNELS")]
pub async fn channels(ctx: &Context, message: &Message, mut args: Args) -> CommandResult {
    let channel = message.channel_id;

    if args.is_empty() || args.len() % 2 != 0 {
        maybe!(channel.say(&ctx.http, CONFIG_CHANNELS_HELP,).await, Result);

        return Ok(());
    };

    let mut sent = maybe!(channel.say(&ctx.http, "collecting channels").await, Result);

    let mut handle = ctx.data.write().await;
    let mut config = handle.get::<server::Key>().unwrap().clone();
    let id = message.guild_id.unwrap();
    let mut table = config.get(id).unwrap_or(&server::Config::new()).clone();

    while let (Ok(name), Ok(channel)) = (args.single::<String>(), args.single::<ChannelId>()) {
        match name.as_str() {
            "introduction" => table.channels.introduction = Some(channel),
            "jail" => table.channels.jail = Some(channel),
            "log" => table.channels.log = Some(channel),
            "role" => table.channels.role = Some(channel),
            "verify" => table.channels.verify = Some(channel),
            "welcome" => table.channels.welcome = Some(channel),
            other => {
                maybe!(
                    edit!(
                        &ctx.http,
                        sent,
                        format!("unsupported channel type `{}`", other)
                    ),
                    Result
                )
            }
        };
    }

    config.set(id, table);
    handle.insert::<server::Key>(config);
    maybe!(edit!(&ctx.http, sent, "config set"), Result);
    Ok(())
}
