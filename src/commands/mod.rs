use std::any::Any;

use serenity::{
    async_trait,
    builder::{CreateEmbed, CreateInteractionResponseData},
    framework::standard::CommandResult,
    model::prelude::Embed,
};

use crate::internal::arguments::ArgumentsLevel;

pub mod jingle;
pub mod language;
pub mod ping;
pub mod poll;
pub mod radio;
pub mod voice;

#[derive(Debug, Clone, Copy)]
pub enum CommandCategory {
    Fun,
    Moderation,
    Music,
    Misc,
    Voice,
    Admin,
    General,
}

pub struct Command {
    pub name: String,
    pub description: String,
    pub category: CommandCategory,
    pub arguments: Vec<ArgumentsLevel>,
    pub runner: Box<dyn RunnerFn + Send + Sync>,
}

impl Command {
    pub fn new(
        name: &str,
        description: &str,
        category: CommandCategory,
        arguments: Vec<ArgumentsLevel>,
        runner: Box<dyn RunnerFn + Send + Sync>,
    ) -> Self {
        let sorted_arguments = {
            let mut sorted_arguments = arguments.clone();
            sorted_arguments.sort_by(|a, b| a.value().cmp(&b.value()));
            sorted_arguments
        };

        Self {
            arguments: sorted_arguments,
            category,
            runner,
            description: description.to_string(),
            name: name.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum CommandResponse<'a> {
    String(String),
    Embed(Embed),
    Message(CreateInteractionResponseData<'a>),
    None,
}

impl CommandResponse<'_> {
    pub fn to_embed(&self) -> CreateEmbed {
        match self {
            CommandResponse::String(string) => {
                let mut embed = CreateEmbed::default();
                embed.description(string);

                embed
            }
            CommandResponse::Embed(command_embed) => {
                let mut embed = CreateEmbed::default();
                embed.author(|a| {
                    a.name(command_embed.author.clone().unwrap().name.clone())
                        .icon_url(command_embed.author.clone().unwrap().icon_url.unwrap())
                        .url(command_embed.author.clone().unwrap().url.unwrap())
                });
                embed.title(command_embed.title.clone().unwrap());
                embed.description(command_embed.description.clone().unwrap());
                embed.fields(
                    command_embed
                        .fields
                        .clone()
                        .iter()
                        .map(|field| (field.name.clone(), field.value.clone(), field.inline)),
                );
                embed.colour(command_embed.colour.clone().unwrap());
                embed.footer(|f| {
                    f.text(command_embed.footer.clone().unwrap().text.clone())
                        .icon_url(command_embed.footer.clone().unwrap().icon_url.unwrap())
                });

                embed
            }
            _ => CreateEmbed::default(),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            CommandResponse::String(string) => string.clone(),
            CommandResponse::Embed(embed) => embed.description.clone().unwrap(),
            _ => "".to_string(),
        }
    }
}

impl PartialEq for CommandResponse<'_> {
    fn eq(&self, other: &Self) -> bool {
        match self {
            CommandResponse::String(string) => match other {
                CommandResponse::String(other_string) => string == other_string,
                _ => false,
            },
            CommandResponse::Embed(embed) => match other {
                CommandResponse::Embed(other_embed) => {
                    Some(embed.title.clone()) == Some(other_embed.title.clone())
                }
                _ => false,
            },
            _ => match other {
                CommandResponse::None => true,
                _ => false,
            },
        }
    }
    fn ne(&self, other: &Self) -> bool {
        match self {
            CommandResponse::String(string) => match other {
                CommandResponse::String(other_string) => string != other_string,
                _ => true,
            },
            CommandResponse::Embed(embed) => match other {
                CommandResponse::Embed(other_embed) => {
                    Some(embed.title.clone()) != Some(other_embed.title.clone())
                }
                _ => true,
            },
            _ => match other {
                CommandResponse::None => false,
                _ => true,
            },
        }
    }
}

impl std::fmt::Display for CommandResponse<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandResponse::String(string) => write!(f, "{}", string),
            CommandResponse::Embed(embed) => write!(f, "{}", embed.description.clone().unwrap()),
            CommandResponse::Message(_) => write!(f, "Message"),
            _ => write!(f, "None"),
        }
    }
}

// command result must be a string or an embed
pub type InternalCommandResult<'a> = CommandResult<CommandResponse<'a>>;

#[async_trait]
pub trait RunnerFn {
    async fn run<'a>(
        &self,
        arguments: &Vec<Box<dyn Any + Send + Sync>>,
    ) -> InternalCommandResult<'a>;
}

pub fn collect_commands() -> Vec<Command> {
    vec![
        self::ping::get_command(),
        self::poll::get_command(),
        self::language::get_command(),
        self::jingle::get_command(),
        self::radio::get_command(),
        self::voice::join::get_command(),
        self::voice::leave::get_command(),
        self::voice::mute::get_command(),
    ]
}
