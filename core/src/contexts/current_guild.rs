use std::sync::Mutex;

use lazy_static::lazy_static;
use serenity::all::Guild;

lazy_static! {
    pub static ref CURRENT_GUILD: Mutex<Option<Guild>> = Mutex::new(None);
}
