use super::{handle_verify, EMOJI_CHECK, EMOJI_CROSS, EMOJI_QUESTION};
use crate::{
    client_data::{ServerConfig, ServerConfigKey},
    config_for,
};
use serenity::{
    async_trait,
    model::channel::{Reaction, ReactionType},
    prelude::{Context, EventHandler},
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn reaction_add(&self, ctx: Context, react: Reaction) {
        let code = match &react.emoji {
            ReactionType::Unicode(code) => code.as_str(),
            _ => return,
        };

        let config = match react.guild_id {
            Some(guild) => config_for!(guild, &ctx.data.read().await),
            None => return,
        };

        // TODO use fallthrough match crate?
        // https://github.com/pythonesque/fallthrough
        let result = match code {
            EMOJI_CHECK | EMOJI_CROSS | EMOJI_QUESTION => {
                if let Some(channel) = config.channels.verify {
                    if channel != react.channel_id {
                        return;
                    };
                } else {
                    return;
                };

                handle_verify(&ctx, &react, code, config).await
            }
            _ => Ok(()),
        };

        if let Err(why) = result {
            println!("{:?}", why);
        };
    }
}
