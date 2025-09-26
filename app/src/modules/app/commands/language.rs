use bostil_core::{
	arguments::{ArgumentsLevel, CommandFnArguments},
	commands::{Command, CommandCategory, CommandContext},
	database::exports::{establish_connection, Guild, LanguageEnum},
	gt as t,
	runners::{CommandResponse, CommandResult, CommandRunnerFn},
};
use diesel::result::Error::NotFound;
use lazy_static::lazy_static;
use serenity::{
	all::{CommandDataOption, CommandOptionType, Guild as SerenityGuild},
	async_trait,
	builder::{CreateCommand, CreateCommandOption},
};
use tracing::{debug, error};

#[derive(Clone)]
struct Language;

#[async_trait]
impl CommandRunnerFn for Language {
	async fn run<'a>(&self, args: CommandFnArguments) -> CommandResult<'a> {
		let current_guild = Guild::from(
			args
				.get(&ArgumentsLevel::Guild)
				.unwrap()
				.downcast_ref::<SerenityGuild>()
				.unwrap()
				.clone(),
		);
		let options = args
			.get(&ArgumentsLevel::Options)
			.unwrap()
			.downcast_ref::<Vec<CommandDataOption>>()
			.unwrap();
		let requested_language = LanguageEnum::from_str(
			options
				.iter()
				.filter(|option| option.name == "choose_language")
				.collect::<Vec<&CommandDataOption>>()[0]
				.value
				.as_str()
				.unwrap(),
		)
		.unwrap();

		debug!(
			"Setting {} language for guild {}",
			requested_language, current_guild.id
		);
		let connection = &mut establish_connection();

		match current_guild.update_language(connection, requested_language) {
			Ok(guild) => {
				debug!("Guild language updated: {:?}", guild);

				Ok(CommandResponse::String(
					t!("commands.language.reply", language_name => guild.language).to_string(),
				))
			}
			Err(e) => match e {
				NotFound => match current_guild.save(connection) {
					Ok(guild) => {
						debug!("Guild language updated: {:?}", guild);

						Ok(CommandResponse::String(
							t!("commands.language.reply", language_name => guild.language).to_string(),
						))
					}
					Err(e) => {
						error!("Error updating guild language: {:?}", e);

						Ok(CommandResponse::String(
							t!("commands.language.reply_error").to_string(),
						))
					}
				},
				_ => {
					error!("Error updating guild language: {:?}", e);

					Ok(CommandResponse::String(
						t!("commands.language.reply_error").to_string(),
					))
				}
			},
		}
	}
}

lazy_static! {
		/// Command to set the language of bot responses within a guild
		pub static ref LANGUAGE_COMMAND: Command = Command::new(
				"language",
				"Sets the language of the bot",
				CommandContext::Guild,
				CommandCategory::General,
				vec![ArgumentsLevel::Options, ArgumentsLevel::Guild],
				Box::new(Language),
				Some(
						CreateCommand::new("language")
								.description("Language Preferences Menu")
								.add_option(
										CreateCommandOption::new(
												CommandOptionType::String,
												"choose_language",
												"Choose the language of preference"
										)
										.add_string_choice("Portuguese", "pt-BR")
										.add_string_choice("English", "en-US")
										.required(true)
								),
				),
		);
}
