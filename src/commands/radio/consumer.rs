use crate::internal::debug::{log_message, MessageTypes};

use super::Radio;

use rust_i18n::t;
use songbird::{input::Input, ytdl};

pub async fn consumer(radio: Radio) -> Result<Input, String> {
    let url = radio.get_url();
    let input = ytdl(&url).await;

    match input {
        Ok(input) => Ok(input),
        Err(why) => {
            log_message(
                format!("Error starting source: {}", why).as_str(),
                MessageTypes::Error,
            );

            Err(t!("commands.radio.connection_error"))
        }
    }
}
