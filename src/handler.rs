use crate::github::GitHubPreview;
use serenity::all::{ActivityData, Context, Message, Ready};
use serenity::client::EventHandler;

pub struct FromisHandler;

#[serenity::async_trait]
impl EventHandler for FromisHandler {
    async fn message(&self, ctx: Context, message: Message) {
        if message.author.bot || !GitHubPreview::is_exist(&message.content) {
            return;
        }

        let preview = match crate::preview::get_preview(&message.content).await {
            Ok(preview) => preview,
            Err(e) => {
                tracing::error!("Failed to get preview: {}", e);
                return;
            }
        };

        if let Err(why) = message.reply(&ctx.http, preview).await {
            tracing::error!("Failed to send message: {}", why);
        }
    }

    async fn ready(&self, ctx: Context, client: Ready) {
        tracing::info!(
            "Connected as {}, id: {}",
            &client.user.name,
            &client.user.id
        );
        ctx.set_activity(Some(ActivityData::playing(format!(
            "v{}",
            env!("CARGO_PKG_VERSION")
        ))));

        tracing::debug!("Client: {:?}", &client);
        tracing::info!("fromis is ready!");
    }
}
