use crate::interactions::{CallbackFn, Interaction, InteractionType};
use crate::internal::debug::{log_message, STATUS_ERROR};
use crate::internal::users::USERS;

use std::cell::RefCell;

use rust_i18n::t;
use serenity::async_trait;
use serenity::client::Context;
use serenity::model::prelude::{ChannelId, UserId};

thread_local! {
    static COUNTER: RefCell<u32> = RefCell::new(0);
    static LAST_MESSAGE_TIME: RefCell<u32> = RefCell::new(0);
}

struct Love {}

#[async_trait]
impl CallbackFn for Love {
    async fn run(&self, channel: &ChannelId, ctx: &Context, user_id: &UserId) -> () {
        match user_id == USERS.get("isadora").unwrap() {
            true => {
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
                                return t!("interactions.chat.love.reply", "user_id" => user_id).into();
                            }
        
                            return t!("interactions.chat.love.reply_counter", "counter" => *counter, "user_id" => user_id)
                                .into();
                        }
                    })
                });
        
                if let Some(message) = message {
                    if let Err(why) = channel.say(&ctx.http, message).await {
                        log_message(&format!("Error sending message: {:?}", why), &STATUS_ERROR);
                    }
                }
            },
            false => {},
        }
    }
}

pub fn get_love_interaction() -> Interaction {
    Interaction::new(
        "love",
        "Love me",
        InteractionType::Chat,
        Box::new(Love {}),
    )
}
