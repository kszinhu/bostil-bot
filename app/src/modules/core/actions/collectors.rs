use bostil_core::collectors::{CommandCollector, ListenerCollector};

use crate::modules::app::{
    commands::commands, listeners::chat, services::integrations::integrations,
};

/// Command registration
pub fn register_commands(collector: &mut CommandCollector) {
    let commands = [
        commands::language.to_command(),
        commands::ping.to_command(),
        commands::jingle.to_command(),
        commands::poll.to_command(),
        commands::radio.to_command(),
        commands::join.to_command(),
        commands::leave.to_command(),
        commands::mute.to_command(),
    ];

    for command in commands.iter().cloned() {
        collector.store_command(command);
    }
}

/// Store all the integrations
pub fn register_integrations(collector: &mut ListenerCollector) {
    let integrations = [integrations::JUKERA_INTEGRATION.to_listener()];

    for integration in integrations.iter().cloned() {
        collector.store_listener(integration);
    }
}

/// Store all the listeners
pub fn register_listeners(collector: &mut ListenerCollector) {
    let listeners = [chat::LOVE_LISTENER.to_listener()];

    for listener in listeners.iter().cloned() {
        collector.store_listener(listener);
    }
}
