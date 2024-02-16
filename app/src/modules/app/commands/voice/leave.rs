use bostil_core::{
    arguments::ArgumentsLevel,
    commands::{Command, CommandCategory, CommandContext},
    runners::runners::{CommandResponse, CommandResult, CommandRunnerFn},
};
use lazy_static::lazy_static;
use serenity::{
    async_trait,
    builder::CreateCommand,
    model::{prelude::Guild, user::User},
    prelude::Context,
};

use crate::modules::core::actions::voice::leave;

#[derive(Clone)]
struct LeaveCommand;

#[async_trait]
impl CommandRunnerFn for LeaveCommand {
    async fn run<'a>(&self, args: &Vec<Box<dyn std::any::Any + Send + Sync>>) -> CommandResult<'a> {
        let ctx = args
            .iter()
            .filter_map(|arg| arg.downcast_ref::<Context>())
            .collect::<Vec<&Context>>();
        let guild = args
            .iter()
            .filter_map(|arg| arg.downcast_ref::<Guild>())
            .collect::<Vec<&Guild>>();
        let user_id = args
            .iter()
            .filter_map(|arg| arg.downcast_ref::<User>())
            .collect::<Vec<&User>>()
            .get(0)
            .unwrap()
            .id;

        match leave(ctx.get(0).unwrap(), guild.get(0).unwrap(), &user_id).await {
            Ok(_) => Ok(CommandResponse::None),
            Err(_) => Ok(CommandResponse::None),
        }
    }
}

lazy_static! {
    pub static ref LEAVE_COMMAND: Command = Command::new(
        "leave",
        "Leave the voice channel you are in",
        CommandContext::Guild,
        CommandCategory::Voice,
        vec![
            ArgumentsLevel::Context,
            ArgumentsLevel::Guild,
            ArgumentsLevel::User,
        ],
        Box::new(LeaveCommand {}),
        Some(
            CreateCommand::new("leave")
                .name_localized("pt-BR", "sair")
                .description("Leave the voice channel you are in")
                .description_localized("pt-BR", "Sai do canal de voz que você está"),
        ),
    );
}
