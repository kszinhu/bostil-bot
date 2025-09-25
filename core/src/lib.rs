use rust_i18n::i18n;

extern crate proc_macro;

pub mod arguments;
pub mod backends;
pub mod collectors;
pub mod commands;
pub mod contexts;
pub mod database;
pub mod embeds;
pub mod integrations;
pub mod listeners;
pub mod runners;
pub mod schema;

#[macro_use]
pub mod macros;

i18n!("../app/public/locales", fallback = "en-US");
