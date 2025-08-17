use chrono::{DateTime, Utc};
use diesel::prelude::*;

use super::{GuildIdWrapper, Language};

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::schema::guilds)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Guild {
    pub id: GuildIdWrapper,
    pub language: Language,
    pub added_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub fn get_guild_by_id(conn: &mut PgConnection, guild_id: GuildIdWrapper) -> Option<Guild> {
    use crate::schema::guilds::dsl::{guilds, id};

    guilds
        .filter(id.eq(guild_id))
        .first::<Guild>(conn)
        .optional()
        .expect("Error loading guild")
}

pub fn get_guilds(conn: &mut PgConnection) -> Vec<Guild> {
    use crate::schema::guilds::dsl::guilds;

    guilds.load::<Guild>(conn).expect("Error loading guilds")
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::guilds)]
pub struct NewGuild {
    pub id: GuildIdWrapper,
    pub language: Language,
}

pub fn create_guild(
    conn: &mut PgConnection,
    guild_id: GuildIdWrapper,
    language: Language,
) -> QueryResult<Guild> {
    use crate::schema::guilds;

    let new_guild = NewGuild {
        id: guild_id,
        language,
    };

    diesel::insert_into(guilds::table)
        .values(&new_guild)
        .on_conflict(guilds::id)
        .do_update()
        .set(guilds::language.eq(language))
        .returning(Guild::as_returning())
        .get_result(conn)
}

pub fn update_guild_language(
    conn: &mut PgConnection,
    guild_id: GuildIdWrapper,
    new_language: Language,
) -> QueryResult<Guild> {
    use crate::schema::guilds::dsl::{guilds, id, language};

    diesel::update(guilds.filter(id.eq(guild_id)))
        .set(language.eq(new_language))
        .returning(Guild::as_returning())
        .get_result(conn)
}
