use crate::internal::users::USERS;

use std::cell::RefCell;

use serenity::client::Context;
use serenity::model::prelude::ChannelId;

thread_local! {
    static COUNTER: RefCell<u32> = RefCell::new(0);
    static LAST_MESSAGE_TIME: RefCell<u32> = RefCell::new(0);
}

pub async fn join_channel(channel: &ChannelId, ctx: &Context, user_id: &str) -> () {
    let message = COUNTER.with(|counter| {
        LAST_MESSAGE_TIME.with(|last_message_time| {
            let mut counter = counter.borrow_mut();
            let mut last_message_time = last_message_time.borrow_mut();
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as u32;

            if now - *last_message_time < 5 {
                *last_message_time = now;

                return None;
            } else {
                *last_message_time = now;
                *counter += 1;

                if user_id == USERS.get("scaliza").unwrap() {
                    // if counter is 1 send without the angry emoji and if it's not send with the angry emoji
                    if *counter == 1 {
                        return format!("VAI TOMAR NO CU <@{}>", user_id).into();
                    }

                    return format!("O CAPETA CHEGOU {}ª vezes 😡", counter).into();
                }

                if *counter == 1 {
                    return format!("Bom dia <@{}> ❤️", user_id).into();
                }

                return None;
            }
        })
    });

    if let Some(message) = message {
        if let Err(why) = channel.say(&ctx.http, message).await {
            println!("Error sending message: {:?}", why);
        }
    }
}
