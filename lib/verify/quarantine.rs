use crate::{config::server, config_for};
use serenity::{
    futures::StreamExt,
    model::{
        channel::{Embed, Message, PermissionOverwrite, PermissionOverwriteType},
        guild::Member,
        misc::Mention,
        permissions::Permissions,
    },
    prelude::Context,
    Result,
};

pub async fn open_quarantine(ctx: &Context, author: Member, config: server::Config) -> Result<()> {
    let name = format!(
        "quarantine-{}-{}",
        author.user.name, author.user.discriminator
    );

    let channel = author
        .guild_id
        .create_channel(&ctx.http, |it| {
            it.name(&name).permissions(vec![PermissionOverwrite {
                allow: Permissions::empty(),
                deny: Permissions::READ_MESSAGES | Permissions::SEND_MESSAGES,
                kind: PermissionOverwriteType::Role((*author.guild_id.as_u64()).into()),
            }])
        })
        .await?;

    {
        let id = author.guild_id;

        let mut handle = ctx.data.write().await;
        let mut config = config_for!(id, handle);
        config.start_quarantine(channel.id, author.user.id);

        let mut table = handle.get::<server::Key>().unwrap().clone();
        table.set(id, config);
        handle.insert::<server::Key>(table);
    }

    channel
        .create_permission(
            &ctx.http,
            &PermissionOverwrite {
                allow: Permissions::READ_MESSAGES | Permissions::SEND_MESSAGES,
                deny: Permissions::empty(),
                kind: PermissionOverwriteType::Member(author.user.id),
            },
        )
        .await
        .ok();

    channel
        .say(
            &ctx.http,
            format!(
                "Hello {who}, we've moved you in here to chat before you're verified",
                who = Mention::from(author.user.id)
            ),
        )
        .await
        .ok();

    if let Some(channel_id) = config.channels.verify {
        let mut buffer = Vec::<Message>::new();
        let mut handle = channel_id.messages_iter(&ctx.http).boxed();

        while let Some(next) = handle.next().await {
            if let Ok(message) = next {
                if message.author.id == author.user.id {
                    buffer.push(message)
                };
            };
        }

        let webhook = channel.create_webhook(&ctx.http, &name).await?;
        let avatar_url = author
            .user
            .avatar_url()
            .unwrap_or_else(|| author.user.default_avatar_url());

        for message in buffer.iter().rev() {
            webhook
                .execute(&ctx.http, true, |it| {
                    it.embeds(vec![Embed::fake(|embed| {
                        embed.description(&message.content)
                    })])
                    .avatar_url(&avatar_url)
                    .username(&author.user.name)
                })
                .await
                .ok();

            message.delete(&ctx.http).await.ok();
        }
    };

    Ok(())
}
