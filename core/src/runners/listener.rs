use dyn_clone::DynClone;
use serenity::all::{async_trait, standard::CommandResult, Embed};

use crate::arguments::ListenerFnArguments;

/// ListenerResponse is a type of response that the listener can return
#[derive(Debug, Clone)]
pub enum ListenerResponse {
	String(String),
	Embed(Embed),
	None,
}

/// ListenerResult is a type of result (ok or error) that the listener can return
pub type ListenerResult = CommandResult<ListenerResponse>;

/// Function that will be executed when the listener is called
#[async_trait]
pub trait ListenerRunnerFn: DynClone {
	async fn run<'a>(&self, arguments: ListenerFnArguments) -> ListenerResult;
}

dyn_clone::clone_trait_object!(ListenerRunnerFn);

impl std::fmt::Debug for dyn ListenerRunnerFn {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "<RunnerFn>")
	}
}
