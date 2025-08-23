use bostil_core::{
	arguments::ArgumentsLevel,
	listeners::{Listener, ListenerKind},
	runners::runners::ListenerRunnerFn,
};
use lazy_static::lazy_static;
use rust_i18n::t;
use serenity::{
	all::{ChannelId, User, UserId},
	async_trait,
	client::Context,
};
use std::{any::Any, cell::RefCell};
use tracing::error;

thread_local! {
		static COUNTER: RefCell<u32> = RefCell::new(0);
		static LAST_MESSAGE_TIME: RefCell<u32> = RefCell::new(0);
}

const USER_ID: UserId = UserId::new(729817162495033485);

#[derive(Clone)]
struct Love;

#[async_trait]
impl ListenerRunnerFn for Love {
	async fn run<'a>(&self, args: &Vec<Box<dyn Any + Send + Sync>>) -> () {
		let ctx = *args
			.iter()
			.filter_map(|arg| arg.downcast_ref::<Context>())
			.collect::<Vec<&Context>>()
			.first()
			.unwrap();
		let channel = *args
			.iter()
			.filter_map(|arg| arg.downcast_ref::<ChannelId>())
			.collect::<Vec<&ChannelId>>()
			.first()
			.unwrap();
		let user_id = *args
			.iter()
			.filter_map(|arg| arg.downcast_ref::<User>())
			.collect::<Vec<&User>>()
			.first()
			.unwrap();

		match USER_ID == user_id.id {
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
