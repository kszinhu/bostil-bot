use std::cell::RefCell;

// uses a counter to keep track of how many times the bot has send a message
thread_local! {
    static COUNTER: RefCell<u32> = RefCell::new(0);
    static LAST_MESSAGE_TIME: RefCell<u32> = RefCell::new(0);
}

// returns a String or None
pub fn love() -> Option<String> {
    COUNTER.with(|counter| {
        LAST_MESSAGE_TIME.with(|last_message_time| {
            let mut counter = counter.borrow_mut();
            let mut last_message_time = last_message_time.borrow_mut();
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as u32;

            // check if the bot has sent a message in the last 5 seconds and if it has, don't send message
            if now - *last_message_time < 5 {
                *last_message_time = now;

                return None.into();
            } else {
                *last_message_time = now;
                *counter += 1;

                return format!("Eu te amo pela {}ª vez 😡", counter).into();
            }
        })
    })
}
