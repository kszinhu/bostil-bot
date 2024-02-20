use diesel::prelude::*;

use super::UserIdWrapper;

#[derive(Queryable, Selectable, Identifiable, Insertable, PartialEq)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: UserIdWrapper,
    pub username: String,
    pub added_at: time::OffsetDateTime,
    pub updated_at: time::OffsetDateTime,
}
