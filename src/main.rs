include!("lib.rs");

use std::env;
use std::sync::Arc;

use internal::debug::{log_message, STATUS_ERROR, STATUS_INFO};
use serenity::async_trait;
use serenity::client::bridge::gateway::ShardManager;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::model::prelude::command::Command;
use serenity::model::voice::VoiceState;
use serenity::prelude::*;

use songbird::SerenityInit;

use interactions::voice_channel::join_channel as voice_channel;

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

            voice_channel::join_channel(&new.channel_id.unwrap(), &ctx, &new.user_id).await;
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
            log_message(
                &format!("Received message from User: {:#?}", msg.author.name),
                &STATUS_INFO,
            );
        }

        interactions::get_chat_interactions()
            .iter()
            .for_each(|interaction| {
                if msg.content.starts_with(&format!("!{}", interaction.name)) {
                    if debug {
                        log_message(
                            &format!("Running interaction: {}", interaction.name),
                            &STATUS_INFO,
                        );
                    }

                    let channel = msg.channel_id;
                    let user_id = msg.author.id;

                    match interaction.interaction_type {
                        interactions::InteractionType::Chat => {
                            let callback = interaction
                                .callback
                                .downcast_ref::<interactions::InteractionCallback>()
                                .unwrap();

                            callback(&channel, &ctx, &user_id);
                        }
                        _ => {}
                    }
                }
            });
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
                "language" => commands::language::run(&command.data.options).await,
                _ => "Unknown command".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                log_message(
                    &format!(
                        "Cannot respond to slash command: {}\nCommand name: {}",
                        why, command.data.name
                    ),
                    &STATUS_ERROR,
                );
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        log_message(
            format!("Connected on Guilds: {}", ready.guilds.len()).as_str(),
            &STATUS_INFO,
        );

        // global commands
        let commands = Command::set_global_application_commands(&ctx.http, |commands| {
            commands.create_application_command(|command| commands::ping::register(command))
        })
        .await;

        if let Err(why) = commands {
            log_message(
                &format!("Cannot register slash commands: {}", why),
                &STATUS_ERROR,
            );
        }

        log_message("Registered global slash commands", &STATUS_INFO);

        // guild commands and apply language to each guild
        for guild in ready.guilds.iter() {
            let commands = GuildId::set_application_commands(&guild.id, &ctx.http, |commands| {
                commands.create_application_command(|command| commands::jingle::register(command));
                commands
                    .create_application_command(|command| commands::language::register(command));
                commands
            })
            .await;

            if let Err(why) = commands {
                log_message(
                    &format!("Cannot register slash commands: {}", why),
                    &STATUS_ERROR,
                );
            }

            log_message(
                &format!("Registered slash commands for guild {}", guild.id),
                &STATUS_INFO,
            );
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
        let _clear_process = voice_channel::clear_cache().await;
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
