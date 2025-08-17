use diesel::{
	migration::{MigrationVersion, Result},
	pg::{Pg as Postgres, PgConnection},
	Connection,
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenvy::dotenv;

pub fn establish_connection() -> PgConnection {
	use std::env;

	dotenv().ok();

	let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
	PgConnection::establish(&database_url)
		.unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub fn run_migrations(
	connection: &mut impl MigrationHarness<Postgres>,
) -> Result<Vec<MigrationVersion<'_>>> {
	match connection.has_pending_migration(MIGRATIONS) {
		Ok(true) => connection.run_pending_migrations(MIGRATIONS),
		Ok(false) => Ok(vec![]),
		Err(e) => Err(e),
	}
}
