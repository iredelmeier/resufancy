use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SassError(String);

impl SassError {
    pub fn new(err: String) -> Self {
        SassError(err)
    }
}

impl Error for SassError {}

impl Display for SassError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl From<String> for SassError {
    fn from(other: String) -> Self {
        SassError::new(other)
    }
}
