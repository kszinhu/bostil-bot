use bostil_core::{
    arguments::ArgumentsLevel,
    commands::{Command, CommandCategory, CommandContext},
    runners::runners::{CommandResponse, CommandResult, CommandRunnerFn},
};
use lazy_static::lazy_static;
use once_cell::sync::Lazy;
use rust_i18n::t;
use serenity::{
    all::{
        AutoArchiveDuration, ButtonStyle, ChannelId, ChannelType, CommandDataOption,
        CommandOptionType, ComponentInteraction, InputTextStyle, User,
    },
    async_trait,
    builder::{
        CreateActionRow, CreateButton, CreateCommandOption, CreateInputText,
        CreateInteractionResponse, CreateModal, CreateSelectMenu, CreateSelectMenuKind,
        CreateSelectMenuOption, CreateThread, EditMessage,
    },
    collector::ComponentInteractionCollector,
    futures::StreamExt,
    prelude::Context,
};
use std::{time::Duration, vec};
use tracing::error;

#[derive(Clone)]
struct CreatePollRunner;

#[async_trait]
impl CommandRunnerFn for CreatePollRunner {
    async fn run<'a>(&self, args: &Vec<Box<dyn std::any::Any + Send + Sync>>) -> CommandResult<'a> {
        use super::embeds::embeds::SETUP_EMBED;

        let options = args
            .iter()
            .filter_map(|arg| arg.downcast_ref::<Option<Vec<CommandDataOption>>>())
            .collect::<Vec<&Option<Vec<CommandDataOption>>>>()[0]
            .as_ref()
            .unwrap();

        let poll_name = match options.iter().find(|option| option.name == "name") {
            Some(option) => option.value.as_str().unwrap(),
            None => {
                panic!("Poll name is required")
            }
        };
        let poll_description = match options.iter().find(|option| option.name == "description") {
            Some(option) => Some(option.value.as_str().unwrap().to_string()),
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
            .create_thread(
                &ctx.http,
                CreateThread::new(poll_name)
                    .kind(ChannelType::PrivateThread)
                    .invitable(true)
                    .auto_archive_duration(AutoArchiveDuration::OneDay),
            )
            .await?;

        thread_channel
            .id
            .add_thread_member(&ctx.http, user_id)
            .await?;

        // Step 2: Create a partial poll and send it to the thread
        let mut embed_message = match SETUP_EMBED.send_message(&ctx, &thread_channel).await {
            Ok(message) => message,
            Err(_) => {
                error!("Failed to send message to thread {}", thread_channel.id);

                return Ok(CommandResponse::String(
                    t!("commands.poll.setup.response.error", "thread_id" => thread_channel.id)
                        .to_string(),
                ));
            }
        };

        // Step 3: Add buttons to the message to choose between add options, starting poll and cancel
        embed_message
            .edit(
                &ctx.http,
                EditMessage::default().components(vec![
                    CreateActionRow::Buttons(vec![
                        CreateButton::new("add_option")
                            .style(ButtonStyle::Secondary)
                            .label("Adicionar opção"),
                        CreateButton::new("start_poll")
                            .style(ButtonStyle::Primary)
                            .label("Iniciar votação"),
                        CreateButton::new("cancel_poll")
                            .style(ButtonStyle::Danger)
                            .label("Cancelar votação"),
                    ]),
                    CreateActionRow::SelectMenu(
                        CreateSelectMenu::new(
                            "poll_kind",
                            CreateSelectMenuKind::String {
                                options: vec![
                                    CreateSelectMenuOption::new("Escolha única", "single_choice")
                                        .description("Cada usuário pode votar em apenas uma opção"),
                                    CreateSelectMenuOption::new(
                                        "Múltipla escolha",
                                        "multiple_choice",
                                    )
                                    .description("Cada usuário pode votar em mais de uma opção"),
                                ],
                            },
                        )
                        .placeholder("Escolha o tipo de votação")
                        .min_values(1)
                        .max_values(1),
                    ),
                ]),
            )
            .await?;

        // Step 5: Add interaction listener
        let interaction_stream = embed_message
            .await_component_interactions(&ctx)
            .timeout(Duration::from_secs(60 * 60 * 24)); // 1 Day to configure the poll

        interaction_handler(interaction_stream, ctx).await;

        Ok(CommandResponse::String(
            t!(
                "commands.poll.setup.response.initial",
                "thread_id" => thread_channel.id,
            )
            .to_string(),
        ))
    }
}

async fn interaction_handler(interaction_stream: ComponentInteractionCollector, ctx: &Context) {
    match interaction_stream.stream().next().await {
        Some(interaction) => {
            let interaction_id = interaction.data.custom_id.as_str();

            match interaction_id {
                "add_option" => add_option(interaction, ctx).await,
                _ => {}
            }
        }

        None => {
            error!("No interaction received in 1 day");
        }
    }
}

async fn add_option(interaction: ComponentInteraction, ctx: &Context) {
    match interaction
        .create_response(
            &ctx.http,
            CreateInteractionResponse::Modal(
                CreateModal::new(
                    format!("option_data_poll/{}", interaction.id),
                    "Adicionar opção",
                )
                .components(vec![
                    CreateActionRow::InputText(
                        CreateInputText::new(InputTextStyle::Short, "Nome da Opção", "name_option")
                            .placeholder("Digite o nome da opção")
                            .max_length(25)
                            .min_length(1)
                            .required(true),
                    ),
                    CreateActionRow::InputText(
                        CreateInputText::new(
                            InputTextStyle::Paragraph,
                            "Descrição da Opção",
                            "description_option",
                        )
                        .placeholder("Digite a descrição da opção")
                        .max_length(500)
                        .min_length(1),
                    ),
                ]),
            ),
        )
        .await
    {
        Ok(_) => {}
        Err(why) => {
            error!("Failed to create interaction response: {}", why);
        }
    }
}

pub static SETUP_OPTION: Lazy<CreateCommandOption> = Lazy::new(|| {
    CreateCommandOption::new(CommandOptionType::SubCommand, "setup", "Setup a poll")
        .name_localized("pt-BR", "configurar")
        .description_localized("pt-BR", "Configura uma votação")
        .add_sub_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "name",
                "The name of the option (max 25 characters)",
            )
            .name_localized("pt-BR", "nome")
            .description_localized("pt-BR", "O nome da opção (máx 25 caracteres)")
            .max_length(25)
            .required(true),
        )
        .add_sub_option(
            CreateCommandOption::new(
                CommandOptionType::Channel,
                "channel",
                "The channel where the poll will be created",
            )
            .name_localized("pt-BR", "canal")
            .description_localized("pt-BR", "O canal onde a votação será realizada")
            .required(true),
        )
        .add_sub_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "description",
                "The description of the option (max 365 characters)",
            )
            .name_localized("pt-BR", "descrição")
            .description_localized(
                "pt-BR",
                "A descrição dessa opção (máximo de 365 caracteres)",
            )
            .max_length(365),
        )
});

lazy_static! {
    pub static ref SETUP_COMMAND: Command = Command::new(
        "setup",
        "Setup a poll",
        CommandContext::Guild,
        CommandCategory::Misc,
        vec![
            ArgumentsLevel::Options,
            ArgumentsLevel::Context,
            ArgumentsLevel::Guild,
            ArgumentsLevel::User,
            ArgumentsLevel::ChannelId,
        ],
        Box::new(CreatePollRunner),
        None,
    );
}
