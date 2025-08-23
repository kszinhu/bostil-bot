// @generated automatically by Diesel CLI.

pub mod sql_types {
	#[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
	#[diesel(postgres_type(name = "language"))]
	pub struct Language;
}

diesel::table! {
		audios (id) {
				id -> Int8,
				user_id -> Int8,
				content -> Bytea,
				caption -> Nullable<Text>,
				added_at -> Timestamptz,
				updated_at -> Timestamptz,
		}
}

diesel::table! {
		use diesel::sql_types::*;
		use super::sql_types::Language;

		guilds (id) {
				id -> Int8,
				language -> Language,
				added_at -> Timestamptz,
				updated_at -> Timestamptz,
		}
}

diesel::table! {
		users (id) {
				id -> Int8,
				#[max_length = 255]
				username -> Varchar,
				added_at -> Timestamptz,
				updated_at -> Timestamptz,
		}
}

diesel::joinable!(audios -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(audios, guilds, users,);
