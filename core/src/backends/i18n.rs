//! Guild-aware internationalization system
//!
//! This module provides macros and utilities to handle translations
//! based on the current Discord guild's language preference stored in the database.

use crate::database::exports::{
	establish_connection, Guild, GuildIdWrapper, LanguageEnum as Language,
};
use rust_i18n::t;
use serenity::all::GuildId;
use std::collections::HashMap;
use std::sync::{Mutex, RwLock};

/// Thread-safe storage for current guild context
static CURRENT_GUILD_ID: RwLock<Option<GuildId>> = RwLock::new(None);

/// Cache for guild languages to avoid repeated database queries
static GUILD_LANGUAGE_CACHE: std::sync::LazyLock<Mutex<HashMap<GuildId, Language>>> =
	std::sync::LazyLock::new(|| Mutex::new(HashMap::new()));

/// Sets the current guild context for translations
pub fn set_current_guild(guild_id: GuildId) {
	if let Ok(mut current) = CURRENT_GUILD_ID.write() {
		*current = Some(guild_id);
	}
}

/// Clears the current guild context
pub fn clear_current_guild() {
	if let Ok(mut current) = CURRENT_GUILD_ID.write() {
		*current = None;
	}
}

/// Gets the current guild ID from context
fn get_current_guild() -> Option<GuildId> {
	CURRENT_GUILD_ID.read().ok()?.clone()
}

/// Gets the language for a specific guild, with caching
fn get_guild_language(guild_id: GuildId) -> Language {
	// Try to get from cache first
	if let Ok(cache) = GUILD_LANGUAGE_CACHE.lock() {
		if let Some(language) = cache.get(&guild_id) {
			return *language;
		}
	}

	let language = {
		let connection = &mut establish_connection();
		match Guild::get_by_id(connection, GuildIdWrapper(guild_id)) {
			Some(guild) => guild.language,
			None => Language::En, // Default to English if not found
		}
	};

	if let Ok(mut cache) = GUILD_LANGUAGE_CACHE.lock() {
		cache.insert(guild_id, language);
	}

	language
}

/// Clears the language cache for a specific guild
pub fn invalidate_guild_language_cache(guild_id: GuildId) {
	if let Ok(mut cache) = GUILD_LANGUAGE_CACHE.lock() {
		cache.remove(&guild_id);
	}
}

/// Clears the entire language cache
pub fn clear_language_cache() {
	if let Ok(mut cache) = GUILD_LANGUAGE_CACHE.lock() {
		cache.clear();
	}
}

/// Guild-aware translation function
///
/// This function automatically uses the current guild's language preference
/// or falls back to the provided locale or default language.
#[allow(dead_code)]
pub fn translate_for_guild(
	key: &str,
	locale: Option<&str>,
	args: Option<&[(&str, &dyn std::fmt::Display)]>,
) -> String {
	let target_locale = match get_current_guild() {
		Some(guild_id) => get_guild_language(guild_id).to_string(),
		None => locale.unwrap_or("en-US").to_string(),
	};

	match args {
		Some(args) => {
			let mut result = t!(key, locale = &target_locale).to_string();
			for (placeholder, value) in args {
				result = result.replace(&format!("%{{{}}}", placeholder), &value.to_string());
			}
			result
		}
		None => t!(key, locale = &target_locale).to_string(),
	}
}
