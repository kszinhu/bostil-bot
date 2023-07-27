use rust_i18n::{available_locales, set_locale, t};

use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

pub async fn run(options: &Vec<CommandDataOption>) -> String {
    let language = options[0].value.as_ref().unwrap().as_str().unwrap();

    if !available_locales!().contains(&language) {
        return t!("commands.language.invalid_language");
    }
    set_locale(language);

    t!("commands.language.reply", "language_code" => t!(&format!("commands.language.{}", language)))
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("language")
        .name_localized("pt-BR", "idioma")
        .description("Change the bot language")
        .description_localized("pt-BR", "Altere o idioma do bot")
        .create_option(|option| {
            option
                .name("language")
                .description("The language to change to")
                .description_localized("pt-BR", "O idioma irá mudar para")
                .kind(command::CommandOptionType::String)
                .required(true)
                .add_string_choice("Portuguese", "pt")
                .add_string_choice("English", "en")
        })
}
