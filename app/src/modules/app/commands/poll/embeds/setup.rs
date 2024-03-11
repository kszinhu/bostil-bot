use bostil_core::{
    arguments::{ApplicationEmbedFnArguments, ArgumentsLevel},
    embeds::{ApplicationEmbed, EmbedLifetime},
};
use diesel::{BoolExpressionMethods, ExpressionMethods};
use lazy_static::lazy_static;
use rust_i18n::t;
use serenity::{all::MessageId, builder::CreateEmbed};
use std::sync::Mutex;
use tracing::debug;
use uuid::Uuid;

use crate::{
    modules::{
        app::commands::poll::PollStage,
        core::{
            entities::{poll::Poll, MessageIdWrapper},
            helpers::establish_connection,
        },
    },
    schema::polls,
};

#[derive(Debug, Clone)]
/// Embed to show the poll configuration and status during the voting stage
struct PollSetupEmbed;

impl EmbedLifetime for PollSetupEmbed {
    fn build(&self, arguments: ApplicationEmbedFnArguments) -> CreateEmbed {
        use crate::diesel::{QueryDsl, RunQueryDsl, SelectableHelper};

        let poll_id = arguments
            .get(&ArgumentsLevel::PollId)
            .unwrap()
            .downcast_ref::<Uuid>()
            .unwrap();
        let stage = arguments
            .get(&ArgumentsLevel::PollStage)
            .unwrap()
            .downcast_ref::<PollStage>()
            .unwrap();

        let connection = &mut establish_connection();
        let poll = polls::table
            .find(poll_id)
            .select(Poll::as_select())
            .first::<Poll>(connection)
            .expect("Error loading poll");

        let embed = CreateEmbed::default().color(stage.embed_color());

        match stage {
            PollStage::Closed => embed
                .title(t!("commands.poll.setup.embed.stages.closed.title"))
                .description(t!("commands.poll.setup.stages.closed.description")),
            PollStage::Voting => embed
                .title(t!("commands.poll.setup.embed.stages.voting.title"))
                .description(t!("commands.poll.setup.stages.voting.description")),
            PollStage::Setup => embed
                .title(t!("commands.poll.setup.embed.stages.setup.title"))
                .description(t!("commands.poll.setup.embed.stages.setup.description"))
                .field("ID", poll.id.to_string(), true)
                .field("User", format!("<@{}>", poll.created_by), true)
                .field("\u{200B}", "\u{200B}", false), // Separator
        }
    }

    fn after_sent(&self, arguments: ApplicationEmbedFnArguments) {
        use crate::diesel::{QueryDsl, RunQueryDsl};

        let poll_id = arguments
            .get(&ArgumentsLevel::PollId)
            .unwrap()
            .downcast_ref::<Uuid>()
            .unwrap();
        let embed_message_id = arguments
            .get(&ArgumentsLevel::Message)
            .unwrap()
            .downcast_ref::<MessageId>()
            .unwrap();

        debug!(
            "[{:?}] updating poll with embed message id -> {:?}",
            poll_id, embed_message_id
        );

        let connection = &mut establish_connection();

        diesel::update(
            polls::table.filter(polls::id.eq(poll_id).and(polls::embed_message_id.is_null())),
        )
        .set(polls::embed_message_id.eq(MessageIdWrapper(*embed_message_id)))
        .execute(connection)
        .expect("Error updating poll");
    }
}

lazy_static! {
    pub static ref SETUP_EMBED: Mutex<ApplicationEmbed> = Mutex::new(ApplicationEmbed::new(
        "Poll Setup",
        Some("Embed to configure poll"),
        Some("Estamos configurando a enquete abaixo:"),
        vec![ArgumentsLevel::PollId, ArgumentsLevel::PollStage],
        Box::new(PollSetupEmbed),
        None,
        None,
        None,
    ));
}
