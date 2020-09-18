use std::{error, fmt};

#[derive(Debug, PartialEq)]
pub enum Error {
    ExpectedModRm,
    ExpectedSib,
    ExpectedImmByte,
    ExpectedImmLong,
    ExpectedOffsetByte,
    ExpectedOffsetLong,
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::ExpectedModRm => write!(f, "expected mod-reg-r/m byte"),
            Error::ExpectedSib => write!(f, "expected scale-index-base byte"),
            Error::ExpectedImmByte => write!(f, "expected 1 immediate byte (byte)"),
            Error::ExpectedImmLong => write!(f, "expected 4 immediate bytes (long)"),
            Error::ExpectedOffsetByte => write!(f, "expected 1 offset byte (byte)"),
            Error::ExpectedOffsetLong => write!(f, "expected 4 offset bytes (long)"),
        }
    }
}
