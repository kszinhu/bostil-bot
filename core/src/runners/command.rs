use dyn_clone::DynClone;
use serenity::{
    all::Embed,
    async_trait,
    builder::{CreateEmbed, EditInteractionResponse},
    framework::standard::CommandResult as SerenityCommandResult,
};
use std::any::Any;

/// CommandResponse is a type of response that the command can return
#[derive(Debug, Clone)]
pub enum CommandResponse {
    String(String),
    Embed(Embed),
    Message(EditInteractionResponse),
    None,
}

/// CommandResult is a type of result (ok or error) that the command can return
pub type CommandResult<'a> = SerenityCommandResult<CommandResponse>;

/// Function that will be executed when the command is called
#[async_trait]
pub trait CommandRunnerFn: DynClone {
    async fn run<'a>(&self, arguments: &Vec<Box<dyn Any + Send + Sync>>) -> CommandResult<'a>;
}

dyn_clone::clone_trait_object!(CommandRunnerFn);

impl std::fmt::Debug for dyn CommandRunnerFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<RunnerFn>")
    }
}

impl CommandResponse {
    pub fn to_embed(&self) -> CreateEmbed {
        match self {
            CommandResponse::String(string) => CreateEmbed::default().description(string.clone()),
            CommandResponse::Embed(command_embed) => CreateEmbed::from(command_embed.clone()),
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

impl PartialEq for CommandResponse {
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

impl std::fmt::Display for CommandResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandResponse::String(string) => write!(f, "{}", string),
            CommandResponse::Embed(embed) => write!(f, "{}", embed.description.clone().unwrap()),
            CommandResponse::Message(_) => write!(f, "Message"),
            _ => write!(f, "None"),
        }
    }
}
