use crate::database::entities::UserIdWrapper;
use chrono::{DateTime, Utc};
use diesel::prelude::*;

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::schema::audios)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Audio {
    pub id: i64,
    pub user_id: UserIdWrapper,
    pub content: Vec<u8>,
    pub caption: Option<String>,
    pub added_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub fn get_audios_from_user(conn: &mut PgConnection, user_id_filter: UserIdWrapper) -> Vec<Audio> {
    use crate::schema::audios::dsl::{audios, user_id};

    audios
        .filter(user_id.eq(user_id_filter))
        .load::<Audio>(conn)
        .unwrap_or_else(|_| vec![])
}

pub fn get_audio_from_content(
    conn: &mut PgConnection,
    content_filter: &str,
) -> QueryResult<Option<Audio>> {
    use crate::schema::audios::dsl::{audios, caption};

    audios
        .filter(caption.ilike(format!("%{}%", content_filter)))
        .first::<Audio>(conn)
        .optional()
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::audios)]
pub struct NewAudio<'a> {
    pub content: &'a [u8],
    pub caption: Option<&'a str>,
    pub user_id: UserIdWrapper,
}

pub fn save_audio(
    conn: &mut PgConnection,
    content: &[u8],
    user_id: UserIdWrapper,
    caption: Option<&str>,
) -> QueryResult<Audio> {
    use crate::schema::audios;

    let new_audio = NewAudio {
        content,
        caption,
        user_id,
    };

    diesel::insert_into(audios::table)
        .values(&new_audio)
        .returning(Audio::as_returning())
        .get_result(conn)
}
