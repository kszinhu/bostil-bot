use bostil_core::{
	arguments::{ArgumentsLevel, ListenerFnArguments},
	database::exports::{establish_connection, User as UserModel},
	listeners::{Listener, ListenerKind},
	runners::ListenerRunnerFn,
};
use lazy_static::lazy_static;
use rust_i18n::t;
use serenity::{
	all::{ChannelId, Context, User, UserId},
	async_trait,
};
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{debug, error, info};

type Cache = HashMap<UserId, (u32, u32, UserId)>;

thread_local! {
		static CACHE: Arc<RefCell<Cache>> = Arc::new(RefCell::new(HashMap::new()));
}

const SCALIZA_USER_ID: UserId = UserId::new(331219229992812556);

#[derive(Clone)]
struct JoinChannel;

#[async_trait]
impl ListenerRunnerFn for JoinChannel {
	async fn run<'a>(&self, args: ListenerFnArguments) {
		let old_voice_state = args
			.get(&ArgumentsLevel::OldVoiceState)
			.unwrap()
			.downcast_ref::<Option<serenity::all::VoiceState>>()
			.unwrap();
		let new_voice_state = args
			.get(&ArgumentsLevel::NewVoiceState)
			.unwrap()
			.downcast_ref::<serenity::all::VoiceState>()
			.unwrap();

		if old_voice_state.as_ref().and_then(|ovs| ovs.channel_id) == new_voice_state.channel_id {
			debug!("User did not change voice channel, ignoring");
			return;
		}

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

		let members = channel
			.to_channel(&ctx)
			.await
			.unwrap()
			.guild()
			.unwrap()
			.members(&ctx)
			.unwrap();

		let connection = &mut establish_connection();
		let user = match UserModel::get_by_id(connection, &user.id) {
			Some(user) => user,
			None => {
				info!(
					"User with ID {} not found in database, creating new user",
					&user.id
				);

				let client_user = match ctx.http.get_user(user.id).await {
					Ok(user) => user,
					Err(why) => {
						error!("Error fetching user: {:?}", why);
						return;
					}
				};

				match UserModel::new(user.id, &client_user.name.as_str(), false).save(connection) {
					Ok(user) => user,
					Err(e) => {
						error!("Error creating user: {:?}", e);
						return;
					}
				}
			}
		};

		info!("{} joined channel", user.username);

		let message = CACHE.with(|cache| {
			let mut cache = cache.borrow_mut();
			let now = std::time::SystemTime::now()
				.duration_since(std::time::UNIX_EPOCH)
				.unwrap()
				.as_secs() as u32;

			if let Some((counter, last_update, _)) = cache.get_mut(&user.id.into()) {
				if now - *last_update < 5 {
					*last_update = now;
					*counter += 1;

					return None;
				}
			}

			if let Some((counter, last_update, _)) = cache.get_mut(&user.id.into()) {
				if now - *last_update < 5 {
					*last_update = now;

					None
				} else {
					*last_update = now;
					*counter += 1;

					if user.id == SCALIZA_USER_ID {
						if members.len() == 1 {
							return t!("interactions.join_channel.scaliza.empty_channel", user_id => &user.id)
								.to_string()
								.into();
						} else if members.len() >= 3 {
							return t!("interactions.join_channel.scaliza.many_users", user_id => &user.id)
								.to_string()
								.into();
						}

						return format!("O CAPETA CHEGOU {} vezes 😡", counter)
							.to_string()
							.into();
					}

					let key = format!("interactions.join_channel.{}", (*counter as u8).min(2));

					t!(key.as_str(), user_id => &user.id).to_string().into()
				}
			} else {
				cache.insert(user.id.into(), (1, now, user.id.into()));
				info!("Added {} to cache", user.username);

				if user.id == SCALIZA_USER_ID {
					return t!("interactions.join_channel.scaliza.0", user_id => user.id)
						.to_string()
						.into();
				}

				return t!("interactions.join_channel.0", user_id => user.id)
					.to_string()
					.into();
			}
		});

		if let Some(message) = message {
			if let Err(why) = channel.say(&ctx.http, message).await {
				error!("Error sending message: {:?}", why);
			}
		}
	}
}

lazy_static! {
	pub static ref JOIN_CHANNEL_LISTENER: Listener = Listener::new(
		"join_channel",
		"Interact with user when they join a voice channel",
		ListenerKind::VoiceState,
		vec![
			ArgumentsLevel::Context,
			ArgumentsLevel::ChannelId,
			ArgumentsLevel::User,
		],
		Box::new(JoinChannel),
	);
}
