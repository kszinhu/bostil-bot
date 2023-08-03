include!("lib.rs");

use std::env;
use std::sync::Arc;

use serenity::async_trait;
use serenity::client::bridge::gateway::ShardManager;
use serenity::framework::StandardFramework;
use serenity::model::application::interaction::Interaction;
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
use internal::debug::{log_message, MessageTypes};

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
                log_message(
                    format!(
                        "User connected to voice channel: {:#?}",
                        new.channel_id.unwrap().to_string()
                    )
                    .as_str(),
                    MessageTypes::Debug,
                );
            }

            voice_channel::join_channel(&new.channel_id.unwrap(), &ctx, &new.user_id).await;
        }

        match old {
            Some(old) => {
                if old.channel_id.is_some() && new.channel_id.is_none() && !is_bot {
                    if debug {
                        log_message(
                            format!(
                                "User disconnected from voice channel: {:#?}",
                                old.channel_id.unwrap().to_string()
                            )
                            .as_str(),
                            MessageTypes::Debug,
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
                format!("Received message from User: {:#?}", msg.author.name).as_str(),
                MessageTypes::Debug,
            );
        }

        let integrations = integrations().into_iter();
        let interactions = chat_interactions().into_iter();

        for interaction in interactions {
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
                    format!(
                        "Received command {} interaction from User: {:#?}",
                        command.data.name, command.user.name
                    )
                    .as_str(),
                    MessageTypes::Debug,
                );
            }

            let _ = command.defer(&ctx.http.clone()).await;

            let content = match command.data.name.as_str() {
                "ping" => commands::ping::run(&command.data.options).await,
                "jingle" => commands::jingle::run(&command.data.options).await,
                "language" => {
                    commands::language::run(&command.data.options, &ctx, &command.guild_id.unwrap())
                        .await
                }
                "radio" => commands::radio::run(
                    &command.data.options,
                    &ctx,
                    &command
                        .guild_id
                        .unwrap()
                        .to_guild_cached(&ctx.cache)
                        .unwrap(),
                    &command.user.id,
                )
                .await
                .unwrap(),
                "mute" => commands::voice::mute::run(
                    &ctx,
                    &command
                        .guild_id
                        .unwrap()
                        .to_guild_cached(&ctx.cache)
                        .unwrap(),
                    &command.user.id,
                    &command.data.options,
                )
                .await
                .unwrap(),
                "leave" => commands::voice::leave::run(
                    &ctx,
                    &command
                        .guild_id
                        .unwrap()
                        .to_guild_cached(&ctx.cache)
                        .unwrap(),
                    &command.user.id,
                    &command.data.options,
                )
                .await
                .unwrap(),
                "join" => commands::voice::join::run(
                    &ctx,
                    &command
                        .guild_id
                        .unwrap()
                        .to_guild_cached(&ctx.cache)
                        .unwrap(),
                    &command.user.id,
                    &command.data.options,
                )
                .await
                .unwrap(),
                _ => "Not implemented".to_string(),
            };

            log_message(
                format!("Responding with: {}", content).as_str(),
                MessageTypes::Debug,
            );

            if let Err(why) = command
                .edit_original_interaction_response(ctx.http, |response| response.content(content))
                .await
            {
                log_message(
                    format!("Cannot respond to slash command: {}", why).as_str(),
                    MessageTypes::Error,
                );
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        log_message(
            format!("Connected on Guilds: {}", ready.guilds.len()).as_str(),
            MessageTypes::Server,
        );

        // global commands
        let commands = Command::set_global_application_commands(&ctx.http, |commands| {
            commands.create_application_command(|command| commands::ping::register(command))
        })
        .await;

        if let Err(why) = commands {
            log_message(
                format!("Cannot register slash commands: {}", why).as_str(),
                MessageTypes::Failed,
            );
        }

        log_message("Registered global slash commands", MessageTypes::Success);

        // guild commands and apply language to each guild
        for guild in ready.guilds.iter() {
            let commands = GuildId::set_application_commands(&guild.id, &ctx.http, |commands| {
                commands.create_application_command(|command| commands::jingle::register(command));
                commands
                    .create_application_command(|command| commands::language::register(command));
                commands.create_application_command(|command| commands::radio::register(command));
                commands.create_application_command(|command| {
                    commands::voice::leave::register(command)
                });
                commands
                    .create_application_command(|command| commands::voice::mute::register(command));
                commands
                    .create_application_command(|command| commands::voice::join::register(command));

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
                    format!("Cannot register slash commands: {}", why).as_str(),
                    MessageTypes::Failed,
                );
            }

            log_message(
                format!("Registered slash commands for guild {}", guild.id).as_str(),
                MessageTypes::Success,
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

    let framework = StandardFramework::new();

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
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
