#[macro_use(i18n)]
extern crate rust_i18n;

i18n!("./public/static/locales", fallback = "en");

pub mod commands;
pub mod integrations;
pub mod interactions;
pub mod internal;
