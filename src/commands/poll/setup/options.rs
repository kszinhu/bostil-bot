use serenity::{
    builder::CreateApplicationCommandOption, model::prelude::command::CommandOptionType,
};

pub fn register_option<'a>() -> CreateApplicationCommandOption {
    let mut command_option = CreateApplicationCommandOption::default();

    command_option
        .name("options")
        .name_localized("pt-BR", "opções")
        .description("Add options to the poll")
        .description_localized("pt-BR", "Adiciona opções à votação")
        .kind(CommandOptionType::SubCommand)
        .create_sub_option(|sub_option| {
            sub_option
                .name("poll_id")
                .name_localized("pt-BR", "id_da_votação")
                .description("The poll id")
                .description_localized("pt-BR", "O id da votação")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_sub_option(|sub_option| {
            sub_option
                .name("option_name")
                .name_localized("pt-BR", "nome_da_opção")
                .description("The name of the option (max 25 characters)")
                .description_localized("pt-BR", "O nome da opção (máx 25 caracteres)")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_sub_option(|sub_option| {
            sub_option
                .name("option_description")
                .name_localized("pt-BR", "descrição_da_opção")
                .description("The description of the option (max 100 characters)")
                .description_localized("pt-BR", "A descrição da votação")
                .kind(CommandOptionType::String)
                .required(true)
        });

    command_option
}
