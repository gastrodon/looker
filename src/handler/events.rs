use super::{handle_verify, EMOJI_CHECK, EMOJI_CROSS};
use crate::client_data::VerifyKey;
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

        let readable = &ctx.data.read().await;

        let result = match code {
            EMOJI_CHECK | EMOJI_CROSS => {
                let verify = *readable.get::<VerifyKey>().unwrap();
                if react.channel_id == verify.channel_id {
                    handle_verify(&ctx, &react, code, verify).await
                } else {
                    Ok(())
                }
            }
            _ => Ok(()),
        };

        if let Err(why) = result {
            println!("{:?}", why);
        }
    }
}
