use std::{os::fd::IntoRawFd, sync::Arc, time::Duration};

use super::embeds::setup::get_embed;
use crate::{
    commands::{
        poll::{PartialPoll, Poll, PollStage, PollStatus, PollType},
        ArgumentsLevel, Command, CommandCategory, CommandResponse, InternalCommandResult, RunnerFn,
    },
    components,
    internal::debug::{log_message, MessageTypes},
};

use rust_i18n::t;
use serenity::{
    async_trait,
    builder::CreateApplicationCommandOption,
    collector::ComponentInteractionCollector,
    futures::StreamExt,
    model::{
        application::interaction::message_component::MessageComponentInteraction,
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

struct CreatePollRunner;

#[async_trait]
impl RunnerFn for CreatePollRunner {
    async fn run<'a>(
        &self,
        args: &Vec<Box<dyn std::any::Any + Send + Sync>>,
    ) -> InternalCommandResult<'a> {
        let options = args
            .iter()
            .filter_map(|arg| arg.downcast_ref::<Option<Vec<CommandDataOption>>>())
            .collect::<Vec<&Option<Vec<CommandDataOption>>>>()[0]
            .as_ref()
            .unwrap();
        let subcommand_options = &options[0].options;

        let poll_name = match subcommand_options
            .iter()
            .find(|option| option.name == "name")
        {
            Some(option) => option.value.as_ref().unwrap().as_str().unwrap(),
            None => {
                panic!("Poll name is required")
            }
        };
        let poll_description = match subcommand_options
            .iter()
            .find(|option| option.name == "description")
        {
            Some(option) => Some(option.value.as_ref().unwrap().to_string()),
            None => None,
        };
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

        // Step 1: Create thread
        let thread_channel = channel_id
            .create_private_thread(ctx.http.clone(), |thread| thread.name(poll_name))
            .await?;

        thread_channel
            .id
            .add_thread_member(ctx.http.clone(), user_id)
            .await?;

        let mut setup_embed = get_embed();

        // Step 2: Create a partial poll and send it to the thread
        setup_embed.arguments = vec![
            Box::new(PartialPoll::new(
                poll_name,
                poll_description,
                None,
                Some(vec![]),
                None,
                Some(PollStatus::NotReady),
                user_id,
            )),
            Box::new(PollStage::Setup),
        ];

        setup_embed.send_message(thread_channel.clone(), ctx).await;

        // Step 3: Add buttons to the message to choose between add options, starting poll and cancel
        setup_embed
            .message
            .clone()
            .unwrap()
            .edit(&ctx.http, |message| {
                message.components(|components| {
                    // Action row for buttons
                    components.create_action_row(|action_row| {
                        action_row
                            .create_button(|button| {
                                button
                                    .custom_id("add_option")
                                    .label("Add option")
                                    .style(ButtonStyle::Secondary)
                            })
                            .create_button(|button| {
                                button
                                    .custom_id("start_poll")
                                    .label("Start poll")
                                    .disabled(true)
                                    .style(ButtonStyle::Primary)
                            })
                            .create_button(|button| {
                                button
                                    .custom_id("cancel")
                                    .label("Cancel")
                                    .style(ButtonStyle::Danger)
                            })
                    });

                    components.create_action_row(|action_row| {
                        // Select menu for poll type
                        action_row.create_select_menu(|select_menu| {
                            select_menu
                                .custom_id("poll_type")
                                .placeholder("Escolha o tipo da votação")
                                .options(|options| {
                                    options
                                        .create_option(|option| {
                                            option
                                                .label("Single choice")
                                                .value(PollType::SingleChoice.to_int().to_string())
                                                .description("Single choice poll")
                                        })
                                        .create_option(|option| {
                                            option
                                                .label("Multiple choice")
                                                .value(
                                                    PollType::MultipleChoice.to_int().to_string(),
                                                )
                                                .description("Multiple choice poll")
                                        })
                                })
                        })
                    })
                })
            })
            .await?;

        // Step 5: Add interaction listener
        let interaction_stream = setup_embed
            .message
            .clone()
            .unwrap()
            .await_component_interactions(&ctx)
            .timeout(Duration::from_secs(60 * 60 * 24)) // 1 Day to configure the poll
            .build();

        interaction_handler(interaction_stream, ctx).await;

        Ok(CommandResponse::String(t!(
            "commands.poll.setup.response.initial",
            "thread_id" => thread_channel.id,
        )))
    }
}

async fn interaction_handler(mut interaction_stream: ComponentInteractionCollector, ctx: &Context) {
    match interaction_stream.next().await {
        Some(interaction) => {
            let interaction_id = interaction.data.custom_id.as_str();

            match interaction_id {
                "add_option" => add_option(interaction, ctx).await,
                _ => {}
            }
        }

        None => {
            log_message("No interaction received in 1 day", MessageTypes::Failed);
        }
    }
}

async fn add_option(interaction: Arc<MessageComponentInteraction>, ctx: &Context) {
    match interaction
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::Modal)
                .interaction_response_data(|message| {
                    message
                        .custom_id(format!("option_data_poll/{}", interaction.id))
                        .title("Adicionar opção")
                        .components(|components| {
                            components
                                .create_action_row(|action_row| {
                                    action_row.create_input_text(|field| {
                                        field
                                            .custom_id(format!("name_option/{}", interaction.id))
                                            .label("Nome da opção")
                                            .placeholder("Digite o nome da opção")
                                            .max_length(25)
                                            .min_length(1)
                                            .required(true)
                                            .style(InputTextStyle::Short)
                                    })
                                })
                                .create_action_row(|action_row| {
                                    action_row.create_input_text(|field| {
                                        field
                                            .custom_id(format!(
                                                "description_option/{}",
                                                interaction.id
                                            ))
                                            .label("Descrição da opção")
                                            .placeholder("Digite a descrição da opção")
                                            .max_length(200)
                                            .min_length(1)
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

// pub async fn handle_modal(ctx: &Context, command: &ModalSubmitInteraction) {
//     if let Err(why) = command
//         .create_interaction_response(&ctx.http, |m| {
//             m.kind(InteractionResponseType::DeferredUpdateMessage)
//         })
//         .await
//     {
//         log_message(
//             &format!("Failed to create interaction response: {}", why),
//             MessageTypes::Error,
//         );
//     }
// }

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
                .name("name")
                .name_localized("pt-BR", "nome")
                .description("The name of the option (max 25 characters)")
                .description_localized("pt-BR", "O nome da opção (máx 25 caracteres)")
                .kind(CommandOptionType::String)
                .max_length(25)
                .required(true)
        })
        .create_sub_option(|sub_option| {
            sub_option
                .name("channel")
                .name_localized("pt-BR", "canal")
                .description("The channel where the poll will be created")
                .description_localized("pt-BR", "O canal onde a votação será realizada")
                .kind(CommandOptionType::Channel)
                .required(true)
        })
        .create_sub_option(|sub_option| {
            sub_option
                .name("description")
                .name_localized("pt-BR", "descrição")
                .description("The description of the option (max 365 characters)")
                .description_localized(
                    "pt-BR",
                    "A descrição dessa opção (máximo de 365 caracteres)",
                )
                .kind(CommandOptionType::String)
                .max_length(365)
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
        Box::new(CreatePollRunner {}),
    )
}
