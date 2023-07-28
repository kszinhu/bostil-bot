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
            let current_music = message
                .embeds
                .first()
                .unwrap()
                .description
                .as_ref()
                .unwrap();

            ctx.set_activity(Activity::listening(current_music)).await;
        }
        false => {}
    }
}

pub fn register() -> Integration {
    Integration::new(
        "jukera".to_string(),
        "Jukera Integration, Listening to jukes_box".to_string(),
        IntegrationType::Chat,
        Box::new(Jukera {}),
    )
}
