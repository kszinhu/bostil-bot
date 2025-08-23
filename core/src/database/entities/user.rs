use chrono::{DateTime, Utc};
use diesel::prelude::*;

use super::UserIdWrapper;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
	pub id: UserIdWrapper,
	pub username: String,
	pub added_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
}

pub fn get_user_by_id(conn: &mut PgConnection, user_id: UserIdWrapper) -> Option<User> {
	use crate::schema::users::dsl::{id, users};

	users
		.filter(id.eq(user_id))
		.first::<User>(conn)
		.optional()
		.expect("Error loading user")
}

pub fn get_users(conn: &mut PgConnection, filter: Option<&str>) -> Vec<User> {
	use crate::schema::users::dsl::{username, users};

	match filter {
		Some(f) => users
			.filter(username.ilike(format!("%{}%", f)))
			.load::<User>(conn)
			.expect("Error loading users"),
		None => users.load::<User>(conn).expect("Error loading users"),
	}
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser<'a> {
	pub id: UserIdWrapper,
	pub username: &'a str,
}

pub fn create_user(
	conn: &mut PgConnection,
	user_id: UserIdWrapper,
	username: &str,
) -> QueryResult<User> {
	use crate::schema::users;

	let new_user = NewUser {
		id: user_id,
		username,
	};

	diesel::insert_into(users::table)
		.values(&new_user)
		.on_conflict(users::id)
		.do_update()
		.set(users::username.eq(username))
		.returning(User::as_returning())
		.get_result(conn)
}
