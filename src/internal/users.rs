use std::{collections::HashMap, fs};

use once_cell::sync::Lazy;
use serenity::model::prelude::UserId;

use crate::internal::constants::USERS_FILE_PATH;

fn parse_users(users_json: String) -> Result<HashMap<String, UserId>, serde_json::Error> {
    let v: HashMap<String, UserId> = serde_json::from_str(users_json.as_str())?;
    Ok(v)
}

fn get_users() -> HashMap<String, UserId> {
    let users = fs::read_to_string(USERS_FILE_PATH).expect("Something went wrong reading the file");

    let users = parse_users(users);

    users.unwrap()
}

pub const USERS: Lazy<HashMap<String, UserId>> = Lazy::new(|| get_users());
