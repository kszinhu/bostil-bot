use diesel::prelude::*;

use super::{GuildIdWrapper, Language};

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::schema::guilds)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Guild {
    pub id: GuildIdWrapper,
    pub language: Language,
    pub added_at: time::OffsetDateTime,
    pub updated_at: time::OffsetDateTime,
}
