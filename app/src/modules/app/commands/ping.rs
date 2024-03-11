use bostil_core::{
    arguments::{ArgumentsLevel, CommandFnArguments},
    commands::{Command, CommandCategory, CommandContext},
    runners::runners::{CommandResponse, CommandResult, CommandRunnerFn},
};
use lazy_static::lazy_static;
use serenity::{async_trait, builder::CreateCommand, client::Context};
use std::time::Duration;
use tracing::{debug, error, info};

use crate::ShardManagerContainer;

#[derive(Clone)]
struct Ping;

#[async_trait]
impl CommandRunnerFn for Ping {
    async fn run<'a>(&self, arguments: CommandFnArguments) -> CommandResult<'a> {
        let context = arguments
            .get(&ArgumentsLevel::Context)
            .unwrap()
            .downcast_ref::<Context>()
            .unwrap();

        let data = context.data.read().await;

        let shard_manager = match data.get::<ShardManagerContainer>() {
            Some(v) => v,
            None => {
                error!("No shard manager found");

                return Ok(CommandResponse::String(
                    "There was a problem getting the shard manager".to_string(),
                ));
            }
        };

        debug!("ShardManager: #{:?}", shard_manager);

        let runners = shard_manager.runners.lock().await;

        let runner = match runners.get(&context.shard_id) {
            Some(runner) => runner,
            None => {
                error!("No shard runner found for shard {}", context.shard_id);

                return Ok(CommandResponse::String(
                    "There was a problem getting the shard runner".to_string(),
                ));
            }
        };

        if runner.latency.is_none() {
            info!("The shard runner latency is not available");
        }

        Ok(CommandResponse::String(format!(
            "Pong! The shard runner latency is: {} ms",
            runner.latency.unwrap_or(Duration::from_secs(0)).as_millis()
        )))
    }
}

lazy_static! {
    /// # Ping Command
    ///
    /// > Command to check if the bot is alive, and test the latency to the server
    pub static ref PING_COMMAND: Command = Command::new(
        "ping",
        "Check if the bot is alive, and test the latency to the server",
        CommandContext::Global,
        CommandCategory::General,
        vec![ArgumentsLevel::Context],
        Box::new(Ping),
        Some(
            CreateCommand::new("ping")
                .description("Check if the bot is alive, and test the latency to the server"),
        ),
    );
}
