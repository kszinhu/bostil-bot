include!("lib.rs");

use bostil_core::{
    arguments::ArgumentsLevel, commands::CommandContext, runners::runners::CommandResponse,
};
use serenity::{
    all::{Command, GatewayIntents, GuildId, Interaction, Message, Ready, VoiceState},
    async_trait,
    builder::EditInteractionResponse,
    client::Context,
    framework::StandardFramework,
    gateway::ActivityData,
    prelude::EventHandler,
    Client,
};
use songbird::SerenityInit;
use std::env;
use tracing::{debug, error, info, warn};

use crate::modules::{
    app::listeners::voice::join_channel, core::actions, core::helpers::establish_connection,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // ---------
    // On receive message
    // ---------
    async fn message(&self, _ctx: Context, msg: Message) {
        debug!("Received message from User: {:#?}", msg.author.name);

        // TODO: use integrations and listeners collectors instead of hardcoding
    }

    // ---------
    // On bot ready
    // ---------
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("Connected on Guilds: {}", ready.guilds.len());

        let collector = match COMMAND_COLLECTOR.lock() {
            Ok(collector) => collector.clone(),
            Err(why) => {
                error!("Cannot get command collector: {}", why);
                return;
            }
        };

        let global_commands = collector
            .clone()
            .get_fingerprints(Some(CommandContext::Global));
        let guild_commands = collector
            .clone()
            .get_fingerprints(Some(CommandContext::Guild));

        if global_commands.len() == 0 && guild_commands.len() == 0 {
            warn!("No commands to register");
            return;
        }

        if let Err(why) = Command::set_global_commands(&ctx.http, global_commands.clone()).await {
            error!("Cannot register global slash commands: {}", why);
            return;
        }

        info!("Registered global slash commands");
        debug!(
            "Registered {:#?} global commands",
            global_commands.clone().len()
        );
        debug!(
            "Registered {:#?} guild commands",
            guild_commands.clone().len()
        );

        for guild in ready.guilds.iter() {
            let commands = GuildId::set_commands(guild.id, &ctx.http, guild_commands.clone()).await;

            if let Err(why) = commands {
                error!("Cannot register slash commands: {}", why);
            }

            info!("Registered slash commands for guild {}", guild.id);
        }

        ctx.set_activity(Some(ActivityData::playing(
            "O Auxílio Emergencial no PIX do Mito",
        )))
    }

    // ---------
    // On User connect to voice channel
    // ---------
    async fn voice_state_update(&self, ctx: Context, old: Option<VoiceState>, new: VoiceState) {
        let is_bot: bool = new.user_id.to_user(&ctx.http).await.unwrap().bot;
        let has_connected: bool = new.channel_id.is_some() && old.is_none();

        if has_connected && !is_bot {
            debug!("User connected to voice channel: {:#?}", new.channel_id);

            join_channel(&new.channel_id.unwrap(), &ctx, &new.user_id).await;
        }

        match old {
            Some(old) => {
                if old.channel_id.is_some() && new.channel_id.is_none() && !is_bot {
                    debug!(
                        "User disconnected from voice channel: {:#?}",
                        old.channel_id
                    );
                }
            }
            None => {}
        }
    }

    // ---------
    // On create interaction (slash command, button, select menu, etc.)
    // ---------
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction {
            Interaction::Modal(submit) => {
                submit.defer(&ctx.http.clone()).await.unwrap();

                debug!(
                    "Received modal submit interaction from User: {:#?}",
                    submit.user.name
                );

                // let registered_interactions = get_modal_interactions();

                // // custom_id is in the format: '<interaction_name>/<id>'
                // match registered_interactions.iter().enumerate().find(|(_, i)| {
                //     i.name
                //         == submit
                //             .clone()
                //             .data
                //             .custom_id
                //             .split("/")
                //             .collect::<Vec<&str>>()
                //             .first()
                //             .unwrap()
                //             .to_string()
                // }) {
                //     Some((_, interaction)) => {
                //         let Some(guild) = ({
                //             let cloned_ctx = ctx.clone();
                //             let guild_reference = cloned_ctx.cache.guild(submit.guild_id.unwrap());

                //             match guild_reference {
                //                 Some(guild) => Some(guild.clone()),
                //                 None => None,
                //             }
                //         }) else {
                //             error!("Cannot get guild from cache");
                //             return;
                //         };

                //         interaction
                //             .runner
                //             .run(&ArgumentsLevel::provide(
                //                 &interaction.arguments,
                //                 &ctx,
                //                 &guild,
                //                 &submit.user,
                //                 &submit.channel_id,
                //                 None,
                //                 Some(submit.id),
                //                 Some(&submit.data),
                //                 None,
                //             ))
                //             .await;
                //     }

                //     None => {
                //         error!(
                //             "Modal submit interaction {} not found",
                //             submit.data.custom_id.split("/").collect::<Vec<&str>>()[0]
                //         );
                //     }
                // };
            }

            Interaction::Command(command) => {
                info!(
                    "Received command \"{}\" interaction from User: {:#?}",
                    command.data.name, command.user.name
                );

                // Defer the interaction and edit it later
                match command.defer(&ctx.http.clone()).await {
                    Ok(_) => {}
                    Err(why) => {
                        error!("Cannot defer slash command: {}", why);
                    }
                }

                let collector = match COMMAND_COLLECTOR.lock() {
                    Ok(collector) => collector.clone(),
                    Err(why) => {
                        error!("Cannot get command collector: {}", why);
                        return;
                    }
                };

                debug!("Running command: {}", command.data.name);

                match collector
                    .commands
                    .iter()
                    .enumerate()
                    .find(|(_, c)| c.name == command.data.name)
                {
                    Some((_, command_interface)) => {
                        let Some(guild) = ({
                            let cloned_ctx = ctx.clone();
                            let guild_reference = cloned_ctx.cache.guild(command.guild_id.unwrap());

                            match guild_reference {
                                Some(guild) => Some(guild.clone()),
                                None => None,
                            }
                        }) else {
                            error!("Cannot get guild from cache");
                            return;
                        };

                        match command_interface
                            .runner
                            .run(&ArgumentsLevel::provide(
                                &command_interface.arguments,
                                &ctx,
                                &guild,
                                &command.user,
                                &command.channel_id,
                                Some(command.data.options.clone()),
                                Some(command.id),
                                None,
                                None,
                            ))
                            .await
                        {
                            Ok(command_response) => {
                                debug!("Responding to slash command: {}", command.data.name);

                                if CommandResponse::None != command_response {
                                    if let Err(why) = match command_response {
                                        CommandResponse::String(string) => {
                                            command
                                                .edit_response(
                                                    &ctx.http,
                                                    EditInteractionResponse::default()
                                                        .content(string),
                                                )
                                                .await
                                        }
                                        CommandResponse::Embed(embed) => {
                                            command
                                                .edit_response(
                                                    &ctx.http,
                                                    EditInteractionResponse::default()
                                                        .embed(embed.into()),
                                                )
                                                .await
                                        }
                                        CommandResponse::Message(message) => {
                                            command.edit_response(&ctx.http, message).await
                                        }
                                        // if none is returned ignore
                                        CommandResponse::None => todo!(),
                                    } {
                                        error!("Cannot respond to slash command: {}", why);
                                    }
                                } else {
                                    debug!("Deleting slash command: {}", command.data.name);

                                    if let Err(why) = command
                                        .edit_response(&ctx.http, EditInteractionResponse::new())
                                        .await
                                    {
                                        error!("Cannot respond to slash command: {}", why);
                                    }
                                }
                            }
                            Err(why) => {
                                error!("Cannot run slash command: {}", why);
                            }
                        }
                    }
                    None => {
                        error!("Command {} not found", command.data.name);
                    }
                };
            }
            _ => {}
        }
    }
}

#[tokio::main]
async fn main() {
    use dotenvy::dotenv;

    dotenv().ok();
    // tracing subscriber with default env variable
    let filter = tracing_subscriber::EnvFilter::from_default_env();
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .compact()
        .init();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    establish_connection();

    let mut command_collector = match COMMAND_COLLECTOR.lock() {
        Ok(collector) => collector.clone(),
        Err(why) => {
            error!("Cannot get command collector: {}", why);
            return;
        }
    };

    let mut listener_collector = match LISTENER_COLLECTOR.lock() {
        Ok(collector) => collector.clone(),
        Err(why) => {
            error!("Cannot get listener collector: {}", why);
            return;
        }
    };

    info!("Starting bot");

    actions::collectors::register_commands(&mut command_collector);
    actions::collectors::register_listeners(&mut listener_collector);

    info!("Collected commands: {:#?}", command_collector.length);
    info!("Collected listeners: {:#?}", listener_collector.length);

    // save the collector
    *COMMAND_COLLECTOR.lock().unwrap() = command_collector;

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
        .type_map_insert::<HttpKey>(HttpClient::new())
        .await
        .expect("Error on creating client");

    {
        let mut data = client.data.write().await;

        data.insert::<ShardManagerContainer>(std::sync::Arc::clone(&client.shard_manager));
    }

    tokio::spawn(async move {
        let _main_process = client
            .start()
            .await
            .map_err(|why| println!("Client ended: {:?}", why));
    });

    tokio::signal::ctrl_c().await.unwrap();
    println!("\rReceived Ctrl-C, shutting down...");
}
