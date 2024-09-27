use serenity::all::{ActivityData, Context, Ready};
use serenity::client::EventHandler;

pub struct FromisHandler;

#[serenity::async_trait]
impl EventHandler for FromisHandler {
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
