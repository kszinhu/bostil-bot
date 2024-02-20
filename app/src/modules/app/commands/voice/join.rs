use bostil_core::{
    arguments::ArgumentsLevel,
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
