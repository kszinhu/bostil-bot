use bostil_core::{
	arguments::{ArgumentsLevel, ListenerFnArguments},
	listeners::{Listener, ListenerKind},
	runners::ListenerRunnerFn,
};
use lazy_static::lazy_static;
use rust_i18n::t;
use serenity::{
	all::{ChannelId, User, UserId},
	async_trait,
	client::Context,
};
use std::cell::RefCell;
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
	async fn run<'a>(&self, args: ListenerFnArguments) -> () {
		let ctx = args
			.get(&ArgumentsLevel::Context)
			.unwrap()
			.downcast_ref::<Context>()
			.unwrap();
		let channel = args
			.get(&ArgumentsLevel::ChannelId)
			.unwrap()
			.downcast_ref::<ChannelId>()
			.unwrap();
		let user = args
			.get(&ArgumentsLevel::User)
			.unwrap()
			.downcast_ref::<User>()
			.unwrap();

		match USER_ID == user.id {
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
								return t!("interactions.chat.love.reply", "user_id" => user.name).into();
							}

							return t!("interactions.chat.love.reply_counter", "counter" => *counter, "user_id" => user.name)
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
