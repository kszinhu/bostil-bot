use crate::database::locale::apply_locale;
use rust_i18n::{locale as current_locale, t};

use serenity::async_trait;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::{
    command, interaction::application_command::CommandDataOption, Guild,
};

use super::{
    ArgumentsLevel, Command, CommandCategory, CommandResponse, InternalCommandResult, RunnerFn,
};

struct Language;

#[async_trait]
impl RunnerFn for Language {
    async fn run(&self, args: &Vec<Box<dyn std::any::Any + Send + Sync>>) -> InternalCommandResult {
        let options = args
            .iter()
            .filter_map(|arg| arg.downcast_ref::<Vec<CommandDataOption>>())
            .collect::<Vec<&Vec<CommandDataOption>>>()[0];
        let guild = args
            .iter()
            .filter_map(|arg| arg.downcast_ref::<Guild>())
            .collect::<Vec<&Guild>>()[0];

        if let Some(language_option) = options.get(0) {
            let selected_language = language_option.value.as_ref().unwrap().as_str().unwrap();

            apply_locale(selected_language, &guild.id, false);

            let current_locale_name = t!(&format!("commands.language.{}", selected_language));
            Ok(CommandResponse::String(
                t!("commands.language.reply", "language_name" => current_locale_name),
            ))
        } else {
            let current_locale_name = t!(&format!("commands.language.{}", current_locale()));
            Ok(CommandResponse::String(
                t!("commands.language.current_language", "language_name" => current_locale_name, "language_code" => current_locale()),
            ))
        }
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
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
                .kind(command::CommandOptionType::String)
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
