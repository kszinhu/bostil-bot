use super::Command;

use serenity::async_trait;
use serenity::builder::CreateApplicationCommand;

struct Jingle;

#[async_trait]
impl super::RunnerFn for Jingle {
    async fn run(
        &self,
        _args: &Vec<Box<dyn std::any::Any + Send + Sync>>,
    ) -> super::InternalCommandResult {
        Ok(super::CommandResponse::String(
            "Tanke o Bostil ou deixe-o".to_string(),
        ))
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("jingle")
        .description("Tanke o Bostil ou deixe-o")
}

pub fn get_command() -> Command {
    Command::new(
        "jingle",
        "Tanke o Bostil ou deixe-o",
        super::CommandCategory::Fun,
        vec![super::ArgumentsLevel::None],
        Box::new(Jingle {}),
    )
}
