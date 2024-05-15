use crate::modules::core::entities::UserIdWrapper;

#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub id: UserIdWrapper,
    pub username: String,
    pub added_at: time::OffsetDateTime,
    pub updated_at: time::OffsetDateTime,
}
