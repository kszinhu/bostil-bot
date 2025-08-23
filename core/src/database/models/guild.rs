use crate::database::{
	entities::exports::{Guild as GuildEntity, GuildIdWrapper, Language},
	exports::establish_connection,
};
use chrono::{DateTime, Utc};
use serenity::all::{Guild as SerenityGuild, GuildId};
use tracing::error;

#[derive(Clone, Debug, PartialEq)]
pub struct Guild {
	pub id: GuildIdWrapper,
	pub language: Language,
	pub added_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
	syncronized_with_database: bool,
}

const DEFAULT_LANGUAGE: Language = Language::Pt;

impl Guild {
	pub fn save(&self, conn: &mut diesel::PgConnection) -> Result<Guild, diesel::result::Error> {
		use crate::database::entities::exports::create_guild;

		create_guild(conn, self.id, self.language).map(Guild::from)
	}

	pub fn update_language(
		&self,
		conn: &mut diesel::PgConnection,
		new_language: Language,
	) -> Result<Guild, diesel::result::Error> {
		use crate::database::entities::exports::update_guild_language;

		update_guild_language(conn, self.id, new_language).map(Guild::from)
	}

	pub fn get_all(conn: &mut diesel::PgConnection) -> Vec<Guild> {
		use crate::database::entities::exports::get_guilds;

		get_guilds(conn).into_iter().map(Guild::from).collect()
	}

	pub fn get_by_id(conn: &mut diesel::PgConnection, guild_id: GuildIdWrapper) -> Option<Guild> {
		use crate::database::entities::exports::get_guild_by_id;

		get_guild_by_id(conn, guild_id).map(Guild::from)
	}

	pub fn new(guild_id: GuildId) -> Self {
		use crate::database::entities::exports::create_guild;

		let connection = &mut establish_connection();

		match create_guild(connection, GuildIdWrapper(guild_id), DEFAULT_LANGUAGE) {
			Ok(guild) => Guild::from(guild),
			Err(e) => {
				error!("Error creating guild in database: {}", e);
				Self {
					id: GuildIdWrapper(guild_id),
					language: DEFAULT_LANGUAGE,
					added_at: Utc::now(),
					updated_at: Utc::now(),
					syncronized_with_database: false,
				}
			}
		}
	}
}

impl From<GuildEntity> for Guild {
	fn from(guild: GuildEntity) -> Self {
		Self {
			id: guild.id,
			language: guild.language,
			added_at: guild.added_at,
			updated_at: guild.updated_at,
			syncronized_with_database: true,
		}
	}
}

impl From<SerenityGuild> for Guild {
	fn from(value: SerenityGuild) -> Self {
		use crate::database::exports::establish_connection;

		let connection = &mut establish_connection();

		match Guild::get_by_id(connection, GuildIdWrapper(value.id)) {
			Some(guild) => guild,
			None => Guild::new(value.id),
		}
	}
}
