use super::{
    ArgumentsLevel, Command, CommandCategory, CommandResponse, InternalCommandResult, RunnerFn,
};

use serenity::{async_trait, builder::CreateCommand};
use std::any::Any;
use tokio::time::Instant;

struct Ping;

#[async_trait]
impl RunnerFn for Ping {
    async fn run<'a>(&self, _: &Vec<Box<dyn Any + Send + Sync>>) -> InternalCommandResult<'a> {
        let get_latency = {
            let now = Instant::now();

            let _ = reqwest::get("https://discord.com/api/v8/gateway").await;
            now.elapsed().as_millis() as f64
        };

        Ok(CommandResponse::String(format!(
            "Pong! Latency: {}ms",
            get_latency
        )))
    }
}

pub fn register(command: &mut CreateCommand) -> &mut CreateCommand {
    command
        .name("ping")
        .description("Check if the bot is alive, and test the latency to the server")
        .into()
}

pub fn get_command() -> Command {
    Command::new(
        "ping",
        "Check if the bot is alive, and test the latency to the server",
        CommandCategory::General,
        vec![ArgumentsLevel::None],
        Box::new(Ping {}),
    )
}
