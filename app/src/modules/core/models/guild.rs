use crate::modules::core::entities::{GuildIdWrapper, Language};

#[derive(Clone, Debug, PartialEq)]
pub struct Guild {
    pub id: GuildIdWrapper,
    pub language: Language,
    pub added_at: time::OffsetDateTime,
    pub updated_at: time::OffsetDateTime,
}
