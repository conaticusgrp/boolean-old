use serenity::futures::future::join_all;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::command::CommandOptionType;
use serenity::{builder::CreateApplicationCommand, prelude::Context};

use crate::types::{CommandResult, Response};

pub fn register() -> CreateApplicationCommand {
    let mut cmd = CreateApplicationCommand::default();
    cmd.name("clear")
        .description("Delete specified amount of messages.");
    cmd.create_option(|option| {
        option
            .name("amount")
            .description("Amount of messages to delete")
            .kind(CommandOptionType::Integer)
            .required(true);
        option
    });
    cmd
}

pub async fn handle(ctx: &Context, interaction: &ApplicationCommandInteraction) -> CommandResult {
    let option = interaction.data.options[0]
        .value
        .as_ref()
        .ok_or("Command isn't forcing option 1 to be utilized.")?;
    let amount = option
        .as_u64()
        .ok_or("Option 1 for this command is not a u64.")?;
    let targets = interaction
        .channel_id
        .messages(&ctx.http, |retriever| {
            retriever.limit(amount);
            retriever
        })
        .await;

    if let Err(why) = targets {
        return Response::err(format!("Failed to retrieve messages: {}", why));
    }

    let messages = targets.unwrap();
    let tasks = messages.iter().map(|msg| msg.delete(&ctx.http));

    join_all(tasks).await;
    Response::ok(format!("Deleted {} messages.", amount).as_str(), true)
}
