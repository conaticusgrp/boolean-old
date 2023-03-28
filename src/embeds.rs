use crate::config;
use serenity::{builder::CreateEmbed, model::prelude::Member, utils::Color};

fn create_embed() -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    embed.color(Color::ORANGE);
    embed
}

pub fn create_error_embed(why: &String) -> CreateEmbed {
    let mut embed = create_embed();
    let contact = config::get_contact_url();
    let message = format!(
    "**An internal error has occurred**, this has been reported to the developers. Please contact us [here]({}) for further discussion.\n```\n{}\n```",
    contact,
    why
  );
    embed.title("Error").description(message).color(Color::RED);
    embed
}

pub fn create_profile_embed(member: &Member) -> CreateEmbed {
    let mut embed = create_embed();
    let user = &member.user;
    embed.title(format!("{}'s Profile ({})", user.name, user.id));
    // fields
    embed.field("**Created At**", format!("{}", user.created_at()), true);
    if let Some(joined_at) = &member.joined_at {
        embed.field("**Joined At**", format!("{}", joined_at), true);
    }
    if let Some(nick) = &member.nick {
        embed.field("**Nickname**", nick, false);
    }

    embed.thumbnail(user.face());
    if let Some(url) = &user.banner_url() {
        embed.image(url);
    }
    embed
}
