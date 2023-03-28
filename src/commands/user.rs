use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::{
        command::CommandOptionType,
        interaction::{
            application_command::ApplicationCommandInteraction, InteractionResponseType,
        },
        UserId,
    },
    prelude::Context,
};

use crate::{
    embeds,
    types::{CommandResult, Response},
};

pub fn register() -> CreateApplicationCommand {
    let mut cmd = CreateApplicationCommand::default();
    cmd.name("user")
        .description("Get details of a certain user.");
    cmd.create_option(|option| {
        option
            .name("user")
            .description("The user to get details of.")
            .kind(CommandOptionType::User)
            .required(true)
    });
    cmd
}

pub async fn handle(ctx: &Context, int: &ApplicationCommandInteraction) -> CommandResult {
    if let None = int.guild_id {
        return Response::warning("This command can only be used in a server.");
    }

    let id = int.data.options[0]
        .value
        .as_ref()
        .ok_or("No user provided.")?;
    let id_str = id
        .as_str()
        .ok_or("Option 0 for this command is not a UserId.")?;
    let id = id_str
        .parse::<u64>()
        .map_err(|_| "Failed to parse UserId.")?;
    let user_id = UserId(id);
    let guild_id = int.guild_id.ok_or("No guild ID, how did we get here?")?;
    let member = guild_id
        .member(&ctx.http, user_id)
        .await
        .map_err(|_| "Failed to get member.")?;
    let profile = embeds::create_profile_embed(&member);

    if let Err(why) = int
        .create_interaction_response(&ctx.http, |response| {
            response.kind(InteractionResponseType::ChannelMessageWithSource);
            response.interaction_response_data(|data| {
                data.embed(|embed| {
                    embed.0 = profile.0;
                    embed
                });
                data
            });
            response
        })
        .await
    {
        return Response::err(format!("Failed to send profile: {}", why));
    }

    Response::ignore()
}
