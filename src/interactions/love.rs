use crate::internal::debug::{log_message, STATUS_ERROR};
use crate::internal::users::USERS;

use std::cell::RefCell;

use rust_i18n::t;
use serenity::client::Context;
use serenity::model::prelude::{ChannelId, UserId};

thread_local! {
    static COUNTER: RefCell<u32> = RefCell::new(0);
    static LAST_MESSAGE_TIME: RefCell<u32> = RefCell::new(0);
}

pub async fn love(channel: &ChannelId, ctx: &Context, user_id: &UserId) -> () {
    if user_id != USERS.get("isadora").unwrap() {
        return;
    }

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

                return None.into();
            } else {
                *last_message_time = now;
                *counter += 1;

                if *counter == 1 {
                    return t!("interactions.chat.love.reply").into();
                }

                return t!("interactions.chat.love.reply_counter", "counter" => *counter).into();
            }
        })
    });

    if let Some(message) = message {
        if let Err(why) = channel.say(&ctx.http, message).await {
            log_message(&format!("Error sending message: {:?}", why), &STATUS_ERROR);
        }
    }
}
