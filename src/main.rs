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
        .insert::<StateWrapper>(State::default());

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
        if let Some(reply) = handle_reply(state, &msg.content) {
            if let Err(e) = msg.reply(ctx.http, reply).await {
                eprintln!("Error: {}", e);
            }
        }
    }
}

fn handle_reply(state: &mut State, they_said: &str) -> Option<String> {
    let they_said = they_said.to_lowercase();
    let components: Vec<_> = they_said.split(|c: char| !c.is_alphanumeric()).collect();

    match state.mode {
        Mode::Neutral => {
            if components.contains(&"hello") || components.contains(&"hi") {
                Some("Greetings!".to_string())
            } else if components.contains(&"who") {
                Some("Glad you asked! I’m robotti, your personal assistant.".to_string())
            } else if components.contains(&"why") {
                Some("Why not?".to_string())
            } else if components.contains(&"reminder") {
                state.mode = Mode::SettingReminder;
                Some("What would you like me to remind you of?".to_string())
            } else if they_said == "Initiating btot fight" {
                state.mode = Mode::Fight;
                Some("KIL!".to_string())
            } else {
                None
            }
        }
        Mode::SettingReminder => {
            state.reminders.push(they_said.to_string());
            state.mode = Mode::Neutral;

            Some("OK, I’ll remember that.".to_string())
        }
        Mode::Fight => Some("KIL!".to_string()),
    }
}

struct StateWrapper;

impl TypeMapKey for StateWrapper {
    type Value = State;
}

#[derive(Default)]
struct State {
    mode: Mode,
    reminders: Vec<String>,
}

enum Mode {
    Neutral,
    SettingReminder,
    Fight,
    WhyKil,
}

impl Default for Mode {
    fn default() -> Self {
        Self::Neutral
    }
}
