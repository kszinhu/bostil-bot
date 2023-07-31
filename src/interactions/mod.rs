use serenity::async_trait;
use serenity::client::Context;
use serenity::model::prelude::{ChannelId, UserId};

use crate::internal::debug::{log_message, STATUS_INFO};

pub mod chat;
pub mod voice_channel;

pub fn interaction_callback(
    name: &str,
    callback: Box<dyn CallbackFn + Send + Sync>,
) -> Box<dyn CallbackFn + Send + Sync> {
    log_message(&format!("Running integration {}", name), &STATUS_INFO);

    callback
}

pub enum InteractionType {
    Chat,
    VoiceChannel,
}

#[async_trait]
pub trait CallbackFn {
    async fn run(&self, channel: &ChannelId, ctx: &Context, user_id: &UserId) -> ();
}

pub struct Interaction {
    pub name: String,
    pub description: String,
    pub interaction_type: InteractionType,
    pub callback: Box<dyn CallbackFn + Send + Sync>,
}

impl Interaction {
    pub fn new(
        name: &str,
        description: &str,
        interaction_type: InteractionType,
        callback: Box<dyn CallbackFn + Send + Sync>,
    ) -> Interaction {
        Interaction {
            name: name.to_string(),
            description: description.to_string(),
            interaction_type,
            callback: interaction_callback(name, callback),
        }
    }
}

pub fn get_chat_interactions() -> Vec<Interaction> {
    vec![chat::love::get_love_interaction()]
}

pub fn get_voice_channel_interactions() -> Vec<Interaction> {
    vec![]
}

pub async fn get_interactions() -> Vec<Interaction> {
    let mut interactions = get_chat_interactions();
    interactions.append(&mut get_voice_channel_interactions());

    interactions
}
