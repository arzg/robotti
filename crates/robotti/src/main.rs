use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::TypeMapKey;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let token = dotenv::var("DISCORD_TOKEN")?;

    let mut client = Client::builder(token).event_handler(Handler).await?;
    client
        .data
        .write()
        .await
        .insert::<StateWrapper>(speech_model::State::default());

    client.start().await?;

    Ok(())
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        eprintln!("{} is ready", ready.user.name);
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.name == ctx.cache.current_user().await.name {
            return;
        }

        let mut data = ctx.data.write().await;
        let state = data.get_mut::<StateWrapper>().unwrap();
        if let Some(reply) = state.handle_msg(&msg.content) {
            if let Err(e) = msg.reply(ctx.http, reply).await {
                eprintln!("Error: {}", e);
            }
        }
    }
}

struct StateWrapper;

impl TypeMapKey for StateWrapper {
    type Value = speech_model::State;
}
