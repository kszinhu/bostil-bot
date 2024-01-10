use serenity::builder::CreateCommand;
use serenity::{all::CommandOptionType, async_trait};

use super::{
    ArgumentsLevel, Command, CommandCategory, CommandResponse, InternalCommandResult, RunnerFn,
};

struct Language;

#[async_trait]
impl RunnerFn for Language {
    async fn run<'a>(
        &self,
        args: &Vec<Box<dyn std::any::Any + Send + Sync>>,
    ) -> InternalCommandResult<'a> {
        Ok(CommandResponse::String("".to_string()))
    }
}

pub fn register(command: &mut CreateCommand) -> &mut CreateCommand {
    command
        .name("language")
        .name_localized("pt-BR", "idioma")
        .description("Language Preferences Menu")
        .description_localized("pt-BR", "Menu de preferências de idioma")
        .create_option(|option| {
            option
                .name("choose_language")
                .name_localized("pt-BR", "alterar_idioma")
                .description("Choose the language of preference")
                .description_localized("pt-BR", "Escolha o idioma de preferência")
                .kind(CommandOptionType::String)
                .add_string_choice_localized(
                    "Portuguese",
                    "pt-BR",
                    [("pt-BR", "Português"), ("en-US", "Portuguese")],
                )
                .add_string_choice_localized(
                    "English",
                    "en-US",
                    [("pt-BR", "Inglês"), ("en-US", "English")],
                )
        })
}

pub fn get_command() -> Command {
    Command::new(
        "language",
        "Language Preferences Menu",
        CommandCategory::General,
        vec![ArgumentsLevel::Options, ArgumentsLevel::Guild],
        Box::new(Language {}),
    )
}
