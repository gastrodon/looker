use crate::{
    client_data::{ServerConfig, ServerConfigKey},
    config_for, edit, maybe,
};
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::{channel::Channel, channel::Message, id::RoleId},
    prelude::Context,
};

static HELP: &str = r#"```
-config <verify|unverify> <@role>

Used to configure the role verified / unverified users should be given / drop.
```"#;

async fn set_role(ctx: &Context, channel: Channel, args: Args, kind: &str) -> CommandResult {
    let mut sent = maybe!(
        channel
            .id()
            .say(&ctx.http, format!("setting {kind}", kind = kind))
            .await,
        Result
    );

    let mut name = String::new();
    let guild = channel.guild().unwrap().guild(&ctx.cache).await.unwrap();
    let role_id = if let Ok(role_id) = args.parse::<RoleId>() {
        println!("{:?}", role_id);
        match role_id.to_role_cached(&ctx.cache).await {
            Some(role) => {
                name = role.name;
                role_id
            }
            None => {
                maybe!(
                    edit!(
                        &ctx.http,
                        sent,
                        format!("No such role with id `{id}`", id = role_id.as_u64())
                    ),
                    Result
                );
                return Ok(());
            }
        }
    } else {
        name = args.parse::<String>().unwrap();
        match guild.role_by_name(&name) {
            Some(role) => role.id,
            None => {
                maybe!(
                    edit!(
                        &ctx.http,
                        sent,
                        format!("I don't know what `{name}` is", name = name)
                    ),
                    Result
                );
                return Ok(());
            }
        }
    };

    let mut handle = ctx.data.write().await;
    let mut config = config_for!(guild.id, handle);

    match kind {
        "verify" => config.verify.verified_id = Some(role_id),
        "unverify" => config.verify.unverified_id = Some(role_id),
        _ => unreachable!(),
    };

    let mut table = handle.get::<ServerConfigKey>().unwrap().clone();
    table.set(guild.id, config);
    handle.insert::<ServerConfigKey>(table);

    maybe!(
        edit!(&ctx.http, sent, format!("Verify is `@{role}`", role = name)),
        Result
    );
    Ok(())
}

#[command]
#[only_in("guild")]
#[required_permissions("MANAGE_CHANNELS", "MANAGE_ROLES")]
pub async fn verify(ctx: &Context, message: &Message, args: Args) -> CommandResult {
    let channel = message.channel(&ctx.cache).await.unwrap();
    if args.len() != 1 {
        maybe!(channel.id().say(&ctx.http, HELP).await, Result);
        return Ok(());
    };

    set_role(ctx, channel, args, "verify").await
}

#[command]
#[only_in("guild")]
#[required_permissions("MANAGE_CHANNELS", "MANAGE_ROLES")]
pub async fn unverify(ctx: &Context, message: &Message, args: Args) -> CommandResult {
    let channel = message.channel(&ctx.cache).await.unwrap();
    if args.len() != 1 {
        maybe!(channel.id().say(&ctx.http, HELP).await, Result);
        return Ok(());
    };

    set_role(ctx, channel, args, "unverify").await
}
