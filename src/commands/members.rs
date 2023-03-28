use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::{
        interaction::application_command::ApplicationCommandInteraction, GuildId, UserId,
    },
    prelude::Context,
};

use crate::types::{CommandResult, Response};

pub fn register() -> CreateApplicationCommand {
    let mut cmd = CreateApplicationCommand::default();
    cmd.name("members")
        .description("Count the number of members in this server (Excluding bots).");
    cmd
}

pub async fn handle(ctx: &Context, int: &ApplicationCommandInteraction) -> CommandResult {
    if let None = int.guild_id {
        return Response::warning("This command can only be used in a server.");
    }
    let guild_id = int.guild_id.unwrap();
    let count = count_all_members(ctx, &guild_id).await;
    Response::ok(
        format!("There are {} members in this server.", count).as_str(),
        true,
    )
}

async fn count_all_members(ctx: &Context, guild: &GuildId) -> u64 {
    let mut result = 0;
    let mut last_id: Option<UserId> = None;
    loop {
        let members = guild.members(&ctx.http, Some(1000), last_id).await.unwrap();
        let length = members.len();
        if members.is_empty() {
            break;
        }

        for member in &members {
            if !member.user.bot {
                result += 1;
            }
        }

        if length < 1000 {
            break;
        }

        let last = members.last();
        if let None = last {
            break;
        }
        last_id = Some(last.unwrap().user.id);
    }
    result
}
