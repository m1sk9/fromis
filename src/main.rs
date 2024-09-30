mod github;
mod handler;
mod preview;

use serenity::prelude::GatewayIntents;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

#[derive(serde::Deserialize)]
pub struct FromisEnv {
    pub discord_api_token: String,
}

pub fn from_env() -> &'static FromisEnv {
    static ENV: std::sync::OnceLock<FromisEnv> = std::sync::OnceLock::new();
    ENV.get_or_init(|| envy::from_env().expect("Failed to parse environment variables"))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("fromis=debug"));
    let subscriber = FmtSubscriber::builder().with_env_filter(filter).finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set tracing_subscriber as global default.");

    if let Err(why) = dotenvy::dotenv() {
        Err(anyhow::anyhow!("Failed to load .env file: {}", why))?;
    };

    let envs = from_env();
    let intents = GatewayIntents::MESSAGE_CONTENT | GatewayIntents::GUILD_MESSAGES;
    let mut client = serenity::Client::builder(&envs.discord_api_token, intents)
        .event_handler(handler::FromisHandler)
        .intents(intents)
        .await?;

    if let Err(why) = client.start().await {
        Err(anyhow::anyhow!("Failed to start the client: {}", why))?;
    };

    Ok(())
}
