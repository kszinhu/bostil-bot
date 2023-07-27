use serenity::async_trait;
use serenity::client::Context;
use serenity::model::prelude::{ChannelId, UserId};

pub mod chat;
pub mod voice_channel;

pub enum InteractionType {
    Chat,
    VoiceChannel,
}

#[async_trait]
pub trait CallbackFn {
    async fn run(&self, channel: &ChannelId, ctx: &Context, user_id: &UserId) -> ();
}

#[allow(dead_code)]
pub struct Interaction {
    pub name: String,
    pub description: String,
    pub interaction_type: InteractionType,
    pub callback: Box<dyn CallbackFn + Send + Sync>,
}

// get all chat interactions
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
