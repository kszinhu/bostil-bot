use serenity::async_trait;

use crate::internal::arguments::ArgumentsLevel;

pub mod chat;
pub mod modal;
pub mod voice_channel;

pub enum InteractionType {
    Chat,
    Modal,
    VoiceChannel,
}

#[async_trait]
pub trait RunnerFn {
    async fn run(&self, arguments: &Vec<Box<dyn std::any::Any + Send + Sync>>) -> ();
}

pub struct Interaction {
    pub name: String,
    pub description: String,
    pub interaction_type: InteractionType,
    pub arguments: Vec<ArgumentsLevel>,
    pub runner: Box<dyn RunnerFn + Send + Sync>,
}

impl Interaction {
    pub fn new(
        name: &str,
        description: &str,
        interaction_type: InteractionType,
        arguments: Vec<ArgumentsLevel>,
        runner: Box<dyn RunnerFn + Send + Sync>,
    ) -> Self {
        let sorted_arguments = {
            let mut sorted_arguments = arguments.clone();
            sorted_arguments.sort_by(|a, b| a.value().cmp(&b.value()));
            sorted_arguments
        };

        Self {
            runner,
            interaction_type,
            arguments: sorted_arguments,
            name: name.to_string(),
            description: description.to_string(),
        }
    }
}

pub fn get_chat_interactions() -> Vec<Interaction> {
    vec![chat::love::get_love_interaction()]
}

pub fn get_voice_channel_interactions() -> Vec<Interaction> {
    vec![]
}

pub fn get_modal_interactions() -> Vec<Interaction> {
    vec![modal::poll_option::get_poll_option_modal_interaction()]
}

pub async fn get_interactions() -> Vec<Interaction> {
    let mut interactions = get_chat_interactions();
    interactions.append(&mut get_voice_channel_interactions());

    interactions
}
