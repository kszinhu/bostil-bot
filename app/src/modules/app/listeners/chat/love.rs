use bostil_core::{
    arguments::ArgumentsLevel,
    listeners::{Listener, ListenerKind},
    runners::runners::ListenerRunnerFn,
};
use diesel::{query_dsl::methods::FilterDsl, ExpressionMethods, RunQueryDsl};
use lazy_static::lazy_static;
use rust_i18n::t;
use serenity::{
    all::{ChannelId, User},
    async_trait,
    client::Context,
};
use std::{any::Any, cell::RefCell};
use tracing::error;

use crate::modules::core::{entities::user::User as UserDB, helpers::establish_connection};

thread_local! {
    static COUNTER: RefCell<u32> = RefCell::new(0);
    static LAST_MESSAGE_TIME: RefCell<u32> = RefCell::new(0);
}

#[derive(Clone)]
struct Love;

#[async_trait]
impl ListenerRunnerFn for Love {
    async fn run<'a>(&self, args: &Vec<Box<dyn Any + Send + Sync>>) -> () {
        use crate::schema::users::dsl::{username, users};

        let binding = args
            .iter()
            .filter_map(|arg| arg.downcast_ref::<Context>())
            .collect::<Vec<&Context>>();
        let ctx = *binding.first().unwrap();

        let binding = args
            .iter()
            .filter_map(|arg| arg.downcast_ref::<ChannelId>())
            .collect::<Vec<&ChannelId>>();
        let channel = *binding.first().unwrap();

        let binding = args
            .iter()
            .filter_map(|arg| arg.downcast_ref::<User>())
            .collect::<Vec<&User>>();
        let user_id = *binding.first().unwrap();

        let connection = &mut establish_connection();
        let user = users
            .filter(username.eq("Isadora"))
            .first::<UserDB>(connection)
            .unwrap() as UserDB;

        match user.id == user_id.id {
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
                                return t!("interactions.chat.love.reply", "user_id" => *user_id).into();
                            }

                            return t!("interactions.chat.love.reply_counter", "counter" => *counter, "user_id" => *user_id)
                                .into();
                        }
                    })
                });

                if let Some(message) = message {
                    if let Err(why) = channel.say(&ctx.http, message).await {
                        error!("Error sending message: {:?}", why);
                    }
                }
            }
            false => {}
        }
    }
}

lazy_static! {
    /// Listener for love messages
    pub static ref LOVE_LISTENER: Listener = Listener::new(
        "love",
        "Interact with user 'Isadora' to send love messages",
        ListenerKind::Message,
        vec![
            ArgumentsLevel::Context,
            ArgumentsLevel::User,
            ArgumentsLevel::ChannelId,
        ],
        Box::new(Love)
    );
}
