use std::{error, fmt};

#[derive(Debug, Clone)]
pub enum EngineError {
    StateError,
    SdlError(String),
}

impl fmt::Display for EngineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: Display detailed error message
        write!(f, "EngineError: {:?}", self)
    }
}

impl error::Error for EngineError {}
