use crate::listeners::{Listener, ListenerKind};

#[derive(Clone)]
pub struct ListenerCollector {
    pub listeners: Vec<Listener>,
    pub length: usize,
}

impl ListenerCollector {
    pub fn new() -> Self {
        Self {
            listeners: Vec::new(),
            length: 0,
        }
    }

    /// Store a listener in the collector
    pub fn store_listener(&mut self, listener: Listener) {
        self.listeners.push(listener);
        self.length += 1;
    }

    /// Get all the listeners in the collector of a specific kind
    pub fn filter_listeners(&self, kind: ListenerKind) -> Vec<Listener> {
        self.listeners
            .iter()
            .filter(|listener| listener.kind == kind)
            .cloned()
            .collect()
    }
}
