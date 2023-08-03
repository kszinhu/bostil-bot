use super::Radio;

use songbird::{input::Input, ytdl};

pub async fn consumer(radio: Radio) -> Result<Input, String> {
    let url = radio.get_url().unwrap();
    let input = ytdl(&url).await;

    match input {
        Ok(input) => Ok(input),
        Err(why) => Err(why.to_string()),
    }
}
