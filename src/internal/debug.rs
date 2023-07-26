use colored::Colorize;

pub struct Status {
    message: &'static str,
    color: &'static str,
}

pub const STATUS_OK: Status = Status {
    message: "SUCCESS",
    color: "green",
};
pub const STATUS_ERROR: Status = Status {
    message: "ERROR",
    color: "red",
};
pub const STATUS_INFO: Status = Status {
    message: "INFO",
    color: "blue",
};

pub fn log_message(message: &str, status: &Status) {
    println!(
        "{} - {}",
        status.message.color(status.color),
        message.color("white")
    );
}
