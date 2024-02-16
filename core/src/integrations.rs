use tracing::info;

use crate::{
    arguments::ArgumentsLevel,
    listeners::{Listener, ListenerKind},
    runners::runners::ListenerRunnerFn,
};

/// Integration is a representation of listerner that interacts with some other service
#[derive(Clone)]
pub struct Integration {
    /// Name of the integration
    pub name: String,
    /// Description of the integration
    pub description: String,
    /// Arguments that the integration uses
    pub arguments: Vec<ArgumentsLevel>,
    /// Kind of the listener
    pub kind: ListenerKind,
    /// Runner of the integration when it is called
    pub runner: Box<dyn ListenerRunnerFn + Send + Sync>,
}

pub type CallbackParams = (String, String, Vec<ArgumentsLevel>, ListenerKind);

impl Integration {
    pub fn new(
        name: &str,
        description: &str,
        arguments: Vec<ArgumentsLevel>,
        kind: ListenerKind,
        runner: Box<dyn ListenerRunnerFn + Send + Sync>,
        callback: Option<impl Fn(CallbackParams) + Send + Sync + 'static>,
    ) -> Self {
        Self {
            kind,
            arguments: arguments.clone(),
            name: name.to_string(),
            description: description.to_string(),
            runner: {
                info!("Running {} integration", name);

                if callback.is_some() {
                    callback.unwrap()((name.to_string(), description.to_string(), arguments, kind));
                }

                runner.clone()
            },
        }
    }

    /// Propagate statics items to listener conversion
    pub fn to_listener(&self) -> Listener {
        self.into()
    }
}

// implements equal integration to listener
impl From<Integration> for Listener {
    fn from(integration: Integration) -> Self {
        Self {
            name: integration.name,
            description: integration.description,
            kind: integration.kind,
            arguments: integration.arguments,
            runner: integration.runner,
        }
    }
}

// implements equal listener to integration
impl From<&Integration> for Listener {
    fn from(integration: &Integration) -> Self {
        Self {
            name: integration.name.clone(),
            description: integration.description.clone(),
            kind: integration.kind,
            arguments: integration.arguments.clone(),
            runner: integration.runner.clone(),
        }
    }
}
