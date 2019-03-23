use std::error;
use std::fmt::{self, Display, Formatter};
use std::io;

#[derive(Debug, PartialEq, Eq)]
pub enum ErrorKind {
    Io(io::ErrorKind),
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    source: Box<dyn error::Error>,
}

impl Error {
    pub fn io(source: io::Error) -> Self {
        Self {
            kind: ErrorKind::Io(source.kind()),
            source: Box::new(source),
        }
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }
}

impl error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.source.fmt(f)
    }
}

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Self {
        Error::io(other)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
