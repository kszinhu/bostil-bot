use downcast::Any;
use serenity::{
    all::Guild,
    builder::{CreateEmbed, CreateMessage, EditMessage},
    client::Context,
    model::channel::{GuildChannel, Message},
};
use tracing::{error, info};

use crate::arguments::{ArgumentsHashMap, ArgumentsLevel, ArgumentsStruct};

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
    pub arguments: Vec<ArgumentsStruct>,
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
        arguments: Vec<ArgumentsLevel>,
        lifetime: Box<dyn EmbedLifetime + Send + Sync>,
        is_recoverable: Option<bool>,
        database_id: Option<i64>,
        message_id: Option<i64>,
    ) -> Self {
        let sorted_arguments = {
            let mut arguments: Vec<ArgumentsStruct> = arguments
                .iter()
                .map(|level| ArgumentsStruct {
                    level: *level,
                    value: None,
                })
                .collect();

            arguments.sort_by(|a, b| a.level.value().cmp(&b.level.value()));

            arguments
        };

        Self {
            lifetime,
            database_id,
            message_id,
            is_recoverable: match is_recoverable {
                Some(val) => val,
                None => false,
            },
            description: match description {
                Some(desc) => Some(desc.to_string()),
                None => None,
            },
            message: match message {
                Some(msg) => Some(msg.to_string()),
                None => None,
            },
            name: name.to_string(),
            arguments: sorted_arguments,
        }
    }

    pub async fn send_message(
        &mut self,
        ctx: &Context,
        channel: &GuildChannel,
    ) -> Result<Message, ()> {
        let guild = &channel.guild(&ctx.cache).unwrap() as &Guild;

        // Update the arguments with the new values
        self.arguments.iter_mut().for_each(|arg| {
            if arg.level == ArgumentsLevel::Guild {
                arg.value = Some(Box::new(guild.clone()));
            }
        });

        let (requested_arquments, arguments) = self.arguments();

        match channel
            .send_message(
                &ctx.http,
                CreateMessage::default()
                    .content(self.message.clone().unwrap())
                    .embed(
                        self.lifetime
                            .build(&ArgumentsLevel::provide(&requested_arquments, &arguments)),
                    ),
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
        &mut self,
        ctx: &Context,
        mut sent_message: Message,
    ) -> Result<Message, ()> {
        let guild_arg = &sent_message.guild(&ctx.cache).unwrap() as &Guild;

        // Update the arguments with the new values
        self.arguments.iter_mut().for_each(|arg| {
            if arg.level == ArgumentsLevel::Message {
                arg.value = Some(Box::new(sent_message.clone()));
            } else if arg.level == ArgumentsLevel::Guild {
                arg.value = Some(Box::new(guild_arg.clone()));
            }
        });

        let (requested_arquments, arguments) = self.arguments();

        match sent_message
            .edit(
                &ctx.http,
                EditMessage::default().embed(
                    self.lifetime
                        .on_update(&ArgumentsLevel::provide(&requested_arquments, &arguments)),
                ),
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

    fn arguments(&self) -> (Vec<ArgumentsLevel>, ArgumentsHashMap) {
        let mut arguments = ArgumentsHashMap::new();
        let requested_arquments = self
            .arguments
            .iter()
            .map(|arg| arg.level.clone())
            .collect::<Vec<ArgumentsLevel>>();

        for required_argument in requested_arquments.iter() {
            let value = match self
                .arguments
                .iter()
                .find(|arg| arg.level == *required_argument)
            {
                Some(arg) => arg.value.clone().unwrap(),
                None => panic!("Argument {:?} not provided", required_argument),
            };

            arguments.insert(*required_argument, value);
        }

        (requested_arquments, arguments)
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
