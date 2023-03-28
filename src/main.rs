use serenity::{model::gateway::GatewayIntents, Client};

use crate::database::start_db;

mod commands;
mod config;
mod database;
mod embeds;
mod events;
mod log;
mod types;
mod util;

async fn start_bot() {
    // Init Discord connection
    let mut intents = GatewayIntents::default();
    intents.insert(GatewayIntents::GUILDS);
    intents.insert(GatewayIntents::GUILD_MESSAGES);
    intents.insert(GatewayIntents::GUILD_MEMBERS);

    let token = config::get_token();
    let mut client = Client::builder(token, intents)
        .event_handler(events::Handler {})
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        log::error("Discord connection error", why);
    }
}

fn init_sentry() -> Option<sentry::ClientInitGuard> {
    // Init logging
    if let Some(sentry_url) = config::get_sentry_url() {
        Some(log::init_sentry(sentry_url))
    } else {
        None
    }
}

#[tokio::main]
async fn main() {
    // TODO(dylhack): wrap up main when it's ready
    log::info("Starting...");
    let _guard = init_sentry();
    start_bot().await;
}
