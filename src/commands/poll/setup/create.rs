use crate::{
    commands::{
        poll::{utils::progress_bar, PartialPoll, PollDatabaseModel as Poll, PollType},
        ArgumentsLevel, Command, CommandCategory, CommandResponse, InternalCommandResult, RunnerFn,
    },
    components::button::Button,
    internal::debug::{log_message, MessageTypes},
};

use serenity::{
    async_trait,
    builder::{CreateApplicationCommandOption, CreateEmbed, EditInteractionResponse},
    framework::standard::CommandResult,
    model::{
        prelude::{
            application_command::CommandDataOption, command::CommandOptionType,
            component::ButtonStyle, ChannelId,
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

        // Create partial poll
        let partial_poll = PartialPoll {
            name: poll_name.to_string(),
            description: Some(poll_description.to_string()),
            created_by: user_id.clone(),
            kind: PollType::SingleChoice,
            thread_id: thread_channel.id,
        };

        log_message(
            format!("Partial poll: {:?}", partial_poll).as_str(),
            MessageTypes::Debug,
        );

        Ok(CommandResponse::None)
    }
}

// fn create_interaction() {
//         // Wait for multiple interactions
//         let mut interaction_stream =
//         m.await_component_interactions(&ctx).timeout(Duration::from_secs(60 * 3)).build();

//     while let Some(interaction) = interaction_stream.next().await {
//         let sound = &interaction.data.custom_id;
//         // Acknowledge the interaction and send a reply
//         interaction
//             .create_interaction_response(&ctx, |r| {
//                 // This time we dont edit the message but reply to it
//                 r.kind(InteractionResponseType::ChannelMessageWithSource)
//                     .interaction_response_data(|d| {
//                         // Make the message hidden for other users by setting `ephemeral(true)`.
//                         d.ephemeral(true)
//                             .content(format!("The **{}** says __{}__", animal, sound))
//                     })
//             })
//             .await
//             .unwrap();
//     }
//     m.delete(&ctx).await?;
// }

fn vote_interaction() {}

fn create_embed_poll(
    mut message_builder: EditInteractionResponse,
    poll: Poll,
) -> CommandResult<EditInteractionResponse> {
    let time_remaining = match poll.timer.as_secs() / 60 > 1 {
        true => format!("{} minutes", poll.timer.as_secs() / 60),
        false => format!("{} seconds", poll.timer.as_secs()),
    };
    let mut embed = CreateEmbed::default();
    embed
        .title(poll.name)
        .description(poll.description.unwrap_or("".to_string()));

    // first row (id, status, user)
    embed.field(
        "ID",
        format!("`{}`", poll.id.to_string().split_at(8).0),
        true,
    );
    embed.field("Status", poll.status.to_string(), true);
    embed.field("User", format!("<@{}>", poll.created_by), true);

    // separator
    embed.field("\u{200B}", "\u{200B}", false);

    poll.options.iter().for_each(|option| {
        embed.field(option, option, false);
    });

    // separator
    embed.field("\u{200B}", "\u{200B}", false);

    embed.field(
        "Partial Results (Live)",
        format!(
            "```diff\n{}\n```",
            progress_bar(poll.votes, poll.options.clone())
        ),
        false,
    );

    // separator
    embed.field("\u{200B}", "\u{200B}", false);

    embed.field(
        "Time remaining",
        format!("{} remaining", time_remaining),
        false,
    );

    message_builder.set_embed(embed);
    message_builder.components(|component| {
        component.create_action_row(|action_row| {
            poll.options.iter().for_each(|option| {
                action_row
                    .add_button(Button::new(option, option, ButtonStyle::Primary, None).create());
            });

            action_row
        })
    });

    Ok(message_builder)
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
