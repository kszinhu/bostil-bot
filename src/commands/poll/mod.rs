use self::utils::progress_bar;
use super::{
    ArgumentsLevel, Command, CommandCategory, CommandResponse, InternalCommandResult, RunnerFn,
};
use crate::{components::button::Button, internal::debug::log_message};

use regex::Regex;
use rust_i18n::t;
use serenity::{
    async_trait,
    builder::{CreateEmbed, CreateMessage, EditInteractionResponse},
    framework::standard::CommandResult,
    model::{
        prelude::{
            application_command::{CommandDataOption, CommandDataOptionValue},
            component::ButtonStyle,
            UserId,
        },
        user::User,
    },
};
use std::{
    borrow::BorrowMut,
    time::{Duration, SystemTime},
};

mod database;
pub mod help;
pub mod management;
pub mod setup;
mod utils;

struct PollCommand;

#[derive(Debug)]
pub struct Vote {
    pub user_id: UserId,
    pub options: Vec<String>,
}

#[derive(Debug, Clone, Copy)]
pub enum PollType {
    SingleChoice,
    MultipleChoice,
}

#[derive(Debug, Clone, Copy)]
pub enum PollStatus {
    Open,
    Closed,
    Stopped,
}

#[derive(Debug)]
pub struct Poll {
    id: uuid::Uuid,
    name: String,
    description: Option<String>,
    kind: PollType,
    options: Vec<String>,
    timer: Duration,
    status: PollStatus,
}

#[derive(Debug)]
pub struct PollDatabaseModel {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: Option<String>,
    pub kind: PollType,
    pub status: PollStatus,
    pub options: Vec<String>,
    pub timer: Duration,
    pub votes: Vec<Vote>,
    pub created_at: SystemTime,
    pub created_by: UserId,
}

impl Poll {
    pub fn new(
        name: String,
        description: Option<String>,
        kind: PollType,
        options: Vec<String>,
        // Receives a minute value as a string (e.g. "0.5" for 30 seconds, "1" for 1 minute, "2" for 2 minutes, etc.)
        timer: Option<String>,
        status: Option<PollStatus>,
    ) -> Poll {
        Poll {
            name,
            description,
            kind,
            options,
            id: uuid::Uuid::new_v4(),
            status: status.unwrap_or(PollStatus::Open),
            timer: match timer {
                Some(timer) => {
                    let timer = timer.parse::<f64>().unwrap_or(0.0);
                    Duration::from_secs_f64(timer * 60.0)
                }
                None => Duration::from_secs(60),
            },
        }
    }
}

impl PollType {
    pub fn to_string(&self) -> String {
        match self {
            PollType::SingleChoice => t!("commands.poll.types.single_choice"),
            PollType::MultipleChoice => t!("commands.poll.types.multiple_choice"),
        }
    }

    pub fn to_label(&self) -> String {
        // TODO: add i18n
        match self {
            PollType::SingleChoice => "Single Choice".to_string(),
            PollType::MultipleChoice => "Multiple Choice".to_string(),
        }
    }
}

impl PollStatus {
    pub fn to_string(&self) -> String {
        match self {
            PollStatus::Open => "open".to_string(),
            PollStatus::Closed => "closed".to_string(),
            PollStatus::Stopped => "stopped".to_string(),
        }
    }
}

fn poll_serializer(command_options: &Vec<CommandDataOption>) -> Poll {
    let option_regex: Regex = Regex::new(r"^option_\d+$").unwrap();
    let kind = match command_options.iter().find(|option| option.name == "type") {
        Some(option) => match option.resolved.as_ref().unwrap() {
            CommandDataOptionValue::String(value) => match value.as_str() {
                "single_choice" => PollType::SingleChoice,
                "multiple_choice" => PollType::MultipleChoice,
                _ => PollType::SingleChoice,
            },
            _ => PollType::SingleChoice,
        },
        None => PollType::SingleChoice,
    };

    Poll::new(
        command_options
            .iter()
            .find(|option| option.name == "name")
            .unwrap()
            .value
            .as_ref()
            .unwrap()
            .to_string(),
        Some(
            command_options
                .iter()
                .find(|option| option.name == "description")
                .unwrap()
                .value
                .as_ref()
                .unwrap()
                .to_string(),
        ),
        kind,
        command_options
            .iter()
            .filter(|option| option_regex.is_match(&option.name))
            .map(|option| match option.resolved.as_ref().unwrap() {
                CommandDataOptionValue::String(value) => value.to_string(),
                _ => "".to_string(),
            })
            .collect::<Vec<String>>(),
        Some(
            command_options
                .iter()
                .find(|option| option.name == "timer")
                .unwrap()
                .value
                .as_ref()
                .unwrap()
                .to_string(),
        ),
        Some(PollStatus::Open),
    )
}

fn create_message(
    mut message_builder: EditInteractionResponse,
    poll: PollDatabaseModel,
) -> CommandResult<EditInteractionResponse> {
    let time_remaining = match poll.timer.as_secs() / 60 > 1 {
        true => format!("{} minutes", poll.timer.as_secs() / 60),
        false => format!("{} seconds", poll.timer.as_secs()),
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

//  TODO: timer to close poll
// fn create_interaction() {
//         // Wait for multiple interactions
//         let mut interaction_stream =
//         m.await_component_interactions(&ctx).timeout(Duration::from_secs(60 * 3)).build();

//     while let Some(interaction) = interaction_stream.next().await {
//         let sound = &interaction.data.custom_id;
//         // Acknowledge the interaction and send a reply
//         interaction
//             .create_interaction_response(&ctx, |r| {
//                 // This time we dont edit the message but reply to it
//                 r.kind(InteractionResponseType::ChannelMessageWithSource)
//                     .interaction_response_data(|d| {
//                         // Make the message hidden for other users by setting `ephemeral(true)`.
//                         d.ephemeral(true)
//                             .content(format!("The **{}** says __{}__", animal, sound))
//                     })
//             })
//             .await
//             .unwrap();
//     }
//     m.delete(&ctx).await?;
// }

#[async_trait]
impl RunnerFn for PollCommand {
    async fn run(&self, args: &Vec<Box<dyn std::any::Any + Send + Sync>>) -> InternalCommandResult {
        let debug = std::env::var("DEBUG").is_ok();
        let options = args
            .iter()
            .filter_map(|arg| arg.downcast_ref::<Vec<CommandDataOption>>())
            .collect::<Vec<&Vec<CommandDataOption>>>();

        let user_id = args
            .iter()
            .filter_map(|arg| arg.downcast_ref::<User>())
            .collect::<Vec<&User>>()
            .get(0)
            .unwrap()
            .id;

        let poll = poll_serializer(options.get(0).unwrap());

        if debug {
            log_message(
                format!("{:?}", poll).as_str(),
                crate::internal::debug::MessageTypes::Debug,
            );
        }

        let message = create_message(
            EditInteractionResponse::default(),
            PollDatabaseModel::from(&poll, vec![], &user_id),
        )
        .unwrap();

        Ok(CommandResponse::Message(message))
    }
}

pub fn get_command() -> Command {
    Command::new(
        "poll",
        "Poll commands",
        CommandCategory::Misc,
        vec![ArgumentsLevel::Options, ArgumentsLevel::User],
        Box::new(PollCommand),
    )
}
