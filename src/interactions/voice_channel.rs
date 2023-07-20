use crate::internal::users::USERS;

use std::cell::RefCell;
use std::collections::HashMap;

use serenity::client::Context;
use serenity::model::id::UserId;
use serenity::model::prelude::ChannelId;

thread_local! {
    static CACHE: RefCell<HashMap<UserId, (u32, u32, UserId)>> = RefCell::new(HashMap::new());
}

pub async fn join_channel(channel: &ChannelId, ctx: &Context, user_id: &UserId) -> () {
    // members on voice channel
    let members = channel
        .to_channel(&ctx)
        .await
        .unwrap()
        .guild()
        .unwrap()
        .members(&ctx)
        .await
        .unwrap();

    let message = CACHE.with(|cache| -> Option<String> {
        let mut cache = cache.borrow_mut();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as u32;

        println!("{:#?}", cache);

        if let Some((counter, last_update, _)) = cache.get_mut(user_id) {
            if now - *last_update > 86400 {
                cache.clear();
            } else if now - *last_update < 5 {
                *last_update = now;
                *counter += 1;

                return None;
            }
        }

        if let Some((counter, last_update, _)) = cache.get_mut(user_id) {
            if now - *last_update < 5 {
                *last_update = now;

                return None;
            } else {
                *last_update = now;
                *counter += 1;

                if user_id == USERS.get("scaliza").unwrap() {
                    if members.len() == 1 {
                        return format!("VAI TOMAR NO CU <@{}>, TÁ SOZINHO PQ NÓIA?", user_id)
                            .into();
                    } else if members.len() >= 3 {
                        return format!(
                            "ISSO MESMO O <@{}> CHEGOU!\n, VOCÊ VEIO ALASTRAR MAIS? SIM OU CLARO?",
                            user_id
                        )
                        .into();
                    }

                    return format!("O CAPETA CHEGOU {} vezes 😡", counter).into();
                }

                if *counter == 1 {
                    return format!("Bom dia <@{}> ❤️", user_id).into();
                }

                return None;
            }
        } else {
            cache.insert(*user_id, (1, now, *user_id));

            println!("Updated Cache {:#?}", cache);

            if user_id == USERS.get("scaliza").unwrap() {
                return format!("VAI TOMAR NO CU <@{}>, ENTROU SÓ AGORA ?", user_id).into();
            }

            return format!("Bom dia <@{}> ❤️", user_id).into();
        }
    });

    if let Some(message) = message {
        if let Err(why) = channel.say(&ctx.http, message).await {
            println!("Error sending message: {:?}", why);
        }
    }
}
