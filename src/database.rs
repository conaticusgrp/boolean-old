use crate::config;
use sqlx::{postgres::PgPoolOptions, PgPool};

mod badges;
mod guild_config;

pub async fn init() -> PgPool {
    let url = config::get_database_url();

    PgPoolOptions::new().connect(url.as_str()).await.unwrap()
}

pub async fn start_db() {
    let pool = init().await;

    let badge = badges::get_badge(&pool, "DISCORD_EMPLOYEE".to_string()).await;

    match badge {
        Some(badge) => println!("Badge: {:?}", badge.emoji),
        None => println!("No badge found"),
    };
}
