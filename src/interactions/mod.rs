use serenity::client::Context;
use serenity::model::prelude::{ChannelId, UserId};

pub mod chat;
pub mod voice_channel;

pub enum InteractionType {
    Chat,
    VoiceChannel,
}

#[allow(dead_code)]
pub type InteractionCallback = fn(&ChannelId, &Context, &UserId) -> Option<()>;

#[allow(dead_code)]
pub struct Interaction {
    pub name: String,
    pub description: String,
    pub interaction_type: InteractionType,
    pub callback: Box<dyn std::any::Any>,
}

// get all chat interactions
pub fn get_chat_interactions() -> Vec<Box<Interaction>> {
    vec![Box::new(chat::love::get_love_interaction())]
}

pub fn get_voice_channel_interactions() -> Vec<Box<Interaction>> {
    vec![]
}

pub fn get_interactions() -> Vec<Box<Interaction>> {
    let mut interactions = get_chat_interactions();
    interactions.append(&mut get_voice_channel_interactions());

    interactions
}
