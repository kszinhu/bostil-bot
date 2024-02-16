use bostil_core::{
    arguments::ArgumentsLevel,
    commands::{Command, CommandCategory, CommandContext},
    runners::runners::{CommandResponse, CommandResult, CommandRunnerFn},
};
use lazy_static::lazy_static;
use serenity::{
    all::{CommandDataOption, CommandOptionType, Guild, User},
    async_trait,
    builder::{CreateCommand, CreateCommandOption},
    prelude::Context,
};

use crate::modules::core::actions::voice::{mute, unmute};

#[derive(Clone)]
struct MuteCommand;

#[async_trait]
impl CommandRunnerFn for MuteCommand {
    async fn run<'a>(&self, args: &Vec<Box<dyn std::any::Any + Send + Sync>>) -> CommandResult<'a> {
        let ctx = args
            .iter()
            .filter_map(|arg| arg.downcast_ref::<Context>())
            .collect::<Vec<&Context>>()[0];
        let guild = args
            .iter()
            .filter_map(|arg| arg.downcast_ref::<Guild>())
            .collect::<Vec<&Guild>>()[0];
        let user = args
            .iter()
            .filter_map(|arg| arg.downcast_ref::<User>())
            .collect::<Vec<&User>>()[0];
        let options = args
            .iter()
            .filter_map(|arg| arg.downcast_ref::<Vec<CommandDataOption>>())
            .collect::<Vec<&Vec<CommandDataOption>>>()[0];

        let enable_sound = options
            .iter()
            .filter(|option| option.name == "enable_sound")
            .collect::<Vec<&CommandDataOption>>()[0]
            .value
            .clone();

        match enable_sound.as_bool().unwrap() {
            true => match unmute(ctx, guild, &user.id).await {
                Ok(_) => Ok(CommandResponse::None),
                Err(_) => Ok(CommandResponse::None),
            },
            false => match mute(ctx, guild, &user.id).await {
                Ok(_) => Ok(CommandResponse::None),
                Err(_) => Ok(CommandResponse::None),
            },
        }
    }
}

lazy_static! {
    pub static ref MUTE_COMMAND: Command = Command::new(
        "mute",
        "Disable sound from a bot",
        CommandContext::Guild,
        CommandCategory::Voice,
        vec![
            ArgumentsLevel::Context,
            ArgumentsLevel::Guild,
            ArgumentsLevel::User,
        ],
        Box::new(MuteCommand),
        Some(
            CreateCommand::new("mute")
                .name_localized("pt-BR", "silenciar")
                .description("Disable sound from a bot")
                .description_localized("pt-BR", "Mute o bot")
                .add_option(
                    CreateCommandOption::new(
                        CommandOptionType::Boolean,
                        "enable_sound",
                        "Enable sound",
                    )
                    .name_localized("pt-BR", "habilitar_som")
                    .description_localized("pt-BR", "Habilitar o som do bot")
                    .required(true),
                ),
        ),
    );
}
