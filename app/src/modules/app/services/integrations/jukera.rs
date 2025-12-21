use bostil_core::{
	arguments::{ArgumentsLevel, ListenerFnArguments},
	integrations::{CallbackParams, Integration},
	listeners::ListenerKind,
	runners::ListenerRunnerFn,
};
use lazy_static::lazy_static;
use serenity::{
	all::{Context, Message, User, UserId},
	async_trait,
	gateway::ActivityData,
};

const JUKERA_ID: UserId = UserId::new(716828755003310091);

#[derive(Clone)]
struct Jukera;

#[async_trait]
impl ListenerRunnerFn for Jukera {
	async fn run<'a>(&self, arguments: ListenerFnArguments) {
		let ctx = arguments
			.get(&ArgumentsLevel::Context)
			.unwrap()
			.downcast_ref::<Context>()
			.unwrap();
		let message = arguments
			.get(&ArgumentsLevel::Message)
			.unwrap()
			.downcast_ref::<Message>()
			.unwrap();
		let user = arguments
			.get(&ArgumentsLevel::User)
			.unwrap()
			.downcast_ref::<User>()
			.unwrap();

		match JUKERA_ID == user.id {
			true => {
				// check if message is a embed message (music session)
				match message.embeds.is_empty() {
					true => {
						ctx.set_activity(Some(ActivityData::competing(
							"Campeonato de Leitada, Modalidade: Volume",
						)));
					}
					false => {
						let current_music = match message.embeds.first() {
							Some(embed) => embed.description.as_ref().unwrap(),
							None => return,
						};

						ctx.set_activity(Some(ActivityData::listening(current_music)))
					}
				}
			}
			false => {}
		}
	}
}

lazy_static! {
		/// # Jukera integration
		///
		/// > On listen messages from jukera check if the user currently listening to music and set the activity
		pub static ref JUKERA_INTEGRATION: Integration = Integration::new(
				"jukera",
				"Listening to jukes_box",
				vec![
						ArgumentsLevel::Context,
						ArgumentsLevel::User,
						ArgumentsLevel::Message,
				],
				ListenerKind::Message,
				Box::new(Jukera),
				None::<fn(CallbackParams)>
		);
}
