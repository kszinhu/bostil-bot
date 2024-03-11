use downcast::{downcast, Any as DownAny};
use dyn_clone::{clone_trait_object, DynClone};
use std::collections::HashMap;

pub trait Value: DynClone + DownAny + Sync + Send {}
clone_trait_object!(Value);
downcast!(dyn Value);

impl<T: Clone + DownAny + Send + Sync> Value for T {}
impl std::fmt::Debug for dyn Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Value").finish_non_exhaustive()
    }
}

#[derive(Debug, Clone)]
pub struct ArgumentsStruct {
    pub level: ArgumentsLevel,
    pub value: Option<Box<dyn Value>>,
}

pub type ArgumentsHashMap = HashMap<ArgumentsLevel, Box<dyn Value>>;
pub type CommandFnArguments = ArgumentsHashMap;
pub type ApplicationEmbedFnArguments = ArgumentsHashMap;

/**
 Arguments to provide to a run function
*/
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd)]
pub enum ArgumentsLevel {
    None,
    Options,
    Context,
    Guild,
    User,
    InteractionId,
    ChannelId,
    ModalSubmitData,
    Message,
    PollId,
    PollStage,
}

impl ArgumentsLevel {
    pub fn value(&self) -> u8 {
        match self {
            ArgumentsLevel::None => 0,
            ArgumentsLevel::Options => 1,
            ArgumentsLevel::Context => 2,
            ArgumentsLevel::Guild => 3,
            ArgumentsLevel::User => 4,
            ArgumentsLevel::InteractionId => 5,
            ArgumentsLevel::ChannelId => 6,
            ArgumentsLevel::ModalSubmitData => 7,
            ArgumentsLevel::Message => 8,
            ArgumentsLevel::PollId => 9,
            ArgumentsLevel::PollStage => 10,
        }
    }

    // function to provide the arguments to the run function
    pub fn provide(
        requested_arguments: &Vec<ArgumentsLevel>,
        provided_arguments: &ArgumentsHashMap,
    ) -> ArgumentsHashMap {
        for argument in requested_arguments {
            match provided_arguments.get(argument) {
                Some(value) => value,
                None => panic!("Argument {:?} not provided", argument),
            };
        }

        provided_arguments.clone()
    }
}
