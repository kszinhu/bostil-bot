use serenity::{
    builder::CreateApplicationCommandOption, model::prelude::command::CommandOptionType,
};

pub fn register_option<'a>() -> CreateApplicationCommandOption {
    let mut command_option = CreateApplicationCommandOption::default();

    command_option
        .name("setup")
        .name_localized("pt-BR", "configurar")
        .description("Setup a poll")
        .description_localized("pt-BR", "Configura uma votação")
        .kind(CommandOptionType::SubCommand)
        .create_sub_option(|sub_option| {
            sub_option
                .name("poll_name")
                .name_localized("pt-BR", "nome_da_votação")
                .description("The name of the option (max 25 characters)")
                .description_localized("pt-BR", "O nome da opção (máx 25 caracteres)")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_sub_option(|sub_option| {
            sub_option
                .name("poll_description")
                .name_localized("pt-BR", "descrição_da_votação")
                .description("The description of the option (max 100 characters)")
                .description_localized("pt-BR", "A descrição da votação")
                .kind(CommandOptionType::String)
                .required(true)
        });

    command_option
}
