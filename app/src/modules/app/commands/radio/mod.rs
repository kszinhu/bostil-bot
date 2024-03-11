pub mod consumer;
pub mod equalizers;

use bostil_core::{
    arguments::{ArgumentsLevel, CommandFnArguments},
    commands::{Command, CommandCategory, CommandContext},
    runners::runners::{CommandResponse, CommandResult, CommandRunnerFn},
};
use lazy_static::lazy_static;
use rust_i18n::t;
use serenity::{
    all::{CommandDataOption, CommandDataOptionValue, CommandOptionType, Guild, User},
    async_trait,
    builder::{CreateCommand, CreateCommandOption},
    framework::standard::CommandResult as SerenityCommandResult,
    prelude::Context,
};
use tracing::{debug, error};

use crate::modules::core::actions::voice::join;

#[derive(Clone)]
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
impl CommandRunnerFn for RadioCommand {
    async fn run<'a>(&self, arguments: CommandFnArguments) -> CommandResult<'a> {
        let ctx = arguments
            .get(&ArgumentsLevel::Context)
            .unwrap()
            .downcast_ref::<Context>()
            .unwrap();
        let guild = arguments
            .get(&ArgumentsLevel::Guild)
            .unwrap()
            .downcast_ref::<Guild>()
            .unwrap();
        let user = arguments
            .get(&ArgumentsLevel::User)
            .unwrap()
            .downcast_ref::<User>()
            .unwrap();
        let options = arguments
            .get(&ArgumentsLevel::Options)
            .unwrap()
            .downcast_ref::<Vec<CommandDataOption>>()
            .unwrap();

        if let Err(_) = join(ctx, guild, &user.id).await {
            error!("Failed to join voice channel");

            return Ok(CommandResponse::String("Dá não pai".to_string()));
        }

        match run(options, ctx, guild).await {
            Ok(response) => Ok(CommandResponse::String(response)),
            Err(_) => Ok(CommandResponse::String("Deu não pai".to_string())),
        }
    }
}

pub async fn run(
    options: &Vec<CommandDataOption>,
    ctx: &Context,
    guild: &Guild,
) -> SerenityCommandResult<String> {
    let radio = match options[0].value.clone() {
        CommandDataOptionValue::String(radio) => match radio.as_str() {
            "Canoa Grande FM" => Radio::CanoaGrandeFM,
            "Pingo nos IFs" => Radio::PingoNosIFs,
            "Tupi FM" => Radio::TupiFM,
            "89 FM" => Radio::EightyNineFM,
            "88.3 FM" => Radio::EightyEightFM,
            "94 FM" => Radio::NinetyFourFm,
            _ => return Ok(t!("commands.radio.radio_not_found").to_string()),
        },
        _ => return Ok(t!("commands.radio.radio_not_found").to_string()),
    };

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild.id) {
        let mut voice_handler = handler_lock.lock().await;

        match consumer::get_source(radio, ctx).await {
            Ok(source) => {
                let _ = voice_handler.enqueue_input(source.into()).await;
                debug!("Playing radio: {}", radio.to_string());
            }
            Err(_) => {
                return Ok(t!("commands.radio.failed_to_get_radio_url").to_string());
            }
        }
    } else {
        debug!("Bot not connected to a voice channel");

        return Ok(t!("commands.radio.bot_not_connected").to_string());
    }

    Ok(t!("commands.radio.reply", "radio_name" => radio.to_string()).to_string())
}

lazy_static! {
    pub static ref RADIO_COMMAND: Command = Command::new(
        "radio",
        "Tune in to the best radios in \"Bostil\"",
        CommandContext::Guild,
        CommandCategory::Voice,
        vec![
            ArgumentsLevel::Options,
            ArgumentsLevel::Context,
            ArgumentsLevel::Guild,
            ArgumentsLevel::User,
        ],
        Box::new(RadioCommand),
        Some(
            CreateCommand::new("radio")
                .description("Tune in to the best radios in Bostil")
                .description_localized("pt-BR", "Sintonize a as melhores rádios do Bostil")
                .add_option(
                    CreateCommandOption::new(
                        CommandOptionType::String,
                        "radio",
                        "The radio to tune in",
                    )
                    .description_localized("pt-BR", "A rádio para sintonizar")
                    .kind(CommandOptionType::String)
                    .required(true)
                    .add_string_choice_localized(
                        "Canoa Grande FM",
                        Radio::CanoaGrandeFM.to_string(),
                        [("pt-BR", "Canoa Grande FM"), ("en-US", "Big Boat FM")],
                    )
                    .add_string_choice_localized(
                        "Pingo nos IFs",
                        Radio::PingoNosIFs.to_string(),
                        [("pt-BR", "Pingo nos IFs"), ("en-US", "Ping in the IFs")],
                    )
                    .add_string_choice_localized(
                        "Tupi FM",
                        Radio::TupiFM.to_string(),
                        [("pt-BR", "Tupi FM"), ("en-US", "Tupi FM")],
                    )
                    .add_string_choice_localized(
                        "88.3 FM",
                        Radio::EightyEightFM.to_string(),
                        [("pt-BR", "88.3 FM"), ("en-US", "88.3 FM")],
                    )
                    .add_string_choice_localized(
                        "89 FM",
                        Radio::EightyNineFM.to_string(),
                        [("pt-BR", "89 FM"), ("en-US", "89 FM")],
                    )
                    .add_string_choice_localized(
                        "94 FM",
                        Radio::NinetyFourFm.to_string(),
                        [("pt-BR", "94 FM"), ("en-US", "94 FM")],
                    ),
                ),
        ),
    );
}
