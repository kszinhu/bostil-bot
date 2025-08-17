use crate::database::entities::exports::{
	create_user, get_audios_from_user, get_user_by_id, get_users, User as UserEntity, UserIdWrapper,
};
use chrono::{DateTime, Utc};
use serenity::all::{User as SerenityUser, UserId};

#[derive(Debug, Clone, PartialEq)]
pub struct User {
	pub id: UserIdWrapper,
	pub username: String,
	pub added_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
	syncronized_with_database: bool,
}

impl User {
	pub fn new(id: UserId, username: &str, sync_db: bool) -> Self {
		Self {
			id: UserIdWrapper(id),
			username: username.to_string(),
			added_at: Utc::now(),
			updated_at: Utc::now(),
			syncronized_with_database: sync_db,
		}
	}

	pub fn save(&self, conn: &mut diesel::PgConnection) -> Result<User, diesel::result::Error> {
		use crate::database::entities::exports::create_user;

		create_user(conn, self.id, &self.username).map(User::from)
	}

	pub fn get_all(conn: &mut diesel::PgConnection, filter: Option<&str>) -> Vec<User> {
		use crate::database::entities::exports::get_users;

		get_users(conn, filter)
			.into_iter()
			.map(User::from)
			.collect()
	}

	pub fn get_by_id(conn: &mut diesel::PgConnection, user_id: &UserId) -> Option<User> {
		use crate::database::entities::exports::get_user_by_id;

		get_user_by_id(conn, UserIdWrapper(*user_id)).map(User::from)
	}

	pub fn get_audios(
		&self,
		conn: &mut diesel::PgConnection,
	) -> Vec<crate::database::models::audio::Audio> {
		get_audios_from_user(conn, self.id)
			.into_iter()
			.map(crate::database::models::audio::Audio::from)
			.collect()
	}
}

impl From<SerenityUser> for User {
	fn from(value: SerenityUser) -> Self {
		Self {
			id: UserIdWrapper(value.id),
			username: value.name,
			added_at: Utc::now(),
			updated_at: Utc::now(),
			syncronized_with_database: false,
		}
	}
}

impl From<UserEntity> for User {
	fn from(user: UserEntity) -> Self {
		Self {
			id: user.id,
			username: user.username,
			added_at: user.added_at,
			updated_at: user.updated_at,
			syncronized_with_database: true,
		}
	}
}
