use crate::util;
use sqlx::PgPool;

#[derive(sqlx::FromRow)]
pub struct Badge {
    pub id: String,
    pub emoji: String,
}

pub async fn get_badge(pool: &PgPool, id: String) -> Option<Badge> {
    let badge = sqlx::query_as!(Badge, "SELECT * FROM badge WHERE id = $1", id)
        .fetch_one(pool)
        .await;

    util::optional_row(badge)
}
