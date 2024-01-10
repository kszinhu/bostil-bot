use super::{equalizers::RADIO_EQUALIZER, Radio};

use songbird::input::Input;

pub async fn consumer(radio: Radio) -> Result<Input, String> {
    let url = radio.get_url().unwrap();
    let input = ffmpeg_optioned(
        &url,
        &[],
        RADIO_EQUALIZER
            .get_filter()
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<&str>>()
            .as_slice(),
    )
    .await;

    match input {
        Ok(input) => {
            log_message(
                format!(
                    "Playing radio: {}\n\tWith equalizer: {}",
                    radio, RADIO_EQUALIZER.name
                )
                .as_str(),
                MessageTypes::Info,
            );

            Ok(input)
        }
        Err(why) => Err(why.to_string()),
    }
}
