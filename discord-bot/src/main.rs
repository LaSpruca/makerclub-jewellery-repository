mod bot;
mod db;

use bot::{submit_jewellery, UserData};
use db::{DbConnection, DB};
use poise::serenity_prelude as serenity;
use tracing::error;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt()
        .with_level(true)
        .with_file(true)
        .with_line_number(true)
        .init();

    let Ok(connection_url) = std::env::var("DATABASE_URL") else {
        error!("Could not get connection url, please set the POSTGRES_URL environment variable");
        return;
    };

    let Ok(discord_token) = std::env::var("DISCORD_TOKEN") else {
        error!("Could not get discord token, please set the DISCORD_TOKEN environment variable");
        return;
    };

    let db = match DbConnection::new(connection_url).await {
        Ok(val) => val,
        Err(ex) => {
            error!("Could not connect to postgres {ex}");
            return;
        }
    };

    if let Err(_) = DB.set(db) {
        error!("Attempted to set DB twice");
        return;
    };

    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![submit_jewellery()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                Ok(UserData {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(discord_token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap();
}
