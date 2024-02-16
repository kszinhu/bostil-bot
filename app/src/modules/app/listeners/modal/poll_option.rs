use bostil_core::{
    arguments::ArgumentsLevel,
    listeners::{Listener, ListenerKind},
    runners::runners::ListenerRunnerFn,
};
use lazy_static::lazy_static;
use serenity::{
    all::{ActionRowComponent, Guild, ModalInteractionData, UserId},
    async_trait,
    client::Context,
};
use std::any::Any;
use tracing::{debug, error};
use uuid::Uuid;

use crate::modules::core::{
    entities::{
        exports::{Poll, PollChoice, PollVote},
        poll::PollWithChoicesAndVotes,
    },
    helpers::establish_connection,
};

#[derive(Clone)]
struct PollOptionModalReceiver;

#[async_trait]
impl ListenerRunnerFn for PollOptionModalReceiver {
    async fn run<'a>(&self, args: &Vec<Box<dyn Any + Send + Sync>>) -> () {
        use crate::schema::{poll_choices, poll_votes, polls};
        use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper};

        let ctx = args
            .iter()
            .filter_map(|arg| arg.downcast_ref::<Context>())
            .collect::<Vec<&Context>>()[0];
        let user_id = args
            .iter()
            .filter_map(|arg| arg.downcast_ref::<UserId>())
            .collect::<Vec<&UserId>>()[0];
        let guild_id = args
            .iter()
            .filter_map(|arg| arg.downcast_ref::<Guild>())
            .collect::<Vec<&Guild>>()[0]
            .id;
        let submit_data = args
            .iter()
            .filter_map(|arg| arg.downcast_ref::<ModalInteractionData>())
            .collect::<Vec<&ModalInteractionData>>()[0];
        let poll_id = submit_data.custom_id.split("/").collect::<Vec<&str>>()[1]
            .parse::<Uuid>()
            .unwrap();

        // Step 1: Recover poll data from database (join with poll_choices, and poll_votes)
        let connection = &mut establish_connection();
        let polls: Vec<(Poll, PollChoice, PollVote)> = polls::dsl::polls
            .filter(polls::dsl::id.eq(poll_id))
            .inner_join(
                poll_choices::dsl::poll_choices.on(polls::dsl::id.eq(poll_choices::dsl::poll_id)),
            )
            .inner_join(poll_votes::dsl::poll_votes.on(polls::dsl::id.eq(poll_votes::dsl::poll_id)))
            .select((
                Poll::as_select(),
                PollChoice::as_select(),
                PollVote::as_select(),
            ))
            .load::<(Poll, PollChoice, PollVote)>(connection)
            .expect("Error getting poll data");

        let poll = PollWithChoicesAndVotes::from(polls);

        println!("Poll test: {poll:?}");

        // Step 2: Get new option to add to poll
        let name = submit_data.components[0]
            .components
            .iter()
            .find_map(|component| match component {
                ActionRowComponent::InputText(input) => match input.custom_id == "option_name" {
                    true => input.value.clone(),
                    false => None,
                },
                _ => None,
            })
            .expect("Error getting option name");

        let description = submit_data.components[1]
            .components
            .iter()
            .find_map(|component| match component {
                ActionRowComponent::InputText(input) => {
                    match input.custom_id == "option_description" {
                        true => input.value.clone(),
                        false => None,
                    }
                }
                _ => None,
            });

        // value is a name instead of spaces replaced by underscores
        let value = name.clone().replace(" ", "_");

        debug!("Name: {:?}, Description: {:?}", name, description);

        // Step 3: Add new option to poll
        diesel::insert_into(poll_choices::table)
            .values((
                poll_choices::dsl::poll_id.eq(poll.id),
                poll_choices::dsl::value.eq(value),
                poll_choices::dsl::label.eq(name),
                poll_choices::dsl::description.eq(description),
            ))
            .execute(connection)
            .expect("Error inserting new option");

        // Step 4: Update poll message
        match ctx
            .http
            .get_message(poll.thread_id.0, poll.embed_message_id.0)
            .await
        {
            Ok(mut message) => {
                // TODO: Get EmbedModel and use update
            }

            Err(why) => {
                error!("Error getting poll message: {:?}", why);
            }
        }
    }
}

lazy_static! {
    pub static ref POLL_OPTION_MODAL_INTERACTION: Listener = Listener::new(
        "option_data_poll",
        "Save a poll option",
        ListenerKind::Modal,
        vec![
            ArgumentsLevel::ChannelId,
            ArgumentsLevel::Context,
            ArgumentsLevel::Guild,
            ArgumentsLevel::User,
            ArgumentsLevel::ModalSubmitData,
        ],
        Box::new(PollOptionModalReceiver),
    );
}
