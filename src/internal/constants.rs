pub const USERS_FILE_PATH: &str = "./public/static/users.json";

#[derive(Debug, Clone)]
pub struct CommandHelp {
    pub name: String,
    pub description: String,
    pub options: Vec<String>,
}
