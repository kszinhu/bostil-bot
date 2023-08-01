use crate::internal::debug::{log_message, STATUS_ERROR, STATUS_INFO};

use rust_i18n::t;

use serenity::framework::standard::CommandResult;
use serenity::model::prelude::{Guild, UserId};
use serenity::prelude::Context;

pub async fn join(ctx: &Context, guild: &Guild, user_id: &UserId) -> CommandResult<String> {
    let channel_id = guild.voice_states.get(user_id).unwrap().channel_id;

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            log_message(&format!("User is not in a voice channel"), &STATUS_ERROR);

            return Ok(t!("commands.voice.user_not_connected"));
        }
    };

    println!("Connecting to {:?}", connect_to);

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    println!("Manager: {:?}", manager);

    let handler = manager.join(guild.id, connect_to).await;

    println!("Handler: {:?}", handler);

    match handler.1 {
        Ok(_) => {}
        Err(why) => {
            log_message(&format!("Failed: {:?}", why), &STATUS_ERROR);

            return Ok(t!("commands.voice.join_failed"));
        }
    }

    log_message(&format!("Joined voice channel"), &STATUS_INFO);

    Ok(t!("commands.voice.join"))
}

pub async fn mute(ctx: &Context, guild: &Guild, _user_id: &UserId) -> CommandResult<String> {
    let debug = std::env::var("DEBUG").is_ok();

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    let handler_lock = match manager.get(guild.id) {
        Some(handler) => handler,
        None => {
            log_message(
                &format!("Bot not connected to a voice channel"),
                &STATUS_ERROR,
            );

            return Ok(t!("commands.voice.bot_not_connected"));
        }
    };

    let mut handler = handler_lock.lock().await;

    if handler.is_mute() {
        if debug {
            log_message(&format!("User already muted"), &STATUS_ERROR);
        }
    } else {
        if let Err(why) = handler.mute(true).await {
            log_message(&format!("Failed: {:?}", why), &STATUS_ERROR);
        }
    }

    Ok(t!("commands.voice.mute"))
}

pub async fn unmute(ctx: &Context, guild: &Guild, _user_id: &UserId) -> CommandResult<String> {
    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    let handler_lock = match manager.get(guild.id) {
        Some(handler) => handler,
        None => {
            log_message(
                &format!("Bot not connected to a voice channel"),
                &STATUS_ERROR,
            );

            return Ok(t!("commands.voice.bot_not_connected"));
        }
    };

    let mut handler = handler_lock.lock().await;

    if handler.is_mute() {
        if let Err(why) = handler.mute(false).await {
            log_message(&format!("Failed: {:?}", why), &STATUS_ERROR);
        }
    }

    Ok(t!("commands.voice.un_mute"))
}

pub async fn leave(ctx: &Context, guild: &Guild, _user_id: &UserId) -> CommandResult<String> {
    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();
    let has_handler = manager.get(guild.id).is_some();

    if has_handler {
        if let Err(why) = manager.remove(guild.id).await {
            log_message(&format!("Failed: {:?}", why), &STATUS_ERROR);
        }
    } else {
        log_message(
            &format!("Bot not connected to a voice channel"),
            &STATUS_ERROR,
        );

        return Ok(t!("commands.voice.bot_not_connected"));
    }

    Ok(t!("commands.voice.leave"))
}
