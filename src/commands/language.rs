use crate::database::locale::apply_locale;
use rust_i18n::t;

use serenity::builder::CreateApplicationCommand;
use serenity::client::Context;
use serenity::model::prelude::{
    command, interaction::application_command::CommandDataOption, GuildId,
};

pub async fn run(options: &Vec<CommandDataOption>, _ctx: &Context, guild_id: &GuildId) -> String {
    let language = options[0].value.as_ref().unwrap().as_str().unwrap();

    apply_locale(language, &guild_id);

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
