use bostil_core::{
    arguments::{ArgumentsLevel, CommandFnArguments},
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
    async fn run<'a>(&self, arguments: CommandFnArguments) -> CommandResult<'a> {
        let ctx = arguments
            .get(&ArgumentsLevel::Context)
            .unwrap()
            .downcast_ref::<Context>()
            .unwrap();
        let guild = arguments
            .get(&ArgumentsLevel::Guild)
            .unwrap()
            .downcast_ref::<Guild>()
            .unwrap();
        let user = arguments
            .get(&ArgumentsLevel::User)
            .unwrap()
            .downcast_ref::<User>()
            .unwrap();
        let options = arguments
            .get(&ArgumentsLevel::Options)
            .unwrap()
            .downcast_ref::<Vec<CommandDataOption>>()
            .unwrap();

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
