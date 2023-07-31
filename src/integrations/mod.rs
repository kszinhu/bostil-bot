use crate::internal::debug::{log_message, STATUS_INFO};

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::id::UserId;
use serenity::prelude::Context;

pub mod jukera;

pub fn integration_callback(
    name: &str,
    callback: Box<dyn CallbackFn + Send + Sync>,
) -> Box<dyn CallbackFn + Send + Sync> {
    log_message(&format!("Running integration {}", name), &STATUS_INFO);

    callback
}

#[async_trait]
pub trait CallbackFn {
    async fn run(&self, msg: &Message, a: &Context, c: &UserId) -> ();
}

#[derive(Clone, Copy)]
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
        name: &str,
        description: &str,
        integration_type: IntegrationType,
        callback: Box<dyn CallbackFn + Send + Sync>,
    ) -> Integration {
        Integration {
            name: name.to_string(),
            description: description.to_string(),
            integration_type,
            callback: integration_callback(name, callback),
        }
    }
}

pub fn get_chat_integrations() -> Vec<Integration> {
    vec![jukera::register()]
}
