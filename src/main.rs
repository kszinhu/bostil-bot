mod commands;

use std::env;
use std::sync::Arc;

use serenity::async_trait;
use serenity::client::bridge::gateway::ShardManager;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::model::prelude::command::Command;
use serenity::prelude::*;

struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!(
                "Received command interaction from User: {:#?}",
                command.user.name
            );
            let content = match command.data.name.as_str() {
                "ping" => commands::ping::run(&command.data.options).await,
                "jingle" => commands::jingle::run(&command.data.options).await,
                _ => "Tu tá saindo do bostil, seu nóia".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        // global commands
        let commands = serenity::http::Http::get_global_application_commands(&ctx.http)
            .await
            .expect("Cannot get global application commands");

        if commands.is_empty() {
            let commands = Command::set_global_application_commands(&ctx.http, |commands| {
                commands.create_application_command(|command| commands::ping::register(command))
            })
            .await;

            if let Err(why) = commands {
                println!("Cannot register slash commands: {}", why);
            }

            println!("Registered global slash commands");
        }

        // iterate over all guilds and register slash commands, using destructuring
        for guild in ready.guilds.iter() {
            let commands = GuildId::set_application_commands(&guild.id, &ctx.http, |commands| {
                commands.create_application_command(|command| commands::jingle::register(command))
            })
            .await;

            if let Err(why) = commands {
                println!("Cannot register slash commands: {}", why);
            }

            println!("Registered slash commands for guild {}", guild.id);
        }
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Error on creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
