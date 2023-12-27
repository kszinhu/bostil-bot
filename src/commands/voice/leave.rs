use crate::{
    commands::{
        ArgumentsLevel, Command, CommandCategory, CommandResponse, InternalCommandResult, RunnerFn,
    },
    events::voice::leave,
};

use serenity::{
    async_trait,
    builder::CreateApplicationCommand,
    model::{prelude::Guild, user::User},
    prelude::Context,
};

struct LeaveCommand;

#[async_trait]
impl RunnerFn for LeaveCommand {
    async fn run<'a>(
        &self,
        args: &Vec<Box<dyn std::any::Any + Send + Sync>>,
    ) -> InternalCommandResult<'a> {
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

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("leave")
        .name_localized("pt-BR", "sair")
        .description("Leave the voice channel you are in")
        .description_localized("pt-BR", "Sai do canal de voz que você está")
}

pub fn get_command() -> Command {
    Command::new(
        "leave",
        "Leave the voice channel you are in",
        CommandCategory::Voice,
        vec![
            ArgumentsLevel::Context,
            ArgumentsLevel::Guild,
            ArgumentsLevel::User,
        ],
        Box::new(LeaveCommand {}),
    )
}
