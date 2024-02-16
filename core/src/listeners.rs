use crate::{arguments::ArgumentsLevel, runners::runners::ListenerRunnerFn};

/// ListenerKind is an enum that represents the different types of listeners that can be used in the bot.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum ListenerKind {
    /// Message is a listener that listens to chat messages
    Message,
    /// Reaction is a listener that listens to reactions
    Reaction,
    /// VoiceState is a listener that listens to voice state updates
    VoiceState,
    /// Modal is a listener that listens to modal submissions
    Modal,
}

#[derive(Clone)]
pub struct Listener {
    pub name: String,
    pub description: String,
    pub kind: ListenerKind,
    pub arguments: Vec<ArgumentsLevel>,
    pub runner: Box<dyn ListenerRunnerFn + Send + Sync>,
}

impl Listener {
    pub fn new(
        name: &str,
        description: &str,
        kind: ListenerKind,
        arguments: Vec<ArgumentsLevel>,
        runner: Box<dyn ListenerRunnerFn + Send + Sync>,
    ) -> Self {
        Self {
            kind,
            arguments,
            runner,
            name: name.to_string(),
            description: description.to_string(),
        }
    }

    pub fn to_listener(&self) -> Self {
        self.clone()
    }
}
