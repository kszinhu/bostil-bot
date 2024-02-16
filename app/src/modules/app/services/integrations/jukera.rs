use bostil_core::{
    arguments::ArgumentsLevel,
    integrations::{CallbackParams, Integration},
    listeners::ListenerKind,
    runners::runners::ListenerRunnerFn,
};
use diesel::{query_dsl::methods::FilterDsl, ExpressionMethods, RunQueryDsl};
use lazy_static::lazy_static;
use serenity::{
    all::{Context, Message, UserId},
    async_trait,
    gateway::ActivityData,
};
use std::any::Any;

use crate::modules::core::{entities::user::User, helpers::establish_connection};

#[derive(Clone)]
struct Jukera;

#[async_trait]
impl ListenerRunnerFn for Jukera {
    async fn run<'a>(&self, arguments: &Vec<Box<dyn Any + Send + Sync>>) {
        let ctx = arguments
            .iter()
            .filter_map(|arg| arg.downcast_ref::<Context>())
            .collect::<Vec<&Context>>()[0];
        let message = arguments
            .iter()
            .filter_map(|arg| arg.downcast_ref::<Message>())
            .collect::<Vec<&Message>>()[0];
        let user_id = arguments
            .iter()
            .filter_map(|arg| arg.downcast_ref::<UserId>())
            .collect::<Vec<&UserId>>()[0];

        run(&message, &ctx, &user_id).await;
    }
}

async fn run(message: &Message, ctx: &Context, user_id: &UserId) {
    use crate::schema::users::dsl::{username, users};

    let connection = &mut establish_connection();
    let user = users
        .filter(username.eq("Isadora"))
        .first::<User>(connection)
        .unwrap() as User;

    match user.id == *user_id {
        true => {
            // check if message is a embed message (music session)
            match message.embeds.is_empty() {
                true => {
                    ctx.set_activity(Some(ActivityData::competing(
                        "Campeonato de Leitada, Modalidade: Volume",
                    )));
                }
                false => {
                    let current_music = match message.embeds.first() {
                        Some(embed) => embed.description.as_ref().unwrap(),
                        None => return,
                    };

                    ctx.set_activity(Some(ActivityData::listening(current_music)))
                }
            }
        }
        false => {}
    }
}

lazy_static! {
    /// # Jukera integration
    ///
    /// > On listen messages from jukera check if the user currently listening to music and set the activity
    pub static ref JUKERA_INTEGRATION: Integration = Integration::new(
        "jukera",
        "Listening to jukes_box",
        vec![
            ArgumentsLevel::Context,
            ArgumentsLevel::User,
            ArgumentsLevel::Message,
        ],
        ListenerKind::Message,
        Box::new(Jukera),
        None::<fn(CallbackParams)>
    );
}
