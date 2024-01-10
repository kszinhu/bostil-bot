use super::{CallbackFn, Integration, IntegrationType};
use crate::internal::users::USERS;

use serenity::async_trait;
use serenity::model::{channel::Message, gateway::Activity, id::UserId};
use serenity::prelude::Context;

struct Jukera {}

#[async_trait]
impl CallbackFn for Jukera {
    async fn run(&self, message: &Message, ctx: &Context, user_id: &UserId) {
        run(message, ctx, user_id).await;
    }
}

async fn run(message: &Message, ctx: &Context, user_id: &UserId) {
    match user_id == USERS.get("jukes_box").unwrap() {
        true => {
            // check if message is a embed message (music session)
            if message.embeds.is_empty() {
                ctx.set_activity(Activity::competing(
                    "Campeonato de Leitada, Modalidade: Volume",
                ))
                .await;

                return;
            }

            let current_music = match message.embeds.first() {
                Some(embed) => embed.description.as_ref().unwrap(),
                None => return,
            };

            ctx.set_activity(Activity::listening(current_music)).await
        }
        false => {}
    }
}

pub fn register() -> Integration {
    Integration::new(
        "jukera",
        "Jukera Integration, Listening to jukes_box",
        IntegrationType::Chat,
        Box::new(Jukera {}),
    )
}
