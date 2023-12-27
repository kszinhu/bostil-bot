use serenity::{
    builder::{CreateEmbed, EditInteractionResponse},
    framework::standard::CommandResult,
    model::prelude::component::ButtonStyle,
};

use crate::{
    commands::poll::{utils::progress_bar, PollDatabaseModel as Poll},
    components::button::Button,
};

pub fn embed(
    mut message_builder: EditInteractionResponse,
    poll: Poll,
) -> CommandResult<EditInteractionResponse> {
    let time_remaining = match poll.timer.unwrap().as_secs() / 60 > 1 {
        true => format!("{} minutes", poll.timer.unwrap().as_secs() / 60),
        false => format!("{} seconds", poll.timer.unwrap().as_secs()),
    };
    let mut embed = CreateEmbed::default();
    embed
        .title(poll.name)
        .description(poll.description.unwrap_or("".to_string()));

    // first row (id, status, user)
    embed.field(
        "ID",
        format!("`{}`", poll.id.to_string().split_at(8).0),
        true,
    );
    embed.field("Status", poll.status.to_string(), true);
    embed.field("User", format!("<@{}>", poll.created_by), true);

    // separator
    embed.field("\u{200B}", "\u{200B}", false);

    poll.options.iter().for_each(|option| {
        embed.field(option, option, false);
    });

    // separator
    embed.field("\u{200B}", "\u{200B}", false);

    embed.field(
        "Partial Results (Live)",
        format!(
            "```diff\n{}\n```",
            progress_bar(poll.votes, poll.options.clone())
        ),
        false,
    );

    // separator
    embed.field("\u{200B}", "\u{200B}", false);

    embed.field(
        "Time remaining",
        format!("{} remaining", time_remaining),
        false,
    );

    message_builder.set_embed(embed);
    message_builder.components(|component| {
        component.create_action_row(|action_row| {
            poll.options.iter().for_each(|option| {
                action_row
                    .add_button(Button::new(option, option, ButtonStyle::Primary, None).create());
            });

            action_row
        })
    });

    Ok(message_builder)
}
