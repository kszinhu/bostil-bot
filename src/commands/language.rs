use crate::database::locale::apply_locale;
use rust_i18n::{locale as current_locale, t};

use serenity::builder::CreateApplicationCommand;
use serenity::client::Context;
use serenity::model::prelude::{
    command, interaction::application_command::CommandDataOption, GuildId,
};

pub async fn run(options: &Vec<CommandDataOption>, _ctx: &Context, guild_id: &GuildId) -> String {
    if let Some(language_option) = options.get(0) {
        let selected_language = language_option.value.as_ref().unwrap().as_str().unwrap();

        apply_locale(selected_language, &guild_id, false);

        let current_locale_name = t!(&format!(
            "commands.language.{}",
            language_option.value.as_ref().unwrap()
        ));
        t!("commands.language.reply", "language_name" => current_locale_name)
    } else {
        let current_locale_name = t!(&format!("commands.language.{}", current_locale()));
        t!("commands.language.current_language", "language_code" => current_locale(), "language_name" => current_locale_name)
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
