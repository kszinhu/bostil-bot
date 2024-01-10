use rust_i18n::t;
use serenity::{builder::CreateEmbed, framework::standard::CommandResult};

use crate::{
    commands::poll::{PartialPoll, PollStage},
    internal::embeds::{ApplicationEmbed, EmbedRunnerFn},
};
struct PollSetupEmbed;

impl EmbedRunnerFn for PollSetupEmbed {
    fn run(&self, arguments: &Vec<Box<dyn std::any::Any + Send + Sync>>) -> CreateEmbed {
        let poll = arguments[0].downcast_ref::<PartialPoll>().unwrap();
        let stage = arguments[1].downcast_ref::<PollStage>().unwrap();

        runner(poll.clone(), stage.clone()).unwrap()
    }
}

fn runner(poll: PartialPoll, stage: PollStage) -> CommandResult<CreateEmbed> {
    let mut embed = CreateEmbed::default();

    embed.color(stage.embed_color());

    match stage {
        PollStage::Setup => {
            embed.title(t!("commands.poll.setup.embed.stages.setup.title"));
            embed.description(t!("commands.poll.setup.embed.stages.setup.description"));

            embed.field(
                "ID",
                poll.id
                    .map_or(t!("commands.poll.setup.embed.id_none"), |id| id.to_string()),
                true,
            );
            embed.field("User", format!("<@{}>", poll.created_by), true);
            embed.field("\u{200B}", "\u{200B}", false); // Separator
        }
        PollStage::Voting => {
            embed.title(t!("commands.poll.setup.embed.stages.voting.title"));
            embed.description(t!("commands.poll.setup.stages.voting.description"));
        }
        PollStage::Closed => {
            embed.title(t!("commands.poll.setup.embed.stages.closed.title"));
            embed.description(t!("commands.poll.setup.stages.closed.description"));
        }
    }

    Ok(embed)
}

pub fn get_embed() -> ApplicationEmbed {
    ApplicationEmbed {
        name: "Poll Setup".to_string(),
        description: Some("Embed to configure poll".to_string()),
        builder: Box::new(PollSetupEmbed),
        arguments: vec![
            Box::new(None::<Option<PartialPoll>>),
            Box::new(None::<Option<PollStage>>),
        ],
        message_content: None,
        message: None,
    }
}
