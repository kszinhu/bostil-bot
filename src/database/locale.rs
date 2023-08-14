use std::borrow::BorrowMut;

use super::{get_database, save_database};
use crate::{
    database::GuildDatabaseModel,
    internal::debug::{log_message, MessageTypes},
};

use rust_i18n::{available_locales, set_locale};
use serenity::model::prelude::GuildId;

pub fn apply_locale(new_locale: &str, guild_id: &GuildId, is_preflight: bool) {
    if available_locales!().contains(&new_locale) {
        let local_database = get_database();

        if let Some(guild) = local_database.lock().unwrap().guilds.get(guild_id) {
            if guild.locale == new_locale {
                return;
            } else if guild.locale != new_locale && is_preflight {
                set_locale(guild.locale.as_str());

                return;
            }
        }

        set_locale(new_locale);

        local_database.lock().unwrap().guilds.insert(
            *guild_id,
            GuildDatabaseModel {
                locale: new_locale.to_string(),
                polls: Vec::new(),
            },
        );

        save_database(local_database.lock().unwrap().borrow_mut());

        log_message(
            format!("Applied locale {} for guild {}", new_locale, guild_id).as_str(),
            MessageTypes::Success,
        );
    } else {
        log_message(
            format!("Locale {} not available for guild {}", new_locale, guild_id).as_str(),
            MessageTypes::Failed,
        );
    }
}
