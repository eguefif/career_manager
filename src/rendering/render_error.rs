use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum RenderError {
    FileNotFound(String),
    TokenError(String),
}

impl Error for RenderError {}

impl fmt::Display for RenderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RenderError::TokenError(value) => write!(f, "Error: Error in template '{}'", value),
            RenderError::FileNotFound(filename) => {
                write!(f, "Error: template file could not be found ({}", filename)
            }
        }
    }
}
