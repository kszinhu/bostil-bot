use serenity::{
    builder::CreateEmbed, client::Context, model::channel::GuildChannel, model::channel::Message,
};
use std::any::Any;

use super::debug::{log_message, MessageTypes};

pub trait EmbedRunnerFn {
    fn run(&self, arguments: &Vec<Box<dyn Any + Send + Sync>>) -> CreateEmbed;
}

pub struct ApplicationEmbed {
    pub name: String,
    pub description: Option<String>,
    pub message_content: Option<String>,
    pub arguments: Vec<Box<dyn Any + Send + Sync>>,
    pub builder: Box<dyn EmbedRunnerFn + Send + Sync>,
    pub message: Option<Message>,
}

impl ApplicationEmbed {
    pub fn new(
        name: String,
        description: Option<String>,
        message_content: Option<String>,
        arguments: Vec<Box<dyn Any + Send + Sync>>,
        builder: Box<dyn EmbedRunnerFn + Send + Sync>,
    ) -> Self {
        Self {
            name,
            builder,
            arguments,
            description,
            message_content,
            message: None,
        }
    }

    pub async fn update(&self, channel: GuildChannel, ctx: &Context) {
        if let Err(why) = channel
            .edit_message(ctx, self.message.clone().unwrap().id, |m| {
                m.embed(|e| {
                    e.clone_from(&self.builder.run(&self.arguments));
                    e
                })
            })
            .await
        {
            log_message(
                format!("Error updating message: {:?}", why).as_str(),
                MessageTypes::Error,
            );
        }
    }

    pub async fn send_message(&mut self, channel: GuildChannel, ctx: &Context) {
        match channel
            .send_message(ctx, |m| {
                m.embed(|e| {
                    e.clone_from(&self.builder.run(&self.arguments));
                    e
                })
            })
            .await
        {
            Ok(message) => {
                self.message = Some(message);
            }
            Err(why) => log_message(
                format!("Error sending message: {:?}", why).as_str(),
                MessageTypes::Error,
            ),
        }
    }
}

impl std::fmt::Display for ApplicationEmbed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Embed: {} \n {}",
            self.name,
            self.description.clone().unwrap()
        )
    }
}
