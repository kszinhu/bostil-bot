use crate::{
    commands::{
        ArgumentsLevel, Command, CommandCategory, CommandResponse, InternalCommandResult, RunnerFn,
    },
    events::voice::join,
};
use serenity::{
    async_trait,
    builder::CreateApplicationCommand,
    model::prelude::{Guild, UserId},
    prelude::Context,
};

struct JoinCommand;

#[async_trait]
impl RunnerFn for JoinCommand {
    async fn run<'a>(&self, args: &Vec<Box<dyn std::any::Any + Send + Sync>>) -> InternalCommandResult<'a> {
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

        match join(ctx, guild, user_id).await {
            Ok(_) => Ok(CommandResponse::None),
            Err(_) => Ok(CommandResponse::None),
        }
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("join")
        .name_localized("pt-BR", "entrar")
        .description("Join the voice channel you are in")
        .description_localized("pt-BR", "Entra no canal de voz que você está")
}

pub fn get_command() -> Command {
    Command::new(
        "join",
        "Join the voice channel you are in",
        CommandCategory::Voice,
        vec![
            ArgumentsLevel::Context,
            ArgumentsLevel::Guild,
            ArgumentsLevel::User,
        ],
        Box::new(JoinCommand {}),
    )
}
