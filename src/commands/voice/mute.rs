use crate::{
    commands::{
        ArgumentsLevel, Command, CommandCategory, CommandResponse, InternalCommandResult, RunnerFn,
    },
    events::voice::{mute, unmute},
};

use serenity::{
    async_trait,
    builder::CreateApplicationCommand,
    framework::standard::CommandResult,
    model::prelude::{
        command::CommandOptionType, interaction::application_command::CommandDataOption, Guild,
        UserId,
    },
    prelude::Context,
};

struct MuteCommand;

#[async_trait]
impl RunnerFn for MuteCommand {
    async fn run(&self, args: &Vec<Box<dyn std::any::Any + Send + Sync>>) -> InternalCommandResult {
        let ctx = args
            .iter()
            .filter_map(|arg| arg.downcast_ref::<Context>())
            .collect::<Vec<&Context>>()[0];
        let guild = args
            .iter()
            .filter_map(|arg| arg.downcast_ref::<Guild>())
            .collect::<Vec<&Guild>>()[0];
        let user_id = args
            .iter()
            .filter_map(|arg| arg.downcast_ref::<UserId>())
            .collect::<Vec<&UserId>>()[0];
        let options = args
            .iter()
            .filter_map(|arg| arg.downcast_ref::<Vec<CommandDataOption>>())
            .collect::<Vec<&Vec<CommandDataOption>>>()[0];

        let enable_sound = options
            .get(0)
            .unwrap()
            .value
            .as_ref()
            .unwrap()
            .as_bool()
            .unwrap();

        match enable_sound {
            true => match unmute(ctx, guild, user_id).await {
                Ok(_) => Ok(CommandResponse::None),
                Err(_) => Ok(CommandResponse::None),
            },
            false => match mute(ctx, guild, user_id).await {
                Ok(_) => Ok(CommandResponse::None),
                Err(_) => Ok(CommandResponse::None),
            },
        }
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

pub fn get_command() -> Command {
    Command::new(
        "mute",
        "Disable sound from a bot",
        CommandCategory::Voice,
        vec![
            ArgumentsLevel::Context,
            ArgumentsLevel::Guild,
            ArgumentsLevel::User,
        ],
        Box::new(MuteCommand {}),
    )
}
