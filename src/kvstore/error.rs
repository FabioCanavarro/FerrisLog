use serde::Deserialize;
use std::error::Error;
use std::fmt;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub enum KvError {
    WriteError,
    ReadError,
    OpenError { path: PathBuf },
    ParseError,
    RemoveError,
    EngineError
}

impl fmt::Display for KvError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KvError::WriteError => writeln!(f, "Writing has failed!"),
            KvError::ReadError => writeln!(f, "Unable to read!"),
            KvError::OpenError { path: _ } => writeln!(f, "Opening has failed!"),
            KvError::ParseError => writeln!(f, "Parsing has failed!"),
            KvError::RemoveError => writeln!(f, "Unable to remove!"),
            KvError::EngineError => writeln!(f, "None Value is Found, Command has failed!!!"),
        }
    }
}

impl Error for KvError {}

pub type KvResult<T> = Result<T, KvError>;
