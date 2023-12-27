use crate::database::{get_database, Database};

use super::{
    ArgumentsLevel, Command, CommandCategory, CommandResponse, InternalCommandResult, RunnerFn,
};

use regex::Regex;
use rust_i18n::t;
use serenity::{
    async_trait,
    model::{
        id::{GuildId, MessageId},
        prelude::{
            application_command::{CommandDataOption, CommandDataOptionValue},
            ChannelId, UserId,
        },
    },
};
use std::time::{Duration, SystemTime};

mod database;
pub mod help;
pub mod management;
pub mod setup;
mod utils;

struct PollCommand;

#[derive(Debug, Clone)]
pub struct Vote {
    pub user_id: UserId,
    pub options: Vec<String>,
}

#[derive(Debug, Clone, Copy)]
pub enum PollType {
    SingleChoice,
    MultipleChoice,
}

#[derive(Debug, Clone, Copy)]
pub enum PollStatus {
    Open,
    Closed,
    Stopped,
    Creating,
}

#[derive(Debug)]
pub struct Poll {
    id: uuid::Uuid,
    name: String,
    description: Option<String>,
    kind: PollType,
    options: Vec<String>,
    timer: Duration,
    status: PollStatus,
}

#[derive(Debug, Clone)]
pub struct PollDatabaseModel {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: Option<String>,
    pub kind: PollType,
    pub status: PollStatus,
    pub timer: Option<Duration>,
    pub options: Vec<String>,
    pub votes: Vec<Vote>,
    pub thread_id: ChannelId,
    pub message_id: MessageId,
    pub created_at: SystemTime,
    pub created_by: UserId,
}

impl PollDatabaseModel {
    pub fn from_id(id: uuid::Uuid) -> PollDatabaseModel {
        let database_manager = get_database();
        let database = database_manager.lock().unwrap();

        let poll = database
            .guilds
            .iter()
            .find_map(|(_, guild)| guild.polls.iter().find(|poll| poll.id == id));

        poll.unwrap().clone()
    }
}

impl Poll {
    pub fn new(
        name: String,
        description: Option<String>,
        kind: PollType,
        options: Vec<String>,
        // Receives a minute value as a string (e.g. "0.5" for 30 seconds, "1" for 1 minute, "2" for 2 minutes, etc.)
        timer: Option<String>,
        status: Option<PollStatus>,
    ) -> Poll {
        Poll {
            name,
            description,
            kind,
            options,
            id: uuid::Uuid::new_v4(),
            status: status.unwrap_or(PollStatus::Open),
            timer: match timer {
                Some(timer) => {
                    let timer = timer.parse::<f64>().unwrap_or(0.0);
                    Duration::from_secs_f64(timer * 60.0)
                }
                None => Duration::from_secs(60),
            },
        }
    }

    pub fn save(
        &self,
        user_id: UserId,
        channel_id: ChannelId,
        guild_id: GuildId,
        message_id: MessageId,
    ) {
        let poll = PollDatabaseModel {
            message_id,
            id: self.id,
            name: self.name.clone(),
            description: self.description.clone(),
            kind: self.kind,
            status: self.status,
            options: self.options.clone(),
            timer: Some(self.timer),
            votes: vec![],
            thread_id: channel_id,
            created_at: SystemTime::now(),
            created_by: user_id,
        };

        poll.save(guild_id);
    }

    pub fn from_id(id: uuid::Uuid) -> PollDatabaseModel {
        PollDatabaseModel::from_id(id)
    }
}

impl PollType {
    pub fn to_string(&self) -> String {
        match self {
            PollType::SingleChoice => "single_choice".to_string(),
            PollType::MultipleChoice => "multiple_choice".to_string(),
        }
    }

    pub fn to_label(&self) -> String {
        match self {
            PollType::SingleChoice => t!("commands.poll.types.single_choice.label"),
            PollType::MultipleChoice => t!("commands.poll.types.single_choice.label"),
        }
    }
}

impl PollStatus {
    pub fn to_string(&self) -> String {
        match self {
            PollStatus::Open => "open".to_string(),
            PollStatus::Closed => "closed".to_string(),
            PollStatus::Stopped => "stopped".to_string(),
            PollStatus::Creating => "creating".to_string(),
        }
    }
}

fn poll_serializer(command_options: &Vec<CommandDataOption>) -> Poll {
    let option_regex: Regex = Regex::new(r"^option_\d+$").unwrap();
    let kind = match command_options.iter().find(|option| option.name == "type") {
        Some(option) => match option.resolved.as_ref().unwrap() {
            CommandDataOptionValue::String(value) => match value.as_str() {
                "single_choice" => PollType::SingleChoice,
                "multiple_choice" => PollType::MultipleChoice,
                _ => PollType::SingleChoice,
            },
            _ => PollType::SingleChoice,
        },
        None => PollType::SingleChoice,
    };

    Poll::new(
        command_options
            .iter()
            .find(|option| option.name == "name")
            .unwrap()
            .value
            .as_ref()
            .unwrap()
            .to_string(),
        Some(
            command_options
                .iter()
                .find(|option| option.name == "description")
                .unwrap()
                .value
                .as_ref()
                .unwrap()
                .to_string(),
        ),
        kind,
        command_options
            .iter()
            .filter(|option| option_regex.is_match(&option.name))
            .map(|option| match option.resolved.as_ref().unwrap() {
                CommandDataOptionValue::String(value) => value.to_string(),
                _ => "".to_string(),
            })
            .collect::<Vec<String>>(),
        Some(
            command_options
                .iter()
                .find(|option| option.name == "timer")
                .unwrap()
                .value
                .as_ref()
                .unwrap()
                .to_string(),
        ),
        Some(PollStatus::Open),
    )
}

#[async_trait]
impl RunnerFn for PollCommand {
    async fn run<'a>(
        &self,
        args: &Vec<Box<dyn std::any::Any + Send + Sync>>,
    ) -> InternalCommandResult<'a> {
        let options = args
            .iter()
            .filter_map(|arg| arg.downcast_ref::<Option<Vec<CommandDataOption>>>())
            .collect::<Vec<&Option<Vec<CommandDataOption>>>>()[0]
            .as_ref()
            .unwrap();
        let first_option = options.get(0).unwrap();
        let command_name = first_option.name.clone();

        let command_runner = command_suite(command_name);

        let response = command_runner.run(args.clone());

        match response.await {
            Ok(response) => match response.to_owned() {
                CommandResponse::Message(message) => Ok(CommandResponse::Message(message)),
                _ => Ok(CommandResponse::None),
            },
            Err(e) => Err(e),
        }
    }
}

fn command_suite(command_name: String) -> Box<dyn RunnerFn + std::marker::Send + Sync> {
    let command_runner = match command_name.as_str() {
        "help" => self::help::get_command().runner,
        "setup" => self::setup::create::get_command().runner,
        "options" => self::setup::options::get_command().runner,
        _ => get_command().runner,
    };

    command_runner
}

pub fn get_command() -> Command {
    Command::new(
        "poll",
        "Poll commands",
        CommandCategory::Misc,
        vec![
            ArgumentsLevel::Options,
            ArgumentsLevel::Context,
            ArgumentsLevel::Guild,
            ArgumentsLevel::User,
            ArgumentsLevel::ChannelId,
        ],
        Box::new(PollCommand),
    )
}
