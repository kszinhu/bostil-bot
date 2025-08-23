use super::{equalizers::Equalizer, Radio};
use crate::modules::core::helpers::get_client;

use serenity::client::Context;
use songbird::input::{Input, YoutubeDl};

pub async fn get_source(
	radio: Radio,
	equalizer: Equalizer,
	ctx: &Context,
) -> Result<Input, String> {
	if let Some(url) = radio.get_url() {
		let http_client = get_client(ctx).await;
		let source = YoutubeDl::new(http_client, url).user_args(vec![
			"--postprocessor-args".to_string(),
			"-af".to_string(),
			equalizer.get_bands().join(","),
		]);

		Ok(source.into())
	} else {
		Err("Failed to get radio URL".to_string())
	}
}
