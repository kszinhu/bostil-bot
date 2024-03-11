use bostil_core::{
    arguments::{ArgumentsLevel, CommandFnArguments},
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

        match leave(ctx, guild, &user.id).await {
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
