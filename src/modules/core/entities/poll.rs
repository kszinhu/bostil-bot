use diesel::prelude::*;

use super::{ChannelIdWrapper, MessageIdWrapper, PollKind, UserIdWrapper};

#[derive(Queryable, Selectable, Identifiable, Insertable)]
#[diesel(table_name = crate::schema::polls)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Poll {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: Option<String>,
    pub kind: PollKind,
    pub thread_id: ChannelIdWrapper,
    pub embed_message_id: MessageIdWrapper,
    pub poll_message_id: MessageIdWrapper,
    pub started_at: Option<time::OffsetDateTime>,
    pub ended_at: Option<time::OffsetDateTime>,
    pub created_at: time::OffsetDateTime,
    pub created_by: UserIdWrapper,
}

#[derive(Queryable, Selectable, Identifiable, Insertable)]
#[diesel(primary_key(poll_id, value))]
#[diesel(table_name = crate::schema::poll_choices)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PollChoice {
    pub poll_id: uuid::Uuid,
    pub value: String,
    pub label: String,
    pub description: Option<String>,
    pub created_at: time::OffsetDateTime,
}

#[derive(Queryable, Selectable, Identifiable, Insertable)]
#[diesel(primary_key(user_id, poll_id, choice_value))]
#[diesel(table_name = crate::schema::poll_votes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PollVote {
    pub poll_id: uuid::Uuid,
    pub choice_value: String,
    pub user_id: UserIdWrapper,
    pub voted_at: time::OffsetDateTime,
}
