use bostil_core::embeds::{ApplicationEmbed, EmbedLifetime};
use once_cell::sync::Lazy;
use serenity::builder::CreateEmbed;
use uuid::Uuid;

use crate::{
    modules::{
        app::commands::poll::PollStage,
        core::{entities::poll::Poll, helpers::establish_connection},
    },
    schema::polls,
};

struct PollVoteEmbed;

impl EmbedLifetime for PollVoteEmbed {
    fn build(&self, arguments: &Vec<Box<dyn std::any::Any + Send + Sync>>) -> CreateEmbed {
        use crate::diesel::{QueryDsl, RunQueryDsl, SelectableHelper};

        let poll_id = arguments[0].downcast_ref::<Uuid>().unwrap();
        let stage = arguments[1].downcast_ref::<PollStage>().unwrap();

        let connection = &mut establish_connection();
        let poll = polls::table
            .find(poll_id)
            .select(Poll::as_select())
            .first::<Poll>(connection)
            .expect("Error loading poll");

        CreateEmbed::default()
    }
}

pub static VOTE_EMBED: Lazy<ApplicationEmbed> = Lazy::new(|| {
    ApplicationEmbed::new(
        "Poll Voting embed",
        Some("Embed to choose an choice in a poll"),
        Some("Selecione uma opção para votar"),
        vec![
            Box::new(None::<Option<Uuid>>),
            Box::new(None::<Option<PollStage>>),
        ],
        Box::new(PollVoteEmbed),
        None,
        None,
        None,
    )
});

// pub fn embed(
//     mut message_builder: EditInteractionResponse,
//     poll: Poll,
// ) -> CommandResult<EditInteractionResponse> {
//     let time_remaining = match poll.timer.is_some() {
//         true => {
//             let time_remaining = poll.timer.unwrap() - poll.started_at.unwrap().timestamp();
//             let minutes = time_remaining / 60;
//             let seconds = time_remaining % 60;

//             format!("{}m {}s", minutes, seconds)
//         }
//         false => "∞".to_string(),
//     };
//     let mut embed = CreateEmbed::default();
//     embed
//         .title(poll.name)
//         .description(poll.description.unwrap_or("".to_string()));

//     // first row (id, status, user)
//     embed.field(
//         "ID",
//         format!("`{}`", poll.id.to_string().split_at(8).0),
//         true,
//     );
//     embed.field("Status", poll.status.to_string(), true);
//     embed.field("User", format!("<@{}>", poll.created_by), true);

//     // separator
//     embed.field("\u{200B}", "\u{200B}", false);

//     poll.options.iter().for_each(|option| {
//         embed.field(
//             option.value.clone(),
//             format!("{} votes", option.votes.len()),
//             true,
//         );
//     });

//     // separator
//     embed.field("\u{200B}", "\u{200B}", false);

//     embed.field(
//         "Partial Results (Live)",
//         format!("```diff\n{}\n```", progress_bar(poll.options.clone())),
//         false,
//     );

//     // separator
//     embed.field("\u{200B}", "\u{200B}", false);

//     embed.field(
//         "Time remaining",
//         format!("{} remaining", time_remaining),
//         false,
//     );

//     message_builder.set_embed(embed);
//     message_builder.components(|component| {
//         component.create_action_row(|action_row| {
//             poll.options.iter().for_each(|option| {
//                 action_row.add_button(
//                     Button::new(
//                         option.value.as_str(),
//                         option.value.as_str(),
//                         ButtonStyle::Primary,
//                         None,
//                     )
//                     .create(),
//                 );
//             });

//             action_row
//         })
//     });

//     Ok(message_builder)
// }
