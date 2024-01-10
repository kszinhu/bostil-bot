pub mod consumer;
pub mod equalizers;

use rust_i18n::t;

use serenity::{
    all::{CommandDataOption, CommandDataOptionValue, CommandOptionType, Guild, User, UserId},
    async_trait,
    builder::CreateCommand,
    framework::standard::CommandResult,
    prelude::Context,
};

use crate::modules::core::{
    actions::voice::join,
    lib::debug::{log_message, MessageTypes},
};

use super::{
    ArgumentsLevel, Command, CommandCategory, CommandResponse, InternalCommandResult, RunnerFn,
};

struct RadioCommand;

#[derive(Debug, Clone, Copy)]
pub enum Radio {
    CanoaGrandeFM,
    TupiFM,
    EightyNineFM,
    EightyEightFM,
    NinetyFourFm,
    PingoNosIFs,
}

impl Radio {
    pub fn get_url(&self) -> Option<String> {
        match self {
            Radio::CanoaGrandeFM => {
                Some("https://servidor39-4.brlogic.com:8300/live?source=website".to_string())
            }
            Radio::TupiFM => Some("https://ice.fabricahost.com.br/topfmbauru".to_string()),
            Radio::EightyNineFM => Some("https://r13.ciclano.io:15223/stream".to_string()),
            Radio::EightyEightFM => Some("http://cast.hoost.com.br:8803/live.m3u".to_string()),
            Radio::NinetyFourFm => {
                Some("https://cast2.hoost.com.br:28456/stream?1691035067242".to_string())
            }
            Radio::PingoNosIFs => None,
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            Radio::CanoaGrandeFM => "Canoa Grande FM".to_string(),
            Radio::PingoNosIFs => "Pingo nos IFs".to_string(),
            Radio::TupiFM => "Tupi FM".to_string(),
            Radio::EightyNineFM => "89 FM".to_string(),
            Radio::EightyEightFM => "88.3 FM".to_string(),
            Radio::NinetyFourFm => "94 FM".to_string(),
        }
    }
}

impl std::fmt::Display for Radio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Radio::CanoaGrandeFM => write!(f, "Canoa Grande FM"),
            Radio::PingoNosIFs => write!(f, "Pingo nos IFs"),
            Radio::TupiFM => write!(f, "Tupi FM"),
            Radio::EightyNineFM => write!(f, "89 FM"),
            Radio::EightyEightFM => write!(f, "88.3 FM"),
            Radio::NinetyFourFm => write!(f, "94 FM"),
        }
    }
}

#[async_trait]
impl RunnerFn for RadioCommand {
    async fn run<'a>(
        &self,
        args: &Vec<Box<dyn std::any::Any + Send + Sync>>,
    ) -> InternalCommandResult<'a> {
        let ctx = args
            .iter()
            .filter_map(|arg| arg.downcast_ref::<Context>())
            .collect::<Vec<&Context>>()[0];
        let guild = args
            .iter()
            .filter_map(|arg| arg.downcast_ref::<Guild>())
            .collect::<Vec<&Guild>>()[0];
        let user_id = &args
            .iter()
            .filter_map(|arg| arg.downcast_ref::<User>())
            .collect::<Vec<&User>>()[0]
            .id;
        let options = args
            .iter()
            .filter_map(|arg| arg.downcast_ref::<Option<Vec<CommandDataOption>>>())
            .collect::<Vec<&Option<Vec<CommandDataOption>>>>()[0]
            .as_ref()
            .unwrap();

        match run(options, ctx, guild, user_id).await {
            Ok(response) => Ok(CommandResponse::String(response)),
            Err(_) => Ok(CommandResponse::None),
        }
    }
}

pub async fn run(
    options: &Vec<CommandDataOption>,
    ctx: &Context,
    guild: &Guild,
    user_id: &UserId,
) -> CommandResult<String> {
    let debug = std::env::var("DEBUG").is_ok();

    let radio = match options[0].resolved.as_ref().unwrap() {
        CommandDataOptionValue::String(radio) => match radio.as_str() {
            "Canoa Grande FM" => Radio::CanoaGrandeFM,
            "Pingo nos IFs" => Radio::PingoNosIFs,
            "Tupi FM" => Radio::TupiFM,
            "89 FM" => Radio::EightyNineFM,
            "88.3 FM" => Radio::EightyEightFM,
            "94 FM" => Radio::NinetyFourFm,
            _ => {
                return Ok(t!("commands.radio.radio_not_found"));
            }
        },
        _ => {
            return Ok(t!("commands.radio.radio_not_found"));
        }
    };

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if debug {
        log_message(
            format!("Radio: {}", radio.to_string()).as_str(),
            MessageTypes::Debug,
        );
    }

    join(ctx, guild, user_id).await?;

    if debug {
        log_message("Joined voice channel successfully", MessageTypes::Debug);
    }

    if let Some(handler_lock) = manager.get(guild.id) {
        let mut handler = handler_lock.lock().await;

        let source = match consumer::consumer(radio).await {
            Ok(source) => source,
            Err(why) => {
                log_message(
                    format!("Error while getting source: {}", why).as_str(),
                    MessageTypes::Error,
                );

                return Ok(t!("commands.radio.connection_error"));
            }
        };

        handler.play_source(source);
    } else {
        if debug {
            log_message("User not connected to a voice channel", MessageTypes::Debug);
        }

        return Ok(t!("commands.radio.user_not_connected"));
    }

    Ok(t!("commands.radio.reply", "radio_name" => radio.to_string()))
}

pub fn register(command: &mut CreateCommand) -> &mut CreateCommand {
    command
        .name("radio")
        .description("Tune in to the best radios in Bostil")
        .description_localized("pt-BR", "Sintonize a as melhores rádios do Bostil")
        .create_option(|option| {
            option
                .name("radio")
                .description("The radio to tune in")
                .description_localized("pt-BR", "A rádio para sintonizar")
                .kind(CommandOptionType::String)
                .required(true)
                .add_string_choice_localized(
                    "Canoa Grande FM",
                    Radio::CanoaGrandeFM,
                    [("pt-BR", "Canoa Grande FM"), ("en-US", "Big Boat FM")],
                )
                .add_string_choice_localized(
                    "Pingo nos IFs",
                    Radio::PingoNosIFs,
                    [("pt-BR", "Pingo nos IFs"), ("en-US", "Ping in the IFs")],
                )
                .add_string_choice_localized(
                    "Tupi FM",
                    Radio::TupiFM,
                    [("pt-BR", "Tupi FM"), ("en-US", "Tupi FM")],
                )
                .add_string_choice_localized(
                    "88.3 FM",
                    Radio::EightyEightFM,
                    [("pt-BR", "88.3 FM"), ("en-US", "88.3 FM")],
                )
                .add_string_choice_localized(
                    "89 FM",
                    Radio::EightyNineFM,
                    [("pt-BR", "89 FM"), ("en-US", "89 FM")],
                )
                .add_string_choice_localized(
                    "94 FM",
                    Radio::NinetyFourFm,
                    [("pt-BR", "94 FM"), ("en-US", "94 FM")],
                )
        })
        .into()
}

pub fn get_command() -> Command {
    Command::new(
        "radio",
        "Tune in to the best radios in \"Bostil\"",
        CommandCategory::Voice,
        vec![
            ArgumentsLevel::Options,
            ArgumentsLevel::Context,
            ArgumentsLevel::Guild,
            ArgumentsLevel::User,
        ],
        Box::new(RadioCommand {}),
    )
}
