use bostil_core::{
    arguments::CommandFnArguments,
    commands::{Command, CommandCategory, CommandContext},
    runners::runners::{CommandResponse, CommandResult, CommandRunnerFn},
};
use lazy_static::lazy_static;
use serenity::{
    all::CommandOptionType,
    async_trait,
    builder::{CreateCommand, CreateCommandOption},
};

#[derive(Clone)]
struct Language;

#[async_trait]
impl CommandRunnerFn for Language {
    async fn run<'a>(&self, _: CommandFnArguments) -> CommandResult<'a> {
        Ok(CommandResponse::String("".to_string()))
    }
}

lazy_static! {
    /// Command to set the language of bot responses within a guild
    pub static ref LANGUAGE_COMMAND: Command = Command::new(
        "language",
        "Sets the language of the bot",
        CommandContext::Guild,
        CommandCategory::General,
        vec![],
        Box::new(Language),
        Some(
            CreateCommand::new("language")
                .description("Language Preferences Menu")
                .add_option(
                    CreateCommandOption::new(
                        CommandOptionType::String,
                        "choose_language",
                        "Choose the language of preference"
                    )
                    .add_string_choice("Portuguese", "pt-BR")
                    .add_string_choice("English", "en-US")
                    .required(true)
                ),
        ),
    );
}
