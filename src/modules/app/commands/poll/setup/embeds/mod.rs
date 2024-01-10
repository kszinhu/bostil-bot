use serenity::builder::CreateEmbed;
use serenity::model::prelude::{ChannelId, GuildId, MessageId};
use serenity::prelude::Context;

use crate::commands::poll::PollDatabaseModel;

pub mod setup;
pub mod vote;

/**
 * EmbedPoll is a struct that represents the embed message that is sent to the channel
 *
 * builder is a function that takes a PollDatabaseModel sends a message to the channel
 */
struct EmbedPoll {
    name: String,
    message_id: Option<MessageId>,
    channel_id: ChannelId,
    guild_id: GuildId,
    builder: Box<dyn Fn(PollDatabaseModel) -> CreateEmbed + Send + Sync>,
}

impl EmbedPoll {
    pub async fn update_message(&self, poll: PollDatabaseModel, ctx: Context) -> () {
        let mut message = self
            .channel_id
            .message(&ctx.http, self.message_id.unwrap())
            .await
            .unwrap();

        let embed = self.builder.as_ref()(poll);

        message
            .edit(&ctx.http, |m| m.set_embed(embed))
            .await
            .unwrap();
    }

    pub async fn remove_message(&self, ctx: Context) {
        self.channel_id
            .delete_message(&ctx.http, self.message_id.unwrap())
            .await
            .unwrap();
    }
}
