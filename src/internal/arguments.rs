use std::any::Any;

use serenity::{
    client::Context,
    model::{
        application::interaction::{
            application_command::CommandDataOption, modal::ModalSubmitInteractionData,
        },
        guild::Guild,
        id::{ChannelId, InteractionId},
        user::User,
    },
};

/**
 Arguments to provide to a run function
 - None: No arguments
   - Value: 0
 - Options: options (&command.data.options)
   - Value: 1
 - Context: context (&context)
   - Value: 2
 - Guild: guild (&guild)
   - Value: 3
 - User: user (&user)
   - Value: 4
 - InteractionId: interaction_id (&interaction_id)
   - Value: 5
 - ChannelId: channel_id (&channel_id)
   - Value: 6
 - ModalSubmitInteractionData: modal_submit_data (&modal_submit_data)
   - Value: 7
*/
#[derive(Debug, Clone, Copy)]
pub enum ArgumentsLevel {
    None,
    Options,
    Context,
    Guild,
    User,
    InteractionId,
    ChannelId,
    ModalSubmitData,
}

impl ArgumentsLevel {
    pub fn value(&self) -> u8 {
        match self {
            ArgumentsLevel::None => 0,
            ArgumentsLevel::Options => 1,
            ArgumentsLevel::Context => 2,
            ArgumentsLevel::Guild => 3,
            ArgumentsLevel::User => 4,
            ArgumentsLevel::InteractionId => 5,
            ArgumentsLevel::ChannelId => 6,
            ArgumentsLevel::ModalSubmitData => 7,
        }
    }

    // function to provide the arguments to the run function
    pub fn provide(
        requested_arguments: &Vec<ArgumentsLevel>,
        context: &Context,
        guild: &Guild,
        user: &User,
        channel_id: &ChannelId,
        options: Option<Vec<CommandDataOption>>,
        interaction_id: Option<InteractionId>,
        modal_submit_data: Option<&ModalSubmitInteractionData>,
    ) -> Vec<Box<dyn Any + Send + Sync>> {
        let mut arguments: Vec<Box<dyn Any + Send + Sync>> = vec![];

        for argument in requested_arguments {
            match argument {
                ArgumentsLevel::None => (),
                ArgumentsLevel::Options => arguments.push(Box::new(options.clone())),
                ArgumentsLevel::Context => arguments.push(Box::new(context.clone())),
                ArgumentsLevel::Guild => arguments.push(Box::new(guild.clone())),
                ArgumentsLevel::User => arguments.push(Box::new(user.clone())),
                ArgumentsLevel::InteractionId => arguments.push(Box::new(interaction_id.clone())),
                ArgumentsLevel::ChannelId => arguments.push(Box::new(channel_id.clone())),
                ArgumentsLevel::ModalSubmitData => {
                    arguments.push(Box::new(modal_submit_data.unwrap().clone()))
                }
            }
        }

        arguments
    }
}
