use diesel::prelude::*;

use super::{ChannelIdWrapper, MessageIdWrapper, PollKind, PollState, UserIdWrapper};

#[derive(Queryable, Selectable, Identifiable, Insertable, Debug, Clone)]
#[diesel(table_name = crate::schema::polls)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Poll {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: Option<String>,
    pub kind: PollKind,
    pub state: PollState,
    pub thread_id: ChannelIdWrapper,
    pub embed_message_id: MessageIdWrapper,
    pub poll_message_id: Option<MessageIdWrapper>,
    pub started_at: Option<time::OffsetDateTime>,
    pub ended_at: Option<time::OffsetDateTime>,
    pub created_at: time::OffsetDateTime,
    pub created_by: UserIdWrapper,
}

#[derive(Queryable, Selectable, Identifiable, Insertable, Associations, Debug, Clone)]
#[diesel(belongs_to(Poll))]
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

#[derive(Queryable, Selectable, Identifiable, Insertable, Associations, Debug, Clone)]
#[diesel(belongs_to(PollChoice, foreign_key = choice_value))]
#[diesel(belongs_to(Poll))]
#[diesel(primary_key(user_id, poll_id, choice_value))]
#[diesel(table_name = crate::schema::poll_votes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PollVote {
    pub poll_id: uuid::Uuid,
    pub choice_value: String,
    pub user_id: UserIdWrapper,
    pub voted_at: time::OffsetDateTime,
}

#[derive(Debug)]
pub struct PollWithChoicesAndVotes {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: Option<String>,
    pub kind: PollKind,
    pub state: PollState,
    pub thread_id: ChannelIdWrapper,
    pub embed_message_id: MessageIdWrapper,
    pub poll_message_id: Option<MessageIdWrapper>,
    pub started_at: Option<time::OffsetDateTime>,
    pub ended_at: Option<time::OffsetDateTime>,
    pub created_at: time::OffsetDateTime,
    pub created_by: UserIdWrapper,
    pub choices: Vec<PollChoice>,
    pub votes: Vec<PollVote>,
}

impl PollWithChoicesAndVotes {
    pub fn from(polls: Vec<(Poll, PollChoice, PollVote)>) -> Self {
        let poll = polls[0].0.clone();
        let mut choices = Vec::new();
        let mut votes = Vec::new();

        for (_, choice, vote) in polls {
            choices.push(choice);
            votes.push(vote);
        }

        Self {
            id: poll.id,
            name: poll.name,
            description: poll.description,
            kind: poll.kind,
            state: poll.state,
            thread_id: poll.thread_id,
            embed_message_id: poll.embed_message_id,
            poll_message_id: poll.poll_message_id,
            started_at: poll.started_at,
            ended_at: poll.ended_at,
            created_at: poll.created_at,
            created_by: poll.created_by,
            choices,
            votes,
        }
    }
}
