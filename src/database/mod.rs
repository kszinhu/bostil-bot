pub mod locale;

/*
    Create a database file (.yml) on DATABASE_PATH path with the following content:

    {GUILD_ID}:
        locale: "en-US"
        polls:
            - id: {POLL_ID}
              name: "Poll name"
              description: "Poll description"
              kind: "single_choice"
              options:
                  - "Option 1"
                  - "Option 2"
              timer: 60
              votes:
                  - user_id: "USER_ID"
                    options:
                        - "Option 1"
                  - user_id: "USER_ID"
                    options:
                        - "Option 1"
                        - "Option 2"
             created_at: 2021-01-01T00:00:00Z

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

use crate::commands::poll::PollDatabaseModel;

#[derive(Debug)]
pub struct GuildDatabaseModel {
    pub locale: String,
    pub polls: Vec<PollDatabaseModel>,
}

#[derive(Debug)]
pub struct Database {
    pub guilds: HashMap<GuildId, GuildDatabaseModel>,
}

impl TypeMapKey for Database {
    type Value = Arc<Mutex<Database>>;
}

impl Database {
    pub fn init() -> Arc<Mutex<Database>> {
        // TODO: CREATE INIT FOR DATABASE FILE
        let database = get_database();

        Arc::clone(&database)
    }
}

fn open_or_create_file(database_path: &String) -> File {
    let file = File::open(database_path);

    match file {
        Ok(file) => file,
        Err(_) => {
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
        guilds: HashMap::new(),
    };

    for doc in docs {
        for (guild_id, guild) in doc.as_hash().unwrap() {
            let guild_id = GuildId(guild_id.as_i64().unwrap() as u64);

            let locale = match guild["locale"].as_str() {
                Some(locale) => locale.to_string(),
                None => "".to_string(),
            };
            let polls = match guild["polls"].as_vec() {
                Some(polls) => polls.to_vec(),
                None => vec![] as Vec<yaml_rust::Yaml>,
            };

            database.guilds.insert(
                guild_id,
                GuildDatabaseModel {
                    locale,
                    polls: polls
                        .iter()
                        .map(|poll| PollDatabaseModel::from_yaml(poll))
                        .collect::<Vec<PollDatabaseModel>>(),
                },
            );
        }
    }

    Arc::new(Mutex::new(database))
}

pub fn save_database(database: &Database) {
    let database_path = env::var("DATABASE_PATH").expect("DATABASE_PATH not found");

    let mut file = File::create(database_path).expect("Database file not found");

    let mut contents = String::new();

    for (guild_id, guild) in &database.guilds {
        contents.push_str(&format!("{}:\n", guild_id.as_u64()));

        contents.push_str(&format!("  locale: \"{}\"\n", guild.locale));
        contents.push_str("  polls:\n");

        guild.polls.iter().for_each(|poll| {
            println!("chegou no iter poll");

            contents.push_str(&format!("    - id: {}\n", poll.id));

            contents.push_str(&format!("      name: \"{}\"\n", poll.name));

            if let Some(description) = &poll.description {
                contents.push_str(&format!("      description: \"{}\"\n", description));
            }

            contents.push_str(&format!("      kind: \"{}\"\n", poll.kind.to_string()));

            contents.push_str("      options:\n");

            for option in &poll.options {
                contents.push_str(&format!("        - \"{}\"\n", option));
            }

            contents.push_str(&format!("      timer: {}\n", poll.timer.unwrap().as_secs()));

            contents.push_str("      votes:\n");

            for vote in &poll.votes {
                contents.push_str(&format!("        - user_id: {}\n", vote.user_id));

                for option in &vote.options {
                    contents.push_str(&format!("          - \"{}\"\n", option));
                }
            }

            contents.push_str(&format!(
                "      created_at: {}\n",
                poll.created_at
                    .duration_since(std::time::SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
            ));
        });
    }

    file.write_all(contents.as_bytes())
        .expect("Something went wrong writing the file");
}
