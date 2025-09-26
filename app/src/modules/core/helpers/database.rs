use diesel::{pg::PgConnection, Connection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use dotenvy::dotenv;

pub fn establish_connection() -> PgConnection {
	use std::env;

	dotenv().ok();

	let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
	PgConnection::establish(&database_url)
		.unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");
