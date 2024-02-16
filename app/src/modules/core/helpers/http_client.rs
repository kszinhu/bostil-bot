use reqwest::Client;
use serenity::client::Context;

use crate::HttpKey;

pub async fn get_client(ctx: &Context) -> Client {
    let data = ctx.data.read().await;

    data.get::<HttpKey>()
        .cloned()
        .expect("Guaranteed to exist in the typemap.")
}
