use crate::{
    config, embeds, log,
    types::{CommandResult, Response},
};
use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::{
        command::Command, interaction::application_command::ApplicationCommandInteraction, GuildId,
    },
    prelude::Context,
};

mod clear;
mod members;
mod ping;
mod user;

pub async fn handle(ctx: &Context, cmd: &ApplicationCommandInteraction) {
    sentry::configure_scope(|scope| {
        let context = log::command_ctx(cmd);
        scope.set_context("Command", context);
        scope.set_tag("Controller Type", "command");
        scope.set_tag("Command", cmd.data.name.clone());
    });

    ///////////////////////////////////////////////////////////////////////////
    // Command Handler                                                       //
    ///////////////////////////////////////////////////////////////////////////
    let result = match cmd.data.name.as_str() {
        "ping" => ping::handle(ctx, cmd).await,
        "clear" => clear::handle(ctx, cmd).await,
        "members" => members::handle(ctx, cmd).await,
        "user" => user::handle(ctx, cmd).await,
        _ => Err(format!("Command not found `{}`", cmd.data.name)),
    };

    respond(ctx, cmd, &result).await;
}

pub async fn register_all(ctx: &Context) -> std::io::Result<()> {
    let dev_server_id = config::get_dev_server_id();
    ///////////////////////////////////////////////////////////////////////////
    // Register commands here                                                //
    ///////////////////////////////////////////////////////////////////////////
    let cmds = vec![
        clear::register(),
        members::register(),
        ping::register(),
        user::register(),
    ];

    if let Some(id) = dev_server_id {
        register_to_dev(cmds, ctx, &id).await?;
    } else {
        register_to_global(cmds, ctx).await?;
    }

    Ok(())
}

async fn register_to_dev(
    cmds: Vec<CreateApplicationCommand>,
    ctx: &Context,
    guild_id: &GuildId,
) -> std::io::Result<()> {
    let result = guild_id
        .set_application_commands(&ctx.http, |f| {
            for cmd in cmds {
                f.add_application_command(cmd);
            }
            f
        })
        .await;

    if let Err(why) = result {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            why.to_string(),
        ));
    }

    Ok(())
}

async fn register_to_global(
    cmds: Vec<CreateApplicationCommand>,
    ctx: &Context,
) -> std::io::Result<()> {
    let result = Command::set_global_application_commands(&ctx.http, |f| {
        for cmd in cmds {
            f.add_application_command(cmd);
        }
        f
    })
    .await;

    if let Err(why) = result {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            why.to_string(),
        ));
    }

    Ok(())
}

async fn respond(ctx: &Context, cmd: &ApplicationCommandInteraction, result: &CommandResult) {
    const DEFAULT_EPHEMERAL: bool = true;

    let response = cmd
        .create_interaction_response(&ctx.http, |f| {
            f.interaction_response_data(|f| {
                match result {
                    Err(why) => {
                        let embed = embeds::create_error_embed(why);
                        f.add_embed(embed);
                        f.ephemeral(DEFAULT_EPHEMERAL);
                        // TODO(dylhack): so it worky
                        log::error("Failed to respond to interaction", why);
                    }
                    Ok(resp) => {
                        match resp {
                            Response::Success => {
                                f.content("Done.");
                                f.ephemeral(DEFAULT_EPHEMERAL);
                            }
                            Response::Ok(msg, is_eph) => {
                                f.content(msg);
                                f.ephemeral(*is_eph);
                            }
                            // NOTE(dylhack): this will be the same as Ok for now
                            Response::Warning(why) => {
                                f.content(why);
                                f.ephemeral(DEFAULT_EPHEMERAL);
                            }
                            Response::Ignore => {}
                        };
                    }
                }
                f
            })
        })
        .await;

    if let Err(why) = response {
        log::error("Error sending response", why);
    }
}
