use std::{collections::HashMap, fs};

use once_cell::sync::Lazy;
use serenity::model::prelude::UserId;

pub const USERS_FILE: &str = "/src/static/users.json";

fn parser_untyped(json_data: String) -> Result<HashMap<String, UserId>, serde_json::Error> {
    let v: HashMap<String, UserId> = serde_json::from_str(json_data.as_str())?;
    Ok(v)
}

fn get_users() -> HashMap<String, UserId> {
    let users = fs::read_to_string(format!("{}{}", env!("CARGO_MANIFEST_DIR"), USERS_FILE))
        .expect("Something went wrong reading the file");

    let users = parser_untyped(users);

    users.unwrap()
}

pub const USERS: Lazy<HashMap<String, UserId>> = Lazy::new(|| get_users());
