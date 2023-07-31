use super::Radio;

use songbird::{input::Input, ytdl};

pub async fn consumer(radio: Radio) -> Result<Input, Box<dyn std::error::Error>> {
    let url = radio.get_url();
    let input = ytdl(&url).await.unwrap();

    Ok(input)
}
