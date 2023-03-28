use crate::commands;
use crate::{config, log};
use sentry::protocol::AppContext;
use serenity::{client::Context, model::gateway::Ready};
use std::time::SystemTime;

pub async fn handle(ctx: &Context, ready: &Ready) {
    sentry::configure_scope(|scope| {
        let release_stage = config::get_release_stage();
        scope.set_context(
            "Core Context",
            AppContext {
                app_start_time: Some(SystemTime::now()),
                build_type: Some(release_stage),
                app_name: Some(ready.user.tag()),
                app_identifier: Some(ready.user.id.to_string()),
                ..Default::default()
            },
        );
    });

    if let Err(why) = commands::register_all(ctx).await {
        log::error("Error registering command", why);
    }

    let num_members = get_member_count(ctx, ready).await;
    let num_guilds = ready.guilds.len();
    log::info(
        format!(
            "Ready. Serving {} guilds and ~{} members",
            num_guilds, num_members
        )
        .as_str(),
    );
}

async fn get_member_count(ctx: &Context, ready: &Ready) -> u64 {
    let mut count: u64 = 0;
    for guild in &ready.guilds {
        let guild_id = guild.id;
        let result = guild_id.to_partial_guild_with_counts(&ctx.http).await;
        if let Ok(guild) = result {
            if let Some(amount) = guild.approximate_member_count {
                count += amount;
            }
        }
    }
    count
}
