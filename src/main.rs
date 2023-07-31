include!("lib.rs");

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

use database::locale::apply_locale;
use integrations::get_chat_integrations as integrations;
use interactions::get_chat_interactions as chat_interactions;
use interactions::voice_channel::join_channel as voice_channel;
use internal::debug::{log_message, STATUS_ERROR, STATUS_INFO};

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
                if old.channel_id.is_some() && new.channel_id.is_none() && !is_bot {
                    if debug {
                        log_message(
                            &format!(
                                "User disconnected from voice channel: {:#?}",
                                old.channel_id.unwrap().to_string()
                            ),
                            &STATUS_INFO,
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

        let integrations = integrations().into_iter();
        let interactions = chat_interactions().into_iter();

        for interaction in interactions {
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
                    let _ = interaction.callback.run(&channel, &ctx, &user_id).await;
                }
                _ => {}
            }
        }

        for integration in integrations {
            if debug {
                log_message(
                    &format!("Running integration: {}", integration.name),
                    &STATUS_INFO,
                );
            }

            let user_id = msg.author.id;

            match integration.integration_type {
                integrations::IntegrationType::Chat => {
                    let _ = integration.callback.run(&msg, &ctx, &user_id).await;
                }
            }
        }
    }

    // Slash commands
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        let debug: bool = env::var("DEBUG").is_ok();

        if let Interaction::ApplicationCommand(command) = interaction {
            if debug {
                log_message(
                    &format!(
                        "Received command interaction from User: {:#?}",
                        command.user.name
                    ),
                    &STATUS_INFO,
                );
            }

            let content = match command.data.name.as_str() {
                "ping" => commands::ping::run(&command.data.options).await,
                "jingle" => commands::jingle::run(&command.data.options).await,
                "language" => {
                    commands::language::run(&command.data.options, &ctx, &command.guild_id.unwrap())
                        .await
                }
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
                    &format!("Cannot respond to slash command: {}", why),
                    &STATUS_ERROR,
                );

                if debug {
                    log_message(
                        &format!("Command name: {}", command.data.name),
                        &STATUS_INFO,
                    );
                }
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

            apply_locale(
                &guild
                    .id
                    .to_guild_cached(&ctx.cache)
                    .unwrap()
                    .preferred_locale,
                &guild.id,
                true,
            );

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

    let intents = GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::GUILD_WEBHOOKS
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILD_VOICE_STATES
        | GatewayIntents::GUILD_INTEGRATIONS;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .register_songbird()
        .await
        .expect("Error on creating client");

    tokio::spawn(async move {
        let _main_process = client
            .start()
            .await
            .map_err(|why| println!("Client ended: {:?}", why));
        let _clear_process = voice_channel::clear_cache().await;
    });

    tokio::signal::ctrl_c().await.unwrap();
    println!("Received Ctrl-C, shutting down...");
}
