use dyn_clone::DynClone;
use serenity::async_trait;
use std::any::Any;

#[async_trait]
pub trait ListenerRunnerFn: DynClone {
    async fn run<'a>(&self, arguments: &Vec<Box<dyn Any + Send + Sync>>) -> ();
}

dyn_clone::clone_trait_object!(ListenerRunnerFn);

impl std::fmt::Debug for dyn ListenerRunnerFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<RunnerFn>")
    }
}
