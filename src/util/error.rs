use std::fmt::{self, Display};

pub type GSResult<T> = std::result::Result<T, GSError>;

#[derive(Debug)]
pub enum GSError {
    Internal(String),
    Parse(String),
}

impl Display for GSError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GSError::Internal(s) | GSError::Parse(s) => write!(f, "{}", s),
        }
    }
}

impl From<std::io::Error> for GSError {
    fn from(err: std::io::Error) -> Self {
        GSError::Internal(err.to_string())
    }
}

impl From<std::num::ParseIntError> for GSError {
    fn from(err: std::num::ParseIntError) -> Self {
        GSError::Parse(err.to_string())
    }
}
