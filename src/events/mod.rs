use serenity::model::channel::Message;
use serenity::Result as SerenityResult;

pub mod voice;

pub fn check_msg(result: SerenityResult<Message>) {
    if let Err(why) = result {
        println!("Error sending message: {:?}", why);
    }
}
