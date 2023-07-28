pub mod locale;

/*
    Create a database file (.yml) on DATABASE_PATH path with the following content:

    {GUILD_ID}:
        locale: "en-US"

    using Arc and Mutex to share the parsed database between threads
*/

use std::collections::HashMap;
use std::env;
use std::fmt::Debug;
use std::fs::File;
use std::io::prelude::*;
use std::sync::{Arc, Mutex};

use serenity::model::id::GuildId;
use serenity::prelude::*;

use yaml_rust::YamlLoader;

#[derive(Debug)]
pub struct Database {
    pub locale: HashMap<GuildId, String>,
}

impl TypeMapKey for Database {
    type Value = Arc<Mutex<Database>>;
}

fn open_or_create_file(database_path: &String) -> File {
    let file = File::open(database_path);

    match file {
        Ok(file) => file,
        Err(_) => {
            // check if directory exists and create if not
            let mut dir_path = std::path::PathBuf::from(database_path);

            dir_path.pop();
            if !dir_path.exists() {
                std::fs::create_dir_all(dir_path).expect("Failed to create directory");
            }

            File::create(database_path).expect("Something went wrong creating the file")
        }
    }
}

pub fn get_database() -> Arc<Mutex<Database>> {
    let database_path = env::var("DATABASE_PATH").expect("DATABASE_PATH not found");

    let mut file = open_or_create_file(&database_path);
    let mut contents = String::new();

    if file.metadata().unwrap().len() > 0 {
        file.read_to_string(&mut contents)
            .expect("Something went wrong reading the file");
    }

    let docs = YamlLoader::load_from_str(&contents).unwrap();

    let mut database = Database {
        locale: HashMap::new(),
    };

    for doc in docs {
        for (guild_id, guild) in doc.as_hash().unwrap() {
            let guild_id = GuildId(guild_id.as_i64().unwrap() as u64);

            let locale = guild["locale"].as_str().unwrap().to_string();

            database.locale.insert(guild_id, locale);
        }
    }

    Arc::new(Mutex::new(database))
}

pub fn save_database(database: &Database) {
    let database_path = env::var("DATABASE_PATH").expect("DATABASE_PATH not found");

    let mut file = File::create(database_path).expect("Database file not found");

    let mut contents = String::new();

    for (guild_id, locale) in &database.locale {
        contents.push_str(&format!("{}:\n", guild_id));
        contents.push_str(&format!("    locale: \"{}\"\n", locale));
    }

    file.write_all(contents.as_bytes())
        .expect("Something went wrong writing the file");
}
