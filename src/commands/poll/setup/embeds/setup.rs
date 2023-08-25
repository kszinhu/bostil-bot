use rust_i18n::t;
use serenity::{builder::CreateEmbed, framework::standard::CommandResult, model::prelude::UserId};

pub fn embed(
    name: String,
    description: Option<String>,
    created_by: UserId,
) -> CommandResult<CreateEmbed> {
    let mut embed = CreateEmbed::default();
    embed
        .title(name)
        .description(t!("commands.poll.setup.embed.description").as_str());

    // first row (id, status, user)
    embed.field("User", format!("<@{}>", created_by), true);

    // separator
    embed.field("\u{200B}", "\u{200B}", false);

    embed.field(
        "Description",
        description.unwrap_or(t!("commands.poll.setup.embed.description_none")),
        false,
    );

    Ok(embed)
}
