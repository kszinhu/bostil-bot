use super::Radio;
use crate::modules::core::helpers::get_client;

use serenity::client::Context;
use songbird::input::{Input, YoutubeDl};

pub async fn get_source(radio: Radio, ctx: &Context) -> Result<Input, String> {
    if let Some(url) = radio.get_url() {
        let http_client = get_client(ctx).await;
        let source = YoutubeDl::new(http_client, url);

        Ok(source.into())
    } else {
        Err("Failed to get radio URL".to_string())
    }
}
