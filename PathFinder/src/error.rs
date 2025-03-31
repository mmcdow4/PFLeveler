use std::fmt;

#[derive(Debug)]
pub enum PathFinderError {
    InvalidArgument(String),
}

impl fmt::Display for PathFinderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PathFinderError::InvalidArgument(ref msg) => write!(f, "Invalid Argument Error: {}", msg),
        }
    }
}

impl std::error::Error for PathFinderError {}