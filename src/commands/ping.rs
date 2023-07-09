use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use tokio::time::Instant;

pub async fn run(_options: &Vec<CommandDataOption>) -> String {
    let get_latency = {
        let now = Instant::now();

        let _ = reqwest::get("https://discord.com/api/v8/gateway").await;
        now.elapsed().as_millis() as f64
    };

    format!("Pong! Latency: {}ms", get_latency)
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("ping")
        .description("Check if the bot is alive, and test the latency to the server")
}
