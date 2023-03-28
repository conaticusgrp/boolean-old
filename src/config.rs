use std::env::{self, VarError};

use serenity::model::prelude::GuildId;

fn get_var(name: &str) -> Result<String, String> {
    let key = format!("BOOLEAN_{}", name);

    match env::var(key.clone()) {
        Ok(value) => Ok(value),
        Err(VarError::NotPresent) => Err(format!("Missing {} env var", key)),
        Err(VarError::NotUnicode(_)) => Err(format!("Invalid \"{}\" env var", key)),
    }
}

pub fn get_token() -> String {
    let result = get_var("BOT_TOKEN");

    match result {
        Ok(token) => token,
        Err(reason) => panic!("{}", reason),
    }
}

pub fn get_dev_server_id() -> Option<GuildId> {
    let result = env::var("BOOLEAN_DEV_SERVER");

    match result {
        Ok(id) => {
            let parsed = id.parse::<u64>().unwrap_or(0);
            Some(GuildId(parsed))
        }
        Err(_) => None,
    }
}

pub fn get_database_url() -> String {
    let result = env::var("DATABASE_URL");

    match result {
        Ok(url) => url,
        Err(reason) => panic!("{}", reason),
    }
}

pub fn get_contact_url() -> String {
    let result = get_var("CONTACT_URL");

    match result {
        Ok(url) => url,
        Err(reason) => panic!("{}", reason),
    }
}

pub fn get_release_stage() -> String {
    let result = get_var("RELEASE_STAGE");

    if result.is_err() {
        return "development".to_string();
    }

    let parsed = result.unwrap_or("development".to_string()).to_lowercase();

    match parsed.as_str() {
        "2" | "production" => "production".to_string(),
        "1" | "staging" => "staging".to_string(),
        "0" | "development" => "development".to_string(),
        _ => panic!("Invalid release stage"),
    }
}

pub fn get_sentry_url() -> Option<String> {
    let result = get_var("SENTRY_URL");

    match result {
        Ok(url) => Some(url),
        Err(_) => None,
    }
}

pub fn get_log_level() -> u8 {
    let result = get_var("LOG_LEVEL");
    let level = result.unwrap_or("INFO".to_string());

    match level.as_str() {
        "0" | "DEBUG" => 0,
        "1" | "INFO" => 1,
        "2" | "WARN" => 2,
        "3" | "ERROR" => 3,
        _ => 1,
    }
}
