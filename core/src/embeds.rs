use serenity::{
    builder::{CreateEmbed, CreateMessage, EditMessage},
    client::Context,
    model::channel::GuildChannel,
    model::channel::Message,
};
use std::any::Any;
use tracing::{error, info};

pub trait EmbedLifetime {
    /// Function to create the embed (BUILDER)
    fn build(&self, arguments: &Vec<Box<dyn Any + Send + Sync>>) -> CreateEmbed;
    /// Function to run when the embed is being updated
    fn on_update(&self, arguments: &Vec<Box<dyn Any + Send + Sync>>) -> CreateEmbed {
        self.build(arguments)
    }
    /// Function to run when the embed is being sent (after build)
    fn after_sent(&self, _arguments: &Vec<Box<dyn Any + Send + Sync>>) {}
    /// Function to check if the embed should be updated
    fn should_update(&self, _arguments: &Vec<Box<dyn Any + Send + Sync>>) -> bool {
        false
    }
    /// Function to check if the embed should be removed
    fn should_delete(&self, _arguments: &Vec<Box<dyn Any + Send + Sync>>) -> bool {
        false
    }
}

pub struct ApplicationEmbed {
    /// The name of the embed
    pub name: String,
    /// The description of the embed
    pub description: Option<String>,
    /// The content of the message that will be sent
    pub message: Option<String>,
    pub arguments: Vec<Box<dyn Any + Send + Sync>>,
    /// The lifetime of the embed
    pub lifetime: Box<dyn EmbedLifetime + Send + Sync>,
    /// The embed was saved to the database and can be recovered
    pub is_recoverable: bool,
    /// The identifier of the embed on the database
    pub database_id: Option<i64>,
    /// The message id related of sent message
    pub message_id: Option<i64>,
}

impl ApplicationEmbed {
    pub fn new(
        name: &str,
        description: Option<&str>,
        message: Option<&str>,
        arguments: Vec<Box<dyn Any + Send + Sync>>,
        lifetime: Box<dyn EmbedLifetime + Send + Sync>,
        is_recoverable: Option<bool>,
        database_id: Option<i64>,
        message_id: Option<i64>,
    ) -> Self {
        Self {
            lifetime,
            arguments,
            database_id,
            message_id,
            is_recoverable: match is_recoverable {
                Some(val) => val,
                None => false,
            },
            name: name.to_string(),
            description: match description {
                Some(desc) => Some(desc.to_string()),
                None => None,
            },
            message: match message {
                Some(msg) => Some(msg.to_string()),
                None => None,
            },
        }
    }

    pub async fn send_message(&self, ctx: &Context, channel: &GuildChannel) -> Result<Message, ()> {
        match channel
            .send_message(
                &ctx.http,
                CreateMessage::default()
                    .content(self.message.clone().unwrap())
                    .embed(self.lifetime.build(&self.arguments)),
            )
            .await
        {
            Ok(sent_message) => {
                info!("Embed {} sent", self.name);

                Ok(sent_message)
            }

            Err(_) => {
                error!("Embed {} not sent", self.name);

                Err(())
            }
        }
    }

    pub async fn update_message(
        &self,
        ctx: &Context,
        mut sent_message: Message,
    ) -> Result<Message, ()> {
        match sent_message
            .edit(
                &ctx.http,
                EditMessage::default().embed(self.lifetime.on_update(&self.arguments)),
            )
            .await
        {
            Ok(_) => {
                info!("Embed {} updated", self.name);

                Ok(sent_message.clone())
            }

            Err(_) => {
                info!("Embed {} not updated", self.name);

                Err(())
            }
        }
    }

    pub async fn delete_message(&self, ctx: &Context, sent_message: Message) -> Result<(), ()> {
        sent_message.delete(&ctx.http).await.map_err(|_| {
            error!("Embed failed to delete");
        })
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
