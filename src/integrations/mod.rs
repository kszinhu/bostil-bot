use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::id::UserId;
use serenity::prelude::Context;

pub mod jukera;

#[async_trait]
pub trait CallbackFn {
    async fn run(&self, msg: &Message, a: &Context, c: &UserId) -> ();
}

pub enum IntegrationType {
    Chat,
}

pub struct Integration {
    pub name: String,
    pub description: String,
    pub integration_type: IntegrationType,
    pub callback: Box<dyn CallbackFn + Send + Sync>,
}

impl Integration {
    pub fn new(
        name: String,
        description: String,
        integration_type: IntegrationType,
        callback: Box<dyn CallbackFn + Send + Sync>,
    ) -> Integration {
        Integration {
            name,
            description,
            integration_type,
            callback,
        }
    }
}

pub fn get_chat_integrations() -> Vec<Integration> {
    vec![jukera::register()]
}
