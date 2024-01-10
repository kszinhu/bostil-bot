use diesel::pg::PgConnection;
use diesel::Connection;
use dotenvy::dotenv;
use std::env;

#[macro_use(i18n)]
extern crate rust_i18n;
extern crate diesel;

// TODO: implementar algum jeito para que cada servidor tenha seu próprio idioma e não alterar o idioma de todos os servidores // CHECK Backend implementation
i18n!("public/locales", fallback = "en-US");

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub mod modules;
pub mod schema;
