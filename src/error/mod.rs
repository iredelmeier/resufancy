use std::error;
use std::fmt::{self, Display, Formatter};
use std::io;
use std::string::FromUtf8Error;

use pug::{self, RuleType};

use crate::error::sass_error::SassError;

pub mod sass_error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorKind {
    Io(io::ErrorKind),
    Pug,
    Sass,
    InvalidUtf8,
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

    pub fn pug<R: 'static + RuleType>(source: pug::Error<R>) -> Self {
        Self {
            kind: ErrorKind::Pug,
            source: Box::new(source),
        }
    }

    pub fn sass(source: SassError) -> Self {
        Self {
            kind: ErrorKind::Sass,
            source: Box::new(source),
        }
    }

    pub fn invalid_utf8(source: FromUtf8Error) -> Self {
        Self {
            kind: ErrorKind::InvalidUtf8,
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

impl<R: 'static + RuleType> From<pug::Error<R>> for Error {
    fn from(other: pug::Error<R>) -> Self {
        Error::pug(other)
    }
}

impl From<SassError> for Error {
    fn from(other: SassError) -> Self {
        Error::sass(other)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(other: FromUtf8Error) -> Self {
        Error::invalid_utf8(other)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
