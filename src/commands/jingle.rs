use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

pub async fn run(_options: &Vec<CommandDataOption>) -> String {
    "Tanke o Bostil ou deixe-o".to_string()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("jingle")
        .description("Tanke o Bostil ou deixe-o")
}
