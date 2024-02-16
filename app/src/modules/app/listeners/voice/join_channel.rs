use diesel::result::Error;
use rust_i18n::t;
use tracing::{error, info};

use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;

use serenity::client::Context;
use serenity::model::id::UserId;
use serenity::model::prelude::ChannelId;
use tokio::time;

use crate::modules::core::entities::user::User;
use crate::modules::core::entities::UserIdWrapper;
use crate::modules::core::helpers::establish_connection;

type Cache = HashMap<UserId, (u32, u32, UserId)>;

thread_local! {
    static CACHE: Arc<RefCell<Cache>> = Arc::new(RefCell::new(HashMap::new()));
}

pub async fn clear_cache() {
    info!("Starting clear cache task");

    loop {
        time::sleep(time::Duration::from_secs(86400)).await;
        info!("Clearing cache");

        CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();

            cache.clear();
        });
    }
}

pub async fn join_channel(channel: &ChannelId, ctx: &Context, user_id: &UserId) -> () {
    use crate::schema::users;
    use diesel::{QueryDsl, RunQueryDsl};

    let members = channel
        .to_channel(&ctx)
        .await
        .unwrap()
        .guild()
        .unwrap()
        .members(&ctx)
        .unwrap();

    let connection = &mut establish_connection();
    let user = users::table
        .find(UserIdWrapper(*user_id))
        .first::<User>(connection) as Result<User, Error>;

    match user {
        Ok(user) => {
            info!("{} joined channel", user.username);

            let message = CACHE.with(|cache| {
                let mut cache = cache.borrow_mut();
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as u32;

                if let Some((counter, last_update, _)) = cache.get_mut(user_id) {
                    if now - *last_update < 5 {
                        *last_update = now;
                        *counter += 1;

                        return None;
                    }
                }

                if let Some((counter, last_update, _)) = cache.get_mut(user_id) {
                    if now - *last_update < 5 {
                        *last_update = now;

                        None
                    } else {
                        *last_update = now;
                        *counter += 1;

                        if user.username == "scaliza" {
                            if members.len() == 1 {
                                return t!("interactions.join_channel.scaliza.empty_channel", user_id => user_id).to_string().into();
                            } else if members.len() >= 3 {
                                return t!("interactions.join_channel.scaliza.many_users", user_id => user_id).to_string().into();
                            }

                            return format!("O CAPETA CHEGOU {} vezes 😡", counter).to_string().into()
                        }

                        let key = format!("interactions.join_channel.{}", (*counter as u8).min(2));

                        t!(key.as_str(), user_id => user_id).to_string().into()
                    }
                } else {
                    cache.insert(*user_id, (1, now, *user_id));
                    info!("Added {} to cache", user.username);

                    if user.username == "scaliza" {
                        return t!("interactions.join_channel.scaliza.0", user_id => user_id).to_string().into();
                    }

                    return t!("interactions.join_channel.0", user_id => user_id).to_string().into();
                }
            });

            if let Some(message) = message {
                if let Err(why) = channel.say(&ctx.http, message).await {
                    error!("Error sending message: {:?}", why);
                }
            }
        }

        Err(_) => {
            error!("User not found")
        }
    }
}
