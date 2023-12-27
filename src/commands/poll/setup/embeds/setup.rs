use rust_i18n::t;
use serenity::{
    builder::CreateEmbed, client::Context, framework::standard::CommandResult,
    model::prelude::UserId,
};
use uuid::Uuid;

use crate::commands::poll::Poll;

pub fn embed(
    name: String,
    created_by: UserId,
    description: Option<String>,
    id: Option<Uuid>,
) -> CommandResult<CreateEmbed> {
    let mut embed = CreateEmbed::default();
    embed
        .title(name)
        .description(t!("commands.poll.setup.embed.description").as_str());

    // first row (id, status, user)
    embed.field(
        "ID",
        id.map_or(t!("commands.poll.setup.embed.id_none"), |id| id.to_string()),
        true,
    );
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

impl Poll {
    pub fn update_message(&self, ctx: Context) {}
}
