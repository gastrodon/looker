use serenity::{async_trait, model::gateway, prelude};

pub struct Handler;

#[async_trait]
impl prelude::EventHandler for Handler {
    async fn ready(&self, _: prelude::Context, _: gateway::Ready) {
        println!("Connected");
    }
}
