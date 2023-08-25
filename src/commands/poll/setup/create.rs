use super::embeds;
use crate::{
    commands::{
        poll::{PartialPoll, PollType},
        ArgumentsLevel, Command, CommandCategory, CommandResponse, InternalCommandResult, RunnerFn,
    },
    internal::debug::{log_message, MessageTypes},
};

use rust_i18n::t;
use serenity::{
    async_trait,
    builder::CreateApplicationCommandOption,
    futures::StreamExt,
    model::{
        prelude::{
            application_command::CommandDataOption,
            command::CommandOptionType,
            component::{ButtonStyle, InputTextStyle},
            modal::ModalSubmitInteraction,
            ChannelId, InteractionResponseType,
        },
        user::User,
    },
    prelude::Context,
};
use std::time::Duration;

struct CreatePollRunner;

#[async_trait]
impl RunnerFn for CreatePollRunner {
    async fn run<'a>(
        &self,
        args: &Vec<Box<dyn std::any::Any + Send + Sync>>,
    ) -> InternalCommandResult<'a> {
        let options = args
            .iter()
            .filter_map(|arg| arg.downcast_ref::<Vec<CommandDataOption>>())
            .collect::<Vec<&Vec<CommandDataOption>>>();
        let subcommand_options = &options.get(0).unwrap().get(0).unwrap().options;

        let poll_name = subcommand_options
            .iter()
            .find(|option| option.name == "poll_name")
            .unwrap()
            .value
            .as_ref()
            .unwrap()
            .as_str()
            .unwrap();
        let poll_description = subcommand_options
            .iter()
            .find(|option| option.name == "poll_description")
            .unwrap()
            .value
            .as_ref()
            .unwrap()
            .as_str()
            .unwrap();
        let ctx = args
            .iter()
            .find_map(|arg| arg.downcast_ref::<Context>())
            .unwrap();
        let channel_id = args
            .iter()
            .find_map(|arg| arg.downcast_ref::<ChannelId>())
            .unwrap();
        let user_id = args
            .iter()
            .filter_map(|arg| arg.downcast_ref::<User>())
            .collect::<Vec<&User>>()
            .get(0)
            .unwrap()
            .id;

        // Create thread
        let thread_channel = channel_id
            .create_private_thread(ctx.http.clone(), |thread| thread.name(poll_name))
            .await?;

        thread_channel
            .id
            .add_thread_member(ctx.http.clone(), user_id)
            .await?;

        // Setup poll
        let mut message = thread_channel
            .send_message(&ctx.http, |message| {
                let embed = embeds::setup::embed(
                    poll_name.to_string(),
                    Some(poll_description.to_string()),
                    user_id.clone(),
                )
                .unwrap();

                message.set_embed(embed)
            })
            .await?;

        // Add buttons (kind)
        message
            .edit(&ctx.http, |message| {
                message.components(|components| {
                    components.create_action_row(|action_row| {
                        action_row
                            .create_button(|button| {
                                button
                                    .style(ButtonStyle::Primary)
                                    .label("Single choice")
                                    .custom_id("single_choice")
                            })
                            .create_button(|button| {
                                button
                                    .style(ButtonStyle::Primary)
                                    .label("Multiple choice")
                                    .custom_id("multiple_choice")
                            })
                    })
                })
            })
            .await?;

        let mut interaction_stream = message
            .await_component_interactions(&ctx)
            .timeout(Duration::from_secs(60 * 3))
            .build();

        while let Some(interaction) = interaction_stream.next().await {
            let interaction_id = interaction.data.custom_id.as_str();
            let interaction_user = interaction.user.clone();

            if interaction_user.id != user_id {
                match interaction
                    .create_interaction_response(&ctx.http, |response| {
                        response.kind(InteractionResponseType::DeferredUpdateMessage)
                    })
                    .await
                {
                    Ok(_) => {}
                    Err(_) => {
                        log_message("Failed to defer update message", MessageTypes::Error);
                    }
                }
            }

            match interaction_id {
                "single_choice" => {
                    match interaction
                        .create_interaction_response(&ctx.http, |response| {
                            response.kind(InteractionResponseType::Modal);

                            response.interaction_response_data(|message| {
                                message
                                    .title("Single choice")
                                    .custom_id("option_data")
                                    .components(|components| {
                                        components
                                            .create_action_row(|action_row| {
                                                action_row.create_input_text(|input| {
                                                    input
                                                        .custom_id("option_name")
                                                        .required(true)
                                                        .label("Name of the option")
                                                        .placeholder("Insert a name")
                                                        .style(InputTextStyle::Short)
                                                })
                                            })
                                            .create_action_row(|action_row| {
                                                action_row.create_input_text(|input| {
                                                    input
                                                        .custom_id("option_description")
                                                        .required(true)
                                                        .label("Description of the option")
                                                        .placeholder("Insert a description")
                                                        .style(InputTextStyle::Paragraph)
                                                })
                                            })
                                    })
                            })
                        })
                        .await
                    {
                        Ok(_) => {}
                        Err(why) => {
                            log_message(
                                &format!("Failed to create interaction response: {}", why),
                                MessageTypes::Error,
                            );
                        }
                    }
                }
                "multiple_choice" => {
                    match interaction
                        .create_interaction_response(&ctx.http, |response| {
                            response.kind(InteractionResponseType::Modal);

                            response.interaction_response_data(|message| {
                                message
                                    .title("Single choice")
                                    .custom_id("option_data")
                                    .components(|components| {
                                        components
                                            .create_action_row(|action_row| {
                                                action_row.create_input_text(|input| {
                                                    input
                                                        .custom_id("option_name")
                                                        .required(true)
                                                        .label("Name of the option")
                                                        .placeholder("Insert a name")
                                                        .style(InputTextStyle::Short)
                                                })
                                            })
                                            .create_action_row(|action_row| {
                                                action_row.create_input_text(|input| {
                                                    input
                                                        .custom_id("option_description")
                                                        .required(true)
                                                        .label("Description of the option")
                                                        .placeholder("Insert a description")
                                                        .style(InputTextStyle::Paragraph)
                                                })
                                            })
                                    })
                            })
                        })
                        .await
                    {
                        Ok(_) => {}
                        Err(why) => {
                            log_message(
                                &format!("Failed to create interaction response: {}", why),
                                MessageTypes::Error,
                            );
                        }
                    }
                }

                _ => {
                    log_message(
                        format!("Unknown interaction id: {}", interaction_id).as_str(),
                        MessageTypes::Error,
                    );

                    interaction
                        .create_interaction_response(&ctx.http, |response| {
                            response.kind(InteractionResponseType::DeferredUpdateMessage)
                        })
                        .await?;
                }
            }
        }

        // Create partial poll
        let partial_poll = PartialPoll {
            name: poll_name.to_string(),
            description: Some(poll_description.to_string()),
            created_by: user_id.clone(),
            kind: PollType::SingleChoice,
            thread_id: thread_channel.id,
        };

        Ok(CommandResponse::String(
            t!("commands.poll.setup.response.success", "channel_id" => thread_channel.id.to_string()),
        ))
    }
}

pub async fn handle_modal(ctx: &Context, command: &ModalSubmitInteraction) {
    if let Err(why) = command
        .create_interaction_response(&ctx.http, |m| {
            m.kind(InteractionResponseType::DeferredUpdateMessage)
        })
        .await
    {
        log_message(
            &format!("Failed to create interaction response: {}", why),
            MessageTypes::Error,
        );
    }
}

pub fn register_option<'a>() -> CreateApplicationCommandOption {
    let mut command_option = CreateApplicationCommandOption::default();

    command_option
        .name("setup")
        .name_localized("pt-BR", "configurar")
        .description("Setup a poll")
        .description_localized("pt-BR", "Configura uma votação")
        .kind(CommandOptionType::SubCommand)
        .create_sub_option(|sub_option| {
            sub_option
                .name("poll_name")
                .name_localized("pt-BR", "nome_da_votação")
                .description("The name of the option (max 25 characters)")
                .description_localized("pt-BR", "O nome da opção (máx 25 caracteres)")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_sub_option(|sub_option| {
            sub_option
                .name("poll_description")
                .name_localized("pt-BR", "descrição_da_votação")
                .description("The description of the option (max 100 characters)")
                .description_localized("pt-BR", "A descrição da votação")
                .kind(CommandOptionType::String)
                .required(true)
        });

    command_option
}

pub fn get_command() -> Command {
    Command::new(
        "setup",
        "Setup a poll",
        CommandCategory::Misc,
        vec![
            ArgumentsLevel::Options,
            ArgumentsLevel::Context,
            ArgumentsLevel::Guild,
            ArgumentsLevel::User,
            ArgumentsLevel::ChannelId,
            ArgumentsLevel::InteractionId,
        ],
        Box::new(CreatePollRunner),
    )
}
