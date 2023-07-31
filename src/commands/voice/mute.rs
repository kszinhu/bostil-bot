use crate::events::voice::{mute, unmute};

use serenity::{
    builder::CreateApplicationCommand,
    framework::standard::CommandResult,
    model::prelude::{
        command::CommandOptionType, interaction::application_command::CommandDataOption, Guild,
        UserId,
    },
    prelude::Context,
};

pub async fn run(
    ctx: &Context,
    guild: &Guild,
    user_id: &UserId,
    options: &Vec<CommandDataOption>,
) -> CommandResult<String> {
    let enable_sound = options
        .get(0)
        .unwrap()
        .value
        .as_ref()
        .unwrap()
        .as_bool()
        .unwrap();

    match enable_sound {
        true => unmute(ctx, guild, user_id).await,
        false => mute(ctx, guild, user_id).await,
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("mute")
        .name_localized("pt-BR", "silenciar")
        .description("Disable sound from a bot")
        .description_localized("pt-BR", "Mute o bot")
        .create_option(|option| {
            option
                .name("enable_sound")
                .name_localized("pt-BR", "habilitar_som")
                .description("Enable sound")
                .description_localized("pt-BR", "Habilitar som")
                .kind(CommandOptionType::Boolean)
        })
}
