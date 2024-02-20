use rust_i18n::t;

use serenity::framework::standard::CommandResult;
use serenity::model::prelude::{Guild, UserId};
use serenity::prelude::Context;
use tracing::{debug, error, info};

pub async fn join(ctx: &Context, guild: &Guild, user_id: &UserId) -> CommandResult<String> {
    let channel_id = guild.voice_states.get(user_id).unwrap().channel_id;

    debug!("User is in voice channel: {:?}", channel_id);

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            error!("User is not in a voice channel");

            return Ok(t!("commands.voice.user_not_connected").to_string());
        }
    };

    debug!("Connecting to voice channel: {}", connect_to);

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    debug!("Manager: {:?}", manager);

    if let Err(why) = manager.join(guild.id, connect_to).await {
        error!("Failed to join voice channel: {:?}", why);

        return Ok(t!("commands.voice.join_failed").to_string());
    }

    info!("Joined voice channel");

    Ok(t!("commands.voice.join").to_string())
}

pub async fn mute(ctx: &Context, guild: &Guild, _user_id: &UserId) -> CommandResult<String> {
    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    let handler = match manager.get(guild.id) {
        Some(handler) => handler,
        None => {
            error!("Bot not connected to a voice channel");

            return Ok(t!("commands.voice.bot_not_connected").to_string());
        }
    };
    let mut handler = handler.lock().await;

    if handler.is_mute() {
        debug!("User already muted");
    } else {
        if let Err(why) = handler.mute(true).await {
            error!("Failed to mute user: {:?}", why);
        }
    }

    Ok(t!("commands.voice.mute").to_string())
}

pub async fn unmute(ctx: &Context, guild: &Guild, _user_id: &UserId) -> CommandResult<String> {
    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    let handler = match manager.get(guild.id) {
        Some(handler) => handler,
        None => {
            error!("Bot not connected to a voice channel");

            return Ok(t!("commands.voice.bot_not_connected").to_string());
        }
    };
    let mut handler = handler.lock().await;

    if handler.is_mute() {
        if let Err(why) = handler.mute(false).await {
            error!("Failed to unmute user: {:?}", why);
        }
    }

    Ok(t!("commands.voice.un_mute").to_string())
}

pub async fn leave(ctx: &Context, guild: &Guild, _user_id: &UserId) -> CommandResult<String> {
    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();
    let has_handler = manager.get(guild.id).is_some();

    if has_handler {
        if let Err(why) = manager.remove(guild.id).await {
            error!("Failed to leave voice channel: {:?}", why);
        }
    } else {
        error!("Bot not connected to a voice channel");

        return Ok(t!("commands.voice.bot_not_connected").to_string());
    }

    Ok(t!("commands.voice.leave").to_string())
}
