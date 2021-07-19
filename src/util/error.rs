use std::fmt::{self, Display};

pub type Result<T> = std::result::Result<T, Error>;

pub enum Error {
    Internal(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Internal(s) => write!(f, "{}", s),
        }
    }
}
