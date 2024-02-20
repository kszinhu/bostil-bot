use serenity::builder::CreateCommand;

use crate::commands::{Command, CommandContext};

#[derive(Clone)]
pub struct CommandCollector {
    pub commands: Vec<Command>,
    pub length: usize,
}

impl CommandCollector {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
            length: 0,
        }
    }

    /// Store a command in the collector
    pub fn store_command(&mut self, command: Command) {
        self.commands.push(command);
        self.length += 1;
    }

    /// Get the fingerprints of all the commands in the collector
    ///
    /// Args:
    /// - `context` - The context to filter the commands by
    ///
    /// Returns:
    /// - A vector of fingerprints of the commands
    pub fn get_fingerprints(self, context: Option<CommandContext>) -> Vec<CreateCommand> {
        self.commands
            .iter()
            .filter(|command| match context {
                Some(context) => command.context == context,
                None => true,
            })
            .map(|command| command.fingerprint.clone().unwrap())
            .collect::<Vec<CreateCommand>>()
    }
}
