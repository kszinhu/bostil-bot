use bostil_core::{
	backends::i18n::DiscordI18n,
	collectors::{CommandCollector, ListenerCollector},
};
use lazy_static::lazy_static;
use reqwest::Client as HttpClient;
use serenity::prelude::TypeMapKey;

#[macro_use(i18n)]
extern crate rust_i18n;
extern crate diesel;
extern crate openssl;

struct ShardManagerContainer;
struct HttpKey;

impl TypeMapKey for ShardManagerContainer {
	type Value = std::sync::Arc<serenity::all::ShardManager>;
}

impl TypeMapKey for HttpKey {
	type Value = HttpClient;
}

pub mod modules;

// use CUSTOM_BACKEND to i18n! macro
i18n!(
	"public/locales",
	fallback = "en-US",
	backend = DiscordI18n::new()
);

lazy_static! {
	pub static ref COMMAND_COLLECTOR: std::sync::Mutex<CommandCollector> =
		std::sync::Mutex::new(CommandCollector::new());
	pub static ref LISTENER_COLLECTOR: std::sync::Mutex<ListenerCollector> =
		std::sync::Mutex::new(ListenerCollector::new());
}
