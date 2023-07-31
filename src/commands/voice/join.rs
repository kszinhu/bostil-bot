use crate::events::voice::join;

use serenity::{
    builder::CreateApplicationCommand,
    framework::standard::CommandResult,
    model::prelude::{interaction::application_command::CommandDataOption, Guild, UserId},
    prelude::Context,
};

pub async fn run(
    ctx: &Context,
    guild: &Guild,
    user_id: &UserId,
    _options: &Vec<CommandDataOption>,
) -> CommandResult<String> {
    join(ctx, guild, user_id).await
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("join")
        .name_localized("pt-BR", "entrar")
        .description("Join the voice channel you are in")
        .description_localized("pt-BR", "Entra no canal de voz que você está")
}
