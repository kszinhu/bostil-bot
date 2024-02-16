use diesel::{pg::PgConnection, Connection};
use dotenvy::dotenv;

// TODO: implementar algum jeito para que cada servidor tenha seu próprio idioma e não alterar o idioma de todos os servidores
i18n!("public/locales", fallback = "en-US");

pub fn establish_connection() -> PgConnection {
    use std::env;

    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
