use serenity::{
    async_trait,
    model::{
        application::{
            component::ActionRowComponent, interaction::modal::ModalSubmitInteractionData,
        },
        guild::Guild,
    },
};

use crate::{
    commands::poll::{Poll, PollOption},
    interactions::{Interaction, InteractionType, RunnerFn},
    internal::{
        arguments::ArgumentsLevel,
        debug::{log_message, MessageTypes},
    },
};

struct PollOptionModalReceiver {}

#[async_trait]
impl RunnerFn for PollOptionModalReceiver {
    async fn run(&self, args: &Vec<Box<dyn std::any::Any + Send + Sync>>) -> () {
        let guild_id = args
            .iter()
            .filter_map(|arg| arg.downcast_ref::<Guild>())
            .collect::<Vec<&Guild>>()[0]
            .id;
        let submit_data = args
            .iter()
            .filter_map(|arg| arg.downcast_ref::<ModalSubmitInteractionData>())
            .collect::<Vec<&ModalSubmitInteractionData>>()[0];

        // Step 1: Recover poll data from database
        let poll_id = submit_data.custom_id.split("/").collect::<Vec<&str>>()[1];
        let mut poll = Poll::from_id(poll_id.to_string());

        log_message(
            format!("Received interaction with custom_id: {}", poll_id).as_str(),
            MessageTypes::Info,
        );

        // Step 2: Get new option to add to poll
        let name =
            submit_data.components[0]
                .components
                .iter()
                .find_map(|component| match component {
                    ActionRowComponent::InputText(input) => {
                        if input.custom_id == "option_name" {
                            Some(input.value.clone())
                        } else {
                            None
                        }
                    }
                    _ => None,
                });

        let description = submit_data.components[1]
            .components
            .iter()
            .find_map(|component| match component {
                ActionRowComponent::InputText(input) => {
                    if input.custom_id == "option_description" {
                        Some(input.value.clone())
                    } else {
                        None
                    }
                }
                _ => None,
            });

        log_message(
            format!("Name: {:?}, Description: {:?}", name, description).as_str(),
            MessageTypes::Debug,
        );

        // Step 3: Add new option to poll
        match poll.options {
            Some(ref mut options) => {
                options.push(PollOption {
                    value: name.unwrap(),
                    description,
                    votes: vec![],
                });
            }
            None => {
                poll.options = Some(vec![PollOption {
                    value: name.unwrap(),
                    description,
                    votes: vec![],
                }]);
            }
        }

        // Step 4: Save poll to database
        poll.save(guild_id)

        // Step 5: Update poll message
        // poll.embed.update_message(ctx).await;
    }
}

pub fn get_poll_option_modal_interaction() -> Interaction {
    Interaction::new(
        "option_data_poll",
        "Save a poll option",
        InteractionType::Modal,
        vec![
            ArgumentsLevel::User,
            ArgumentsLevel::ChannelId,
            ArgumentsLevel::Guild,
            ArgumentsLevel::ModalSubmitData,
        ],
        Box::new(PollOptionModalReceiver {}),
    )
}
