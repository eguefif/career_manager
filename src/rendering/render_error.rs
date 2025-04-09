use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum RenderError {
    FileNotFound(String),
    TokenError(String),
    MissingContextKey(String),
}

impl Error for RenderError {}

impl fmt::Display for RenderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RenderError::MissingContextKey(value) => {
                write!(f, "Error: the context is missing a key: {}", value)
            }
            RenderError::TokenError(value) => write!(f, "Error: Error in template '{}'", value),
            RenderError::FileNotFound(filename) => {
                write!(f, "Error: template file could not be found ({}", filename)
            }
        }
    }
}
