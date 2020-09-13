use std::{error, fmt};

#[derive(Debug, PartialEq)]
pub enum Error {}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}
