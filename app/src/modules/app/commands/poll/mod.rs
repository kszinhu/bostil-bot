use bostil_core::{
    arguments::{ArgumentsLevel, CommandFnArguments},
    commands::{Command, CommandCategory, CommandContext},
    runners::runners::{CommandResult, CommandRunnerFn},
};
use lazy_static::lazy_static;
use serenity::{
    all::{CommandData, CommandDataOption, User},
    async_trait,
    builder::CreateCommand,
    model::Colour,
};
use tracing::debug;

mod embeds;
mod progress_bar;
mod setup;

#[derive(Clone)]
struct PollCommand;

#[derive(Debug, Clone, Copy)]
pub enum PollStage {
    Setup,
    Voting,
    Closed,
}

impl PollStage {
    pub fn embed_color(&self) -> Colour {
        match self {
            PollStage::Setup => Colour::ORANGE,
            PollStage::Voting => Colour::RED,
            PollStage::Closed => Colour::DARK_GREEN,
        }
    }
}

#[async_trait]
impl CommandRunnerFn for PollCommand {
    async fn run<'a>(&self, arguments: CommandFnArguments) -> CommandResult<'a> {
        let options = arguments
            .get(&ArgumentsLevel::Options)
            .expect("No has provided option argument")
            .downcast_ref::<Vec<CommandDataOption>>()
            .expect("Error on casting options to Vec<CommandDataOption>");

        let command_name = options.first().unwrap().name.clone();

        debug!(
            "Running {} command with options \n{:?}",
            command_name, options
        );

        let command_runner = command_suite(command_name);

        let response = command_runner.run(arguments);

        response.await
    }
}

fn command_suite(command_name: String) -> &'static Box<dyn CommandRunnerFn + Send + Sync> {
    debug!("Running command suite for {}", command_name);

    let command_runner = match command_name.as_str() {
        "setup" => &setup::SETUP_COMMAND.runner,
        _ => {
            panic!("Command not found");
        }
    };

    command_runner
}

lazy_static! {
    pub static ref POLL_COMMANDS: Command = Command::new(
        "poll",
        "Poll commands",
        CommandContext::Guild,
        CommandCategory::Misc,
        vec![
            ArgumentsLevel::User,
            ArgumentsLevel::Options,
            ArgumentsLevel::Context,
            ArgumentsLevel::Guild,
            ArgumentsLevel::ChannelId,
        ],
        Box::new(PollCommand),
        Some(
            CreateCommand::new("poll")
                .name_localized("pt-BR", "urna")
                .description("Create and manage polls")
                .description_localized("pt-BR", "Crie e administre enquetes")
                .add_option(setup::SETUP_OPTION.clone()),
        ),
    );
}
