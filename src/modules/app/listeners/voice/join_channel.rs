use crate::internal::debug::{log_message, MessageTypes};
use crate::internal::users::USERS;
use rust_i18n::t;

use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;

use serenity::client::Context;
use serenity::model::id::UserId;
use serenity::model::prelude::ChannelId;
use tokio::time;

type Cache = HashMap<UserId, (u32, u32, UserId)>;

thread_local! {
    static CACHE: Arc<RefCell<Cache>> = Arc::new(RefCell::new(HashMap::new()));
}

pub async fn clear_cache() {
    log_message("Starting clear cache task", MessageTypes::Server);
    loop {
        time::sleep(time::Duration::from_secs(86400)).await;
        log_message("Clearing cache", MessageTypes::Server);

        CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();

            cache.clear();
        });
    }
}

pub async fn join_channel(channel: &ChannelId, ctx: &Context, user_id: &UserId) -> () {
    let user = user_id.to_user(&ctx.http).await.unwrap();
    let members = channel
        .to_channel(&ctx)
        .await
        .unwrap()
        .guild()
        .unwrap()
        .members(&ctx)
        .await
        .unwrap();

    let message = CACHE.with(|cache| {
        let mut cache = cache.borrow_mut();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as u32;

        if let Some((counter, last_update, _)) = cache.get_mut(&user.id) {
            if now - *last_update < 5 {
                *last_update = now;
                *counter += 1;

                return None;
            }
        }

        if let Some((counter, last_update, _)) = cache.get_mut(&user.id) {
            if now - *last_update < 5 {
                *last_update = now;

                return None;
            } else {
                *last_update = now;
                *counter += 1;

                if user_id == USERS.get("scaliza").unwrap() {
                    if members.len() == 1 {
                        return t!(&format!("interactions.join_channel.scaliza.empty_channel"), user_id => user.id).into();
                    } else if members.len() >= 3 {
                        return t!(&format!("interactions.join_channel.scaliza.many_users"), user_id => user.id).into();
                    }

                    return format!("O CAPETA CHEGOU {} vezes 😡", counter).into();
                }

                return t!(&format!("interactions.join_channel.{}", (*counter as u8).min(2)), user_id => user.id).into();
            }
        } else {
            cache.insert(*user_id, (1, now, *user_id));
            log_message(format!("Added {} to cache", user.name).as_str(), MessageTypes::Success);

            if user_id == USERS.get("scaliza").unwrap() {
                return t!(&format!("interactions.join_channel.scaliza.0"), user_id => user.id).into();
            }

            return t!(&format!("interactions.join_channel.0"), user_id => user.id).into();
        }
    });

    if let Some(message) = message {
        if let Err(why) = channel.say(&ctx.http, message).await {
            log_message(
                format!("Error sending message: {:?}", why).as_str(),
                MessageTypes::Error,
            );
        }
    }
}
