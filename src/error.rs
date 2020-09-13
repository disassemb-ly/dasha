use std::{error, fmt};

#[derive(Debug, PartialEq)]
pub enum Error {
    ExpectedModRm,
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::ExpectedModRm => write!(f, "expected mod-reg-r/m byte"),
        }
    }
}
