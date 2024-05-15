use diesel::result::Error as DieselError;
use std::fmt;

#[derive(Debug)]
pub enum CoreError {
    NotFound,
    InternalServerError,
}

pub trait Error {
    fn as_core_error(&self) -> CoreError;
}

pub fn adapt_error<T: Error>(error: T) -> CoreError {
    error.as_core_error()
}

impl fmt::Display for CoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CoreError::NotFound => write!(f, "Not found"),
            CoreError::InternalServerError => write!(f, "Internal server error"),
        }
    }
}

impl Error for DieselError {
    fn as_core_error(&self) -> CoreError {
        match self {
            DieselError::NotFound => CoreError::NotFound,
            _ => CoreError::InternalServerError,
        }
    }
}
