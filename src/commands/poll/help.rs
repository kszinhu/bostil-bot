use rust_i18n::t;
use serenity::{
    async_trait, builder::CreateApplicationCommandOption,
    model::prelude::command::CommandOptionType,
};

use crate::{
    commands::{
        ArgumentsLevel, Command, CommandCategory, CommandResponse, InternalCommandResult, RunnerFn,
    },
    internal::constants::CommandHelp,
};

use super::PollType;

/**
 * Command: help
 *
 * Return the help message for the poll command
 * - Usage: /poll help
 */

struct PollHelpCommand;

#[async_trait]
impl RunnerFn for PollHelpCommand {
    async fn run(&self, _: &Vec<Box<dyn std::any::Any + Send + Sync>>) -> InternalCommandResult {
        let mut help_message: String = "```".to_string();

        for helper in collect_command_help() {
            help_message.push_str(&format!("/poll {} {}\n", helper.name, helper.description));

            for option in helper.options {
                help_message.push_str(&format!("  {}\n", option));
            }

            help_message.push_str("\n");
        }

        help_message.push_str("```");

        Ok(CommandResponse::String(help_message))
    }
}

fn create_help() -> CommandHelp {
    CommandHelp {
        name: "poll".to_string(),
        description: "Create a poll".to_string(),
        options: vec![
            "name: The name of the poll".to_string(),
            "description: The description of the poll".to_string(),
            format!(
                "type: The type of the poll ({} or {})",
                PollType::SingleChoice.to_label(),
                PollType::MultipleChoice.to_label()
            ),
            "options: It is a voting option".to_string(),
        ],
    }
}

fn setup_help() -> CommandHelp {
    CommandHelp {
        name: "setup".to_string(),
        description: "Setup the poll".to_string(),
        options: vec![
            format!(
                "type: The type of the poll
            {}: {}
            {}: {}
        ",
                PollType::SingleChoice.to_string(),
                PollType::SingleChoice.to_label(),
                PollType::MultipleChoice.to_string(),
                PollType::MultipleChoice.to_label(),
            ),
            "channel: The channel of the poll
            \"current\": The current channel
            \"<channel_id>\": The channel id
        "
            .to_string(),
            "timer: Optional, the timer of the poll".to_string(),
        ],
    }
}

fn management_help() -> CommandHelp {
    CommandHelp {
        name: "management".to_string(),
        description: "Manage the poll".to_string(),
        options: vec![
            "status: The status of the poll
            \"open\": Open the poll
            \"close\": Close the poll
            \"stop\": Stop the poll
        "
            .to_string(),
            "info: The info of the poll
            \"name\": The name of the poll
            \"description\": The description of the poll
            \"type\": The type of the poll
            \"options\": The options of the poll
            \"timer\": The timer of the poll
            \"status\": The status of the poll
            \"votes\": The votes of the poll (only available for closed polls)
            \"created_at\": The created at of the poll
            \"created_by\": The created by of the poll
        "
            .to_string(),
        ],
    }
}

fn collect_command_help() -> Vec<CommandHelp> {
    vec![create_help(), setup_help(), management_help()]
}

pub fn register_option<'a>() -> CreateApplicationCommandOption {
    let mut command_option = CreateApplicationCommandOption::default();

    command_option
        .name("help")
        .name_localized("pt-BR", "ajuda")
        .description("Show the help message for poll commands")
        .description_localized(
            "pt-BR",
            "Mostra a mensagem de ajuda para os comandos de votação",
        )
        .kind(CommandOptionType::SubCommand)
        .create_sub_option(|sub_option| {
            sub_option
                .name("poll_command")
                .name_localized("pt-BR", "comando_de_votação")
                .description("The command to show the help message for poll commands")
                .description_localized(
                    "pt-BR",
                    "O comando para mostrar a mensagem de ajuda para os comandos de votação",
                )
                .kind(CommandOptionType::String)
                .required(true)
                .add_string_choice(t!("commands.poll.setup.label"), "setup_command")
                .add_string_choice(t!("commands.poll.management.label"), "management_command")
        });

    command_option
}

pub fn get_command() -> Command {
    Command::new(
        "help",
        "Show the help message for poll commands",
        CommandCategory::Misc,
        vec![ArgumentsLevel::None],
        Box::new(PollHelpCommand {}),
    )
}
