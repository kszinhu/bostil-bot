use crate::events::voice::leave;

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
    leave(ctx, guild, user_id).await
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("leave")
        .name_localized("pt-BR", "sair")
        .description("Leave the voice channel you are in")
        .description_localized("pt-BR", "Sai do canal de voz que você está")
}
