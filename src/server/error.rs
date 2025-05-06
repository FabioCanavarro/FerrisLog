use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum ServerError {
    FailedToReadStream { e: Box<dyn Error> },
    UnableToDecodeBytes { e: Box<dyn Error> },
    CommandNotFound,
    GetFoundNone,
}

impl Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FailedToReadStream { e } => {
                writeln!(f, "Failed to read from stream, Error: {}", e)
            }
            Self::UnableToDecodeBytes { e } => writeln!(f, "UnableToDecodeBytes, Error: {}", e),
            Self::CommandNotFound => writeln!(f, "Command is not found"),
            Self::GetFoundNone => writeln!(f, "Found None"),
        }
    }
}

impl Error for ServerError {}

