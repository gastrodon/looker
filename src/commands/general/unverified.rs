use crate::{config_for, edit, maybe};
use chrono::offset::Utc;
use lib::config::server;
use serenity::{
    framework::standard::macros::command,
    framework::standard::{Args, CommandResult},
    model::channel::Message,
    model::guild::{Guild, Member},
    model::id::{RoleId, UserId},
    model::misc::Mention,
    prelude::Context,
    Result,
};
use std::thread;
use tokio::runtime::Handle;

const ROLE_CHECK_THREADS: usize = 32;
const ROLE_CHECK_CHUNK_MIN: usize = 4;

macro_rules! chunk_size {
    ($it:expr) => {
        if $it < ROLE_CHECK_THREADS {
            ROLE_CHECK_CHUNK_MIN
        } else {
            $it / ROLE_CHECK_THREADS
        }
    };
}

macro_rules! fmt_duration {
    ($it:expr) => {
        format!(
            "`{days}d {hours}h {minutes}m`",
            days = $it.num_days(),
            hours = $it.num_hours() - ($it.num_days() * 24),
            minutes = $it.num_minutes() - ($it.num_hours() * 60),
        )
    };
}

async fn dispatch_collect_uvs(chunks: Vec<Vec<Member>>, role_id: RoleId) -> Vec<Member> {
    let handles = {
        let mut handles = vec![];

        for chunk in chunks {
            let handle = {
                let tokio_handle = Handle::current();

                thread::spawn(move || {
                    tokio_handle.spawn(async move {
                        let mut uvs = Vec::<Member>::new();

                        // TODO why am I pushing manually instead of using .filter() or something
                        for who in chunk {
                            if who.roles.iter().any(|it| *it == role_id) {
                                continue;
                            }

                            uvs.push(who)
                        }

                        uvs
                    })
                })
            };

            handles.push(handle);
        }

        handles
    };

    let mut collected = Vec::<Member>::new();

    for handle in handles {
        let mut resolved = match handle.join() {
            Ok(it) => match it.await {
                Ok(it) => it,
                Err(why) => panic!("{:?}", why),
            },
            Err(why) => panic!("{:?}", why),
        };

        collected.append(&mut resolved);
    }

    collected
}

async fn collect_chunks(ctx: &Context, guild: Guild) -> Result<Vec<Vec<Member>>> {
    let entire = {
        let mut collected = Vec::<Member>::new();
        let mut last = Option::<UserId>::None;

        loop {
            let mut buffer = guild
                .members(&ctx, Some(1000), last)
                .await?
                .into_iter()
                .filter(|it| !it.user.bot)
                .collect::<Vec<Member>>();

            if buffer.is_empty() {
                break;
            };

            last = buffer.last().map(|it| it.user.id);
            collected.append(&mut buffer)
        }

        collected
    };

    let size = chunk_size!(entire.len());
    let mut chunks = Vec::<Vec<Member>>::new();
    let mut single = Vec::<Member>::new();

    for it in entire.iter() {
        if single.len() == size {
            chunks.push(single);
            single = Vec::<Member>::new();
        };

        single.push(<Member>::clone(it));
    }

    Ok(chunks)
}

#[inline]
fn draw_uvs(mut members: Vec<Member>) -> String {
    let now = Utc::now();

    members.sort_by_key(|who| who.joined_at);
    members
        .iter()
        .map(|who| {
            let diff = match who.joined_at {
                Some(when) => now.signed_duration_since(when),
                None => panic!("{} never joined", who.user.name), // TODO
            };

            format!(
                "{mention} unverified for {time}",
                mention = Mention::from(who).to_string(),
                time = fmt_duration!(diff),
            )
        })
        .rev()
        .collect::<Vec<String>>()
        .join("\n")
}

#[command]
#[aliases("uvs")]
pub async fn unverified(ctx: &Context, message: &Message, _: Args) -> CommandResult {
    let channel = message.channel_id;
    let mut sent = maybe!(channel.say(&ctx.http, "collecting roster").await, Result);

    let config = match message.guild_id {
        Some(guild_id) => config_for!(guild_id, ctx.data.read().await),
        None => return Ok(()),
    };

    let verify_role = match config.verify.verified_id {
        Some(id) => id,
        None => panic!("no verify role"),
    };

    let chunks = maybe!(
        collect_chunks(ctx, maybe!(message.guild(&ctx.cache).await, Option)).await,
        Result
    );

    maybe!(
        edit!(
            &ctx.http,
            sent,
            format!("filtering {} user chunks", chunks.len())
        ),
        Result
    );

    let uvs = dispatch_collect_uvs(chunks, verify_role).await;
    maybe!(edit!(&ctx.http, sent, draw_uvs(uvs)), Result);
    Ok(())
}
