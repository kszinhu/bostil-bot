use bostil_core::collectors::{CommandCollector, ListenerCollector};
use lazy_static::lazy_static;
use reqwest::Client as HttpClient;
use songbird::typemap::TypeMapKey;

#[macro_use(i18n)]
extern crate rust_i18n;
extern crate diesel;

struct ShardManagerContainer;
struct HttpKey;

impl TypeMapKey for ShardManagerContainer {
    type Value = std::sync::Arc<serenity::all::ShardManager>;
}

impl TypeMapKey for HttpKey {
    type Value = HttpClient;
}

// TODO: implementar algum jeito para que cada servidor tenha seu próprio idioma e não alterar o idioma de todos os servidores
i18n!("public/locales", fallback = "en-US");

pub mod modules;
pub mod schema;

lazy_static! {
    pub static ref COMMAND_COLLECTOR: std::sync::Mutex<CommandCollector> =
        std::sync::Mutex::new(CommandCollector::new());
    pub static ref LISTENER_COLLECTOR: std::sync::Mutex<ListenerCollector> =
        std::sync::Mutex::new(ListenerCollector::new());
}
