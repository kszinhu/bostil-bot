use bostil_core::{
    arguments::{ArgumentsLevel, CommandFnArguments},
    commands::{Command, CommandCategory, CommandContext},
    runners::runners::{CommandResponse, CommandResult, CommandRunnerFn},
};
use lazy_static::lazy_static;
use serenity::{
    all::{Guild, User},
    async_trait,
    builder::CreateCommand,
    prelude::Context,
};

use crate::modules::core::actions::voice::join;

#[derive(Clone)]
struct JoinCommand;

#[async_trait]
impl CommandRunnerFn for JoinCommand {
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

        match join(ctx, guild, &user.id).await {
            Ok(_) => Ok(CommandResponse::String("Entrei capeta!".to_string())),
            Err(_) => Ok(CommandResponse::String("Dá não pai".to_string())),
        }
    }
}

lazy_static! {
    pub static ref JOIN_COMMAND: Command = Command::new(
        "join",
        "Join the voice channel you are in",
        CommandContext::Guild,
        CommandCategory::Voice,
        vec![
            ArgumentsLevel::Context,
            ArgumentsLevel::Guild,
            ArgumentsLevel::User,
        ],
        Box::new(JoinCommand {}),
        Some(
            CreateCommand::new("join")
                .name_localized("pt-BR", "entrar")
                .description("Join the voice channel you are in")
                .description_localized("pt-BR", "Entra no canal de voz que você está"),
        ),
    );
}
