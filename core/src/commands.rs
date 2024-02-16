use super::arguments::ArgumentsLevel;
use crate::runners::runners::CommandRunnerFn;

use serenity::builder::CreateCommand;

/// Context of the command that can be used in a guild or global
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum CommandContext {
    Global,
    Guild,
}

/// Category is a type of funcionalities that the command is used (eg.: Fun, Moderation, ...)
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

impl CommandCategory {
    pub fn from(category: &str) -> Self {
        match category {
            "Fun" => Self::Fun,
            "Moderation" => Self::Moderation,
            "Music" => Self::Music,
            "Misc" => Self::Misc,
            "Voice" => Self::Voice,
            "Admin" => Self::Admin,
            "General" => Self::General,
            _ => Self::General,
        }
    }
}

#[derive(Clone)]
/// Struct for Application Command used to executes and register application command
pub struct Command {
    /// Name is the identifier of the command (unique)
    pub name: String,
    /// Description is a short description of the command
    pub description: String,
    /// Category is a type of funcionalities that the command is used (eg.: Fun, Moderation, ...)
    pub category: CommandCategory,
    /// Context is a type of command that can be used in a guild or global
    pub context: CommandContext,
    /// Arguments is a list of arguments that the command uses on Runner
    pub arguments: Vec<ArgumentsLevel>,
    /// Runner is a function that will be executed when the command is called
    pub runner: Box<dyn CommandRunnerFn + Send + Sync>,
    /// Fingerprint is resgiter struct for application command
    pub fingerprint: Option<CreateCommand>,
}

impl Command {
    pub fn new(
        name: &str,
        description: &str,
        context: CommandContext,
        category: CommandCategory,
        arguments: Vec<ArgumentsLevel>,
        runner: Box<dyn CommandRunnerFn + Send + Sync>,
        fingerprint: Option<CreateCommand>,
    ) -> Self {
        let sorted_arguments = {
            let mut sorted_arguments = arguments.clone();
            sorted_arguments.sort_by(|a, b| a.value().cmp(&b.value()));
            sorted_arguments
        };

        Self {
            runner,
            category,
            fingerprint,
            context,
            arguments: sorted_arguments,
            description: description.to_string(),
            name: name.to_string(),
        }
    }

    pub fn to_command(&self) -> Command {
        self.clone()
    }
}
