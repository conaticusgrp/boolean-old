use crate::{log, util};
use sqlx::{types::time::PrimitiveDateTime, PgPool};

#[derive(sqlx::FromRow)]
pub struct GuildConfig {
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
    pub deleted_at: Option<PrimitiveDateTime>,
    pub guild_id: String,
    pub id: String,
}

#[derive(sqlx::FromRow)]
pub struct SpecialRole {
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
    pub deleted_at: Option<PrimitiveDateTime>,
    pub config_id: String,
    pub label: String,
    pub role_id: String,
    pub id: String,
}

#[derive(sqlx::FromRow)]
pub struct SpecialChannel {
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
    pub deleted_at: Option<PrimitiveDateTime>,
    pub config_id: String,
    pub label: String,
    pub channel_id: String,
    pub id: String,
}

async fn get_guild_config_opt(pool: &PgPool, guild_id: &String) -> Option<GuildConfig> {
    let config = sqlx::query_as!(
        GuildConfig,
        "SELECT * FROM guild_config WHERE guild_id = $1",
        guild_id
    ).fetch_one(pool).await;

    util::optional_row(config)
}

pub async fn set_guild_config(pool: &PgPool, guild_id: &String) -> Result<GuildConfig, String> {
    let res = sqlx::query!(
        "UPDATE guild_config SET deleted_at = NOW() WHERE guild_id = $1",
        guild_id
    ).execute(pool).await;

    if let Err(why) = res {
        log::error("Failed to set deleted_at for guild config", why);
    }

    let config = sqlx::query_as!(
        GuildConfig,
        "INSERT INTO guild_config (guild_id, id) VALUES ($1, $2) RETURNING *",
        guild_id, util::new_id()
    ).fetch_one(pool).await;

    match config {
        Ok(c) => Ok(c),
        Err(e) => Err(e.to_string()),
    }
}

pub async fn get_guild_config(pool: &PgPool, guild_id: &String) -> Result<GuildConfig, String> {
    match get_guild_config_opt(pool, guild_id).await {
        Some(config) => Ok(config),
        None => set_guild_config(pool, guild_id).await,
    }
}

impl GuildConfig {
    pub async fn get_special_roles(&self, pool: &PgPool, config_id: &String) -> Vec<SpecialRole> {
        let roles = sqlx::query_as!(
            SpecialRole,
            "SELECT * FROM special_role WHERE config_id = $1",
            config_id
        ).fetch_all(pool).await;
        util::empty_rows(roles)
    }

    pub async fn get_special_channels(
        &self,
        pool: &PgPool,
        config_id: &String,
    ) -> Vec<SpecialChannel> {
        let channels = sqlx::query_as!(
            SpecialChannel,
            "SELECT * FROM special_channel WHERE config_id = $1",
            config_id
        ).fetch_all(pool).await;
        util::empty_rows(channels)
    }

    pub async fn set_special_channel(
        &self,
        pool: &PgPool,
        config_id: &String,
        label: &String,
        channel_id: &String,
    ) -> Option<SpecialChannel> {
        let res = sqlx::query!(
            "UPDATE special_channel SET deleted_at = NOW() WHERE config_id = $1 AND label = $2",
            config_id,
            label
        ).execute(pool).await;
        if let Err(why) = res {
            log::error("Failed to set deleted_at for special channel", why);
        }

        let channel = sqlx::query_as!(SpecialChannel, "INSERT INTO special_channel (config_id, label, channel_id, id) VALUES ($1, $2, $3, $4) RETURNING *", config_id, label, channel_id, util::new_id())
        .fetch_one(pool).await;

        util::optional_row(channel)
    }

    pub async fn set_special_role(
        &self,
        pool: &PgPool,
        config_id: &String,
        label: &String,
        role_id: &String,
    ) -> Option<SpecialRole> {
        let res = sqlx::query!(
            "UPDATE special_role SET deleted_at = NOW() WHERE config_id = $1 AND label = $2",
            config_id,
            label
        )
        .execute(pool)
        .await;
        if let Err(why) = res {
            log::error("Failed to set deleted_at for special role", why);
        }

        let role = sqlx::query_as!(
            SpecialRole, 
            "INSERT INTO special_role (config_id, label, role_id, id) VALUES ($1, $2, $3, $4) RETURNING *",
            config_id, label, role_id, util::new_id()
        ).fetch_one(pool).await;

        util::optional_row(role)
    }
}