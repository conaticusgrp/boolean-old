use std::collections::BTreeMap;

use crate::config::get_log_level;
use sentry::{protocol::Context, ClientInitGuard, Level};
use serenity::model::prelude::{
    interaction::{application_command::ApplicationCommandInteraction, Interaction},
    GuildId,
};

fn log(level: u8, message: &str) {
    let config_level = get_log_level();
    if level <= config_level {
        println!("{}", message);
    }
}

pub fn debug(message: &str) {
    log(0, message);
}

pub fn info(message: &str) {
    log(1, message);
}

pub fn warn<T: ToString>(message: &str, detail: T) {
    log(2, message);
    sentry::capture_message(message, Level::Warning);
}

pub fn error<T: ToString>(message: &str, detail: T) {
    log(3, message);
    sentry::capture_message(message, Level::Error);
}

pub fn init_sentry(url: String) -> ClientInitGuard {
    sentry::init((
        url,
        sentry::ClientOptions {
            release: sentry::release_name!(),
            traces_sample_rate: 0.2,
            ..Default::default()
        },
    ))
}

pub fn interaction_ctx(interaction: &Interaction) -> Context {
    let mut ctx = BTreeMap::new();
    ctx.insert("ID".to_string(), interaction.id().to_string().into());

    match interaction {
        Interaction::ApplicationCommand(cmd) => {
            ctx.insert("Command".to_string(), cmd.data.name.to_string().into());
            ctx.insert(
                "Guild".to_string(),
                cmd.guild_id.unwrap_or(GuildId(0)).to_string().into(),
            );
            ctx.insert("Channel".to_string(), cmd.channel_id.to_string().into());
        }
        Interaction::MessageComponent(component) => {
            ctx.insert(
                "Component".to_string(),
                component.data.custom_id.to_string().into(),
            );
            ctx.insert(
                "Guild".to_string(),
                component.guild_id.unwrap_or(GuildId(0)).to_string().into(),
            );
            ctx.insert(
                "Channel".to_string(),
                component.channel_id.to_string().into(),
            );
        }
        Interaction::ModalSubmit(modal) => {
            ctx.insert("Modal".to_string(), modal.data.custom_id.to_string().into());
            ctx.insert(
                "Guild".to_string(),
                modal.guild_id.unwrap_or(GuildId(0)).to_string().into(),
            );
            ctx.insert("Channel".to_string(), modal.channel_id.to_string().into());
        }
        Interaction::Ping(_) => {}
        Interaction::Autocomplete(_) => {}
    }

    Context::Other(ctx)
}

pub fn command_ctx(command: &ApplicationCommandInteraction) -> Context {
    // NOTE(dylhack) anything already covered by interaction_ctx shouldn't be added here
    let mut ctx = BTreeMap::new();
    ctx.insert("Command ID".to_string(), command.id.to_string().into());
    ctx.insert("Command".to_string(), command.data.name.to_string().into());

    Context::Other(ctx)
}

pub fn user_ctx(user: &serenity::model::user::User) -> sentry::User {
    sentry::User {
        id: Some(user.id.to_string()),
        username: Some(user.tag()),
        ..Default::default()
    }
}

pub fn interaction_user(interaction: &Interaction) -> Option<sentry::User> {
    let user = match interaction {
        Interaction::ApplicationCommand(cmd) => Some(cmd.user.clone()),
        Interaction::MessageComponent(component) => Some(component.user.clone()),
        Interaction::ModalSubmit(modal) => Some(modal.user.clone()),
        Interaction::Ping(_) => None,
        Interaction::Autocomplete(_) => None,
    };
    if let Some(user) = user {
        Some(user_ctx(&user))
    } else {
        None
    }
}
