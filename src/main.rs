mod commands;
mod interactions;
mod internal;

use std::env;
use std::sync::Arc;

use serenity::async_trait;
use serenity::client::bridge::gateway::ShardManager;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::model::prelude::command::Command;
use serenity::model::voice::VoiceState;
use serenity::prelude::*;

use songbird::SerenityInit;

use interactions::voice_channel::clear_cache;
use internal::users::USERS;

struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // On User connect to voice channel
    async fn voice_state_update(&self, ctx: Context, old: Option<VoiceState>, new: VoiceState) {
        let debug: bool = env::var("DEBUG").is_ok();

        let is_bot: bool = new.user_id.to_user(&ctx.http).await.unwrap().bot;
        let has_connected: bool = new.channel_id.is_some() && old.is_none();

        if has_connected && !is_bot {
            if debug {
                println!(
                    "User connected to voice channel: {:#?}",
                    new.channel_id.unwrap().to_string()
                );
            }

            interactions::voice_channel::join_channel(&new.channel_id.unwrap(), &ctx, &new.user_id)
                .await;
        }

        match old {
            Some(old) => {
                if old.channel_id.is_some() && !is_bot {
                    if debug {
                        println!(
                            "User disconnected from voice channel: {:#?}",
                            old.channel_id.unwrap().to_string()
                        );
                    }
                }
            }
            None => {}
        }
    }

    // Each message on the server
    async fn message(&self, ctx: Context, msg: serenity::model::channel::Message) {
        let debug: bool = env::var("DEBUG").is_ok();

        if debug {
            println!("Received message from User: {:#?}", msg.author.name);
        }

        // TODO: register message
        if msg.author.id.to_string() == USERS.get("isadora").unwrap().to_string() {
            interactions::love::love(&msg.channel_id, &ctx).await;
        }
    }

    // Slash commands
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        let debug: bool = env::var("DEBUG").is_ok();

        if let Interaction::ApplicationCommand(command) = interaction {
            if debug {
                println!(
                    "Received command interaction from User: {:#?}",
                    command.user.name
                );
            }

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
        println!("\"{}\" is connected!", ready.user.name);

        // global commands
        let commands = Command::set_global_application_commands(&ctx.http, |commands| {
            commands.create_application_command(|command| commands::ping::register(command))
        })
        .await;

        if let Err(why) = commands {
            println!("Cannot register slash commands: {}", why);
        }

        println!("Registered global slash commands");

        // guild commands
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

        ctx.set_activity(serenity::model::gateway::Activity::playing(
            "O auxílio emergencial no PIX do Mito",
        ))
        .await;
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILD_VOICE_STATES
        | GatewayIntents::GUILDS
        | GatewayIntents::GUILD_INTEGRATIONS;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .register_songbird()
        .await
        .expect("Error on creating client");

    tokio::spawn(async move {
        let _clear_process = clear_cache().await;
    });

    tokio::spawn(async move {
        let _main_process = client
            .start()
            .await
            .map_err(|why| println!("Client ended: {:?}", why));
    });

    tokio::signal::ctrl_c().await.unwrap();
    println!("Received Ctrl-C, shutting down...");
}
