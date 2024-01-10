use colored::Colorize;

#[derive(Debug, Clone, Copy)]
enum DebugLevel {
    Info,
    Error,
    Success,
    Verbose,
    Minimal,
}

impl DebugLevel {
    fn get_level(&self) -> Vec<u8> {
        match self {
            DebugLevel::Minimal => vec![1],
            DebugLevel::Info => vec![1, 2, 3],
            DebugLevel::Success => vec![2],
            DebugLevel::Error => vec![3],
            DebugLevel::Verbose => vec![1, 2, 3, 4],
        }
    }
    fn get_current_level() -> DebugLevel {
        let debug_level = std::env::var("DEBUG").unwrap_or("minimal".to_string());

        match debug_level.as_str() {
            "minimal" => DebugLevel::Minimal,
            "info" => DebugLevel::Info,
            "success" => DebugLevel::Success,
            "error" => DebugLevel::Error,
            "verbose" => DebugLevel::Verbose,
            _ => DebugLevel::Minimal,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MessageTypes {
    Info,
    Error,
    Success,
    Failed,
    Server,
    Debug,
}

impl MessageTypes {
    fn get_console_color(&self) -> &'static str {
        match self {
            MessageTypes::Info => "blue",
            MessageTypes::Error => "red",
            MessageTypes::Success => "green",
            MessageTypes::Failed => "red",
            MessageTypes::Server => "yellow",
            MessageTypes::Debug => "white",
        }
    }
    fn get_console_prefix(&self) -> &'static str {
        match self {
            MessageTypes::Info => "INFO",
            MessageTypes::Error => "ERROR",
            MessageTypes::Success => "SUCCESS",
            MessageTypes::Failed => "FAILED",
            MessageTypes::Server => "SERVER",
            MessageTypes::Debug => "DEBUG",
        }
    }
    fn get_debug_level(&self) -> u8 {
        match self {
            MessageTypes::Info => 1,
            MessageTypes::Success => 2,
            MessageTypes::Failed => 2,
            MessageTypes::Error => 3,
            MessageTypes::Server => 1,
            MessageTypes::Debug => 4,
        }
    }
}

pub fn log_message(message: &str, message_type: MessageTypes) {
    let debug_level = DebugLevel::get_current_level();

    if !DebugLevel::get_level(&debug_level).contains(&message_type.get_debug_level()) {
        return;
    }

    println!(
        "{} - {}",
        message_type
            .get_console_prefix()
            .color(message_type.get_console_color()),
        message
    );
}
