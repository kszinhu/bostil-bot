use std::fs;

use once_cell::sync::Lazy;
use serde_json::Value;

pub const USERS_FILE: &str = "/src/static/users.json";

fn parser_untyped(json_data: String) -> Result<Value, serde_json::Error> {
    let v: Value = serde_json::from_str(json_data.as_str())?;
    Ok(v)
}

fn get_users() -> serde_json::Value {
    let users = fs::read_to_string(format!("{}{}", env!("CARGO_MANIFEST_DIR"), USERS_FILE))
        .expect("Something went wrong reading the file");

    let users = parser_untyped(users);

    users.unwrap()
}

pub const USERS: Lazy<serde_json::Value> = Lazy::new(|| get_users());
