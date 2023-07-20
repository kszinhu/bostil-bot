use std::cell::RefCell;

use serenity::client::Context;
use serenity::model::prelude::ChannelId;

thread_local! {
    static COUNTER: RefCell<u32> = RefCell::new(0);
    static LAST_MESSAGE_TIME: RefCell<u32> = RefCell::new(0);
}

pub async fn love(channel: &ChannelId, ctx: &Context) -> () {
    let message = COUNTER.with(|counter| {
        LAST_MESSAGE_TIME.with(|last_message_time| {
            let mut counter = counter.borrow_mut();
            let mut last_message_time = last_message_time.borrow_mut();
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as u32;

            if now - *last_message_time < 5 {
                *last_message_time = now;

                return None.into();
            } else {
                *last_message_time = now;
                *counter += 1;

                if *counter == 1 {
                    return format!("Eu te amo ❤️").into();
                }

                return format!("Eu te amo pela {}ª vez 😡", counter).into();
            }
        })
    });

    if let Some(message) = message {
        if let Err(why) = channel.say(&ctx.http, message).await {
            println!("Error sending message: {:?}", why);
        }
    }
}
