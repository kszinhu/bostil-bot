use std::borrow::BorrowMut;

use super::{PartialPoll, Poll, PollDatabaseModel as PollModel, PollStatus, PollType, Vote};
use crate::database::{get_database, save_database, GuildDatabaseModel};
use crate::internal::debug::{log_message, MessageTypes};

use serenity::model::prelude::{ChannelId, GuildChannel, GuildId, UserId};
use yaml_rust::Yaml;

impl PollModel {
    pub fn from(
        poll: &Poll,
        votes: Vec<Vote>,
        user_id: &UserId,
        thread_id: &ChannelId,
    ) -> PollModel {
        PollModel {
            votes,
            id: poll.id,
            kind: poll.kind,
            timer: poll.timer,
            status: poll.status,
            name: poll.name.clone(),
            description: poll.description.clone(),
            options: poll.options.clone(),
            thread_id: thread_id.clone(),
            partial: false,
            created_at: std::time::SystemTime::now(),
            created_by: user_id.clone(),
        }
    }

    pub fn from_yaml(yaml: &Yaml) -> PollModel {
        PollModel {
            votes: Vec::new(),
            id: uuid::Uuid::parse_str(yaml["id"].as_str().unwrap()).unwrap(),
            name: yaml["name"].as_str().unwrap().to_string(),
            description: match yaml["description"].as_str() {
                Some(description) => Some(description.to_string()),
                None => None,
            },
            kind: match yaml["kind"].as_str().unwrap() {
                "single_choice" => PollType::SingleChoice,
                "multiple_choice" => PollType::MultipleChoice,
                _ => PollType::SingleChoice,
            },
            options: yaml["options"]
                .as_vec()
                .unwrap()
                .iter()
                .map(|option| option.as_str().unwrap().to_string())
                .collect::<Vec<String>>(),
            timer: std::time::Duration::from_secs(yaml["timer"].as_i64().unwrap() as u64),
            thread_id: ChannelId(yaml["thread_id"].as_i64().unwrap().try_into().unwrap()),
            created_at: std::time::SystemTime::UNIX_EPOCH
                + std::time::Duration::from_secs(yaml["created_at"].as_i64().unwrap() as u64),
            status: match yaml["status"].as_str().unwrap() {
                "open" => PollStatus::Open,
                "closed" => PollStatus::Closed,
                "stopped" => PollStatus::Stopped,
                _ => PollStatus::Open,
            },
            partial: yaml["partial"].as_bool().unwrap(),
            created_by: UserId(yaml["created_by"].as_i64().unwrap() as u64),
        }
    }

    fn from_partial_poll(partial_poll: &PartialPoll) -> PollModel {
        let poll = Poll::new(
            partial_poll.name.clone(),
            partial_poll.description.clone(),
            partial_poll.kind,
            vec![],
            None,
            Some(PollStatus::Creating),
        );

        PollModel::from(
            &poll,
            vec![],
            &partial_poll.created_by,
            &partial_poll.thread_id,
        )
    }
}

pub fn save_poll(
    guild_id: GuildId,
    user_id: &UserId,
    thread_id: &ChannelId,
    poll: &Poll,
    votes: Vec<Vote>,
) {
    let database = get_database();
    let poll_model = PollModel::from(poll, votes, user_id, thread_id);

    if let Some(guild) = database.lock().unwrap().guilds.get_mut(&guild_id) {
        guild.polls.push(poll_model);
    } else {
        database.lock().unwrap().guilds.insert(
            guild_id,
            GuildDatabaseModel {
                locale: "en-US".to_string(),
                polls: vec![poll_model],
            },
        );
    }

    save_database(database.lock().unwrap().borrow_mut());
}
