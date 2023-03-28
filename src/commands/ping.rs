use crate::types::{CommandResult, Response};
use serenity::{
    builder::CreateApplicationCommand,
    model::application::interaction::application_command::ApplicationCommandInteraction,
    prelude::Context,
};

pub fn register() -> CreateApplicationCommand {
    let mut cmd = CreateApplicationCommand::default();
    cmd.name("ping").description("Check the bot's ping.");
    cmd
}

pub async fn handle(_: &Context, _: &ApplicationCommandInteraction) -> CommandResult {
    // TODO(dylhack): Implement this with ctx.http.get_gateway().await
    let ping = {
        let start = std::time::Instant::now();
        let end = std::time::Instant::now();
        end.duration_since(start).as_millis()
    };
    Response::ok(format!("Latency: `{}ms`", ping).as_str(), true)
}
