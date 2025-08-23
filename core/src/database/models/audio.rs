use crate::database::{
	entities::{
		exports::{get_audio_from_content, get_audios_from_user, save_audio, Audio as AudioEntity},
		UserIdWrapper,
	},
	exports::{establish_connection, User},
};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq)]
pub struct Audio {
	pub id: i64,
	pub user_id: UserIdWrapper,
	pub content: Vec<u8>,
	pub caption: Option<String>,
	pub user: User,
	pub added_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
	syncronized_with_database: bool,
}

impl Audio {
	pub fn new(content: Vec<u8>, user: &User, caption: Option<String>) -> Self {
		Self {
			id: 0,
			user_id: user.id,
			content,
			caption,
			user: user.clone(),
			added_at: Utc::now(),
			updated_at: Utc::now(),
			syncronized_with_database: false,
		}
	}

	pub fn save(&self, conn: &mut diesel::PgConnection) -> Result<Audio, diesel::result::Error> {
		save_audio(conn, &self.content, self.user_id, self.caption.as_deref()).map(Audio::from)
	}

	pub fn get_audios_from_user(conn: &mut diesel::PgConnection, user: &User) -> Vec<Audio> {
		get_audios_from_user(conn, user.id)
			.into_iter()
			.map(Audio::from)
			.collect()
	}

	pub fn get_audio_from_content(
		conn: &mut diesel::PgConnection,
		content_filter: &str,
	) -> Result<Option<Audio>, diesel::result::Error> {
		get_audio_from_content(conn, content_filter).map(|opt| opt.map(Audio::from))
	}
}

impl From<AudioEntity> for Audio {
	fn from(value: AudioEntity) -> Self {
		let connection = &mut establish_connection();
		let user = User::get_by_id(connection, &value.user_id.into()).expect("User not exists");

		Self {
			id: value.id,
			user_id: value.user_id,
			content: value.content,
			caption: value.caption,
			user,
			added_at: value.added_at,
			updated_at: value.updated_at,
			syncronized_with_database: true,
		}
	}
}
