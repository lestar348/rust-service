use std::fmt::{self, Display};

/// Common error type for the service project.
#[derive(Debug, Clone)]
pub enum Error {
    Configuration(String),
    Unsupported(String),
    Transport(String),
    Feature(String),
    Other(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Configuration(msg)
            | Error::Unsupported(msg)
            | Error::Transport(msg)
            | Error::Feature(msg)
            | Error::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for Error {}
