use bostil_core::{
    arguments::ArgumentsLevel,
    commands::{Command, CommandCategory, CommandContext},
    runners::runners::{CommandResult, CommandRunnerFn},
};
use lazy_static::lazy_static;
use serenity::{all::CommandDataOption, async_trait, builder::CreateCommand, model::Colour};

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
    async fn run<'a>(
        &self,
        args: &Vec<Box<dyn std::any::Any + Send + Sync>>,
    ) -> CommandResult<'a> {
        let options = args
            .iter()
            .filter_map(|arg| arg.downcast_ref::<Option<Vec<CommandDataOption>>>())
            .collect::<Vec<&Option<Vec<CommandDataOption>>>>()[0]
            .as_ref()
            .unwrap();
        let first_option = options.get(0).unwrap();
        let command_name = first_option.name.clone();

        let command_runner = command_suite(command_name);

        let response = command_runner.run(args);

        response.await
    }
}

fn command_suite(command_name: String) -> &'static Box<dyn CommandRunnerFn + Send + Sync> {
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
            ArgumentsLevel::Options,
            ArgumentsLevel::Context,
            ArgumentsLevel::Guild,
            ArgumentsLevel::User,
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
