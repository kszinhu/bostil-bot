use crate::interactions::{RunnerFn, Interaction, InteractionType};
use crate::internal::arguments::ArgumentsLevel;
use crate::internal::debug::{log_message, MessageTypes};
use crate::internal::users::USERS;

use std::cell::RefCell;

use rust_i18n::t;
use serenity::async_trait;
use serenity::client::Context;
use serenity::model::prelude::ChannelId;
use serenity::model::user::User;

thread_local! {
    static COUNTER: RefCell<u32> = RefCell::new(0);
    static LAST_MESSAGE_TIME: RefCell<u32> = RefCell::new(0);
}

struct Love {}

#[async_trait]
impl RunnerFn for Love {
    async fn run(&self, args: &Vec<Box<dyn std::any::Any + Send + Sync>>) -> () {
        let ctx = args
            .iter()
            .filter_map(|arg| arg.downcast_ref::<Context>())
            .collect::<Vec<&Context>>()[0];
        let channel = args
            .iter()
            .filter_map(|arg| arg.downcast_ref::<ChannelId>())
            .collect::<Vec<&ChannelId>>()[0];
        let user_id = &args
            .iter()
            .filter_map(|arg| arg.downcast_ref::<User>())
            .collect::<Vec<&User>>()[0].id;

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
                        log_message(format!("Error sending message: {:?}", why).as_str(), MessageTypes::Error);
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
        vec![ArgumentsLevel::Context, ArgumentsLevel::ChannelId, ArgumentsLevel::User],
        Box::new(Love {}),
    )
}
