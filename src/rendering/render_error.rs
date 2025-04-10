use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum RenderError {
    TokenError,
    MissingContextKey(String),
    WrongValueTypeForForGen,
    EOF,
}

impl Error for RenderError {}

impl fmt::Display for RenderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RenderError::MissingContextKey(value) => {
                write!(f, "Error: the context is missing a key: {}", value)
            }
            RenderError::WrongValueTypeForForGen => {
                write!(f, "Error: For generator expects a Context")
            }
            RenderError::EOF => write!(f, "Error: Unexpected end of file"),
            RenderError::TokenError => write!(f, "Error: Error in template"),
        }
    }
}
