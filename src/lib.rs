#[macro_use(i18n)]
extern crate rust_i18n;

i18n!("./public/locales", fallback = "en-US");

pub mod commands;
pub mod components;
pub mod database;
pub mod events;
pub mod integrations;
pub mod interactions;
pub mod internal;
pub mod modules;
