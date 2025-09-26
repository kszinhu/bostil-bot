use bostil_core::{
	arguments::CommandFnArguments,
	commands::{Command, CommandCategory, CommandContext},
	runners::{CommandResponse, CommandResult, CommandRunnerFn},
};
use lazy_static::lazy_static;
use serenity::{async_trait, builder::CreateCommand};

#[derive(Clone)]
struct Version;

#[async_trait]
impl CommandRunnerFn for Version {
	async fn run<'a>(&self, _: CommandFnArguments) -> CommandResult<'a> {
		Ok(CommandResponse::String(
			env!("CARGO_PKG_VERSION").to_string(),
		))
	}
}

lazy_static! {
	pub static ref VERSION_COMMAND: Command = Command::new(
		"version",
		"Bot version",
		CommandContext::Global,
		CommandCategory::Admin,
		vec![],
		Box::new(Version {}),
		Some(CreateCommand::new("version").description("shows the bot version"))
	);
}
