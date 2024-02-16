use bostil_core::{
    arguments::ArgumentsLevel,
    commands::{Command, CommandCategory, CommandContext},
    runners::runners::{CommandResponse, CommandResult, CommandRunnerFn},
};
use lazy_static::lazy_static;
use serenity::{async_trait, builder::CreateCommand};
use std::any::Any;

#[derive(Clone)]
struct Jingle;

#[async_trait]
impl CommandRunnerFn for Jingle {
    async fn run<'a>(&self, _args: &Vec<Box<dyn Any + Send + Sync>>) -> CommandResult<'a> {
        Ok(CommandResponse::String(
            "Tanke o Bostil ou deixe-o".to_string(),
        ))
    }
}

lazy_static! {
    pub static ref JINGLE_COMMAND: Command = Command::new(
        "jingle",
        "Tanke o Bostil ou deixe-o",
        CommandContext::Guild,
        CommandCategory::Fun,
        vec![ArgumentsLevel::None],
        Box::new(Jingle {}),
        Some(CreateCommand::new("jingle").description("Tanke o Bostil ou deixe-o")),
    );
}
