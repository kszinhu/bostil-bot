// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType, std::fmt::Debug)]
    #[diesel(postgres_type(name = "language"))]
    pub struct Language;

    #[derive(diesel::sql_types::SqlType, std::fmt::Debug)]
    #[diesel(postgres_type(name = "poll_kind"))]
    pub struct PollKind;

    #[derive(diesel::sql_types::SqlType, std::fmt::Debug)]
    #[diesel(postgres_type(name = "poll_state"))]
    pub struct PollState;
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::modules::core::entities::exports::*;
    use super::sql_types::Language;

    guilds (id) {
        id -> Int8,
        language -> Language,
        added_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::modules::core::entities::exports::*;

    poll_choices (poll_id, value) {
        poll_id -> Uuid,
        #[max_length = 50]
        value -> Varchar,
        #[max_length = 25]
        label -> Varchar,
        description -> Nullable<Text>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::modules::core::entities::exports::*;

    poll_votes (user_id, choice_value, poll_id) {
        user_id -> Int8,
        #[max_length = 50]
        choice_value -> Varchar,
        poll_id -> Uuid,
        voted_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::modules::core::entities::exports::*;
    use super::sql_types::PollKind;
    use super::sql_types::PollState;

    polls (id) {
        id -> Uuid,
        #[max_length = 50]
        name -> Varchar,
        description -> Nullable<Text>,
        kind -> PollKind,
        state -> PollState,
        timer -> Int8,
        thread_id -> Int8,
        embed_message_id -> Int8,
        poll_message_id -> Nullable<Int8>,
        started_at -> Nullable<Timestamptz>,
        ended_at -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
        created_by -> Int8,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::modules::core::entities::exports::*;

    users (id) {
        id -> Int8,
        #[max_length = 255]
        username -> Varchar,
        added_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(poll_choices -> polls (poll_id));
diesel::joinable!(poll_votes -> polls (poll_id));

diesel::allow_tables_to_appear_in_same_query!(guilds, poll_choices, poll_votes, polls, users,);
