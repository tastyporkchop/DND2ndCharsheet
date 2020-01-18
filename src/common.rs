use std::error::Error;
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;

pub type Result<T> = std::result::Result<T, CharError>;

#[derive(Debug, Clone)]
pub enum CharError {
    StrParseError(ParseIntError),
    PercentParseError(ParseIntError),
    InvalidStrength(i32),
    InvalidPercentile(Option<i32>),
    CharacterClassParseError(String),
}

impl Display for CharError {
    fn fmt(&self, f: &mut Formatter) -> std::result::Result<(), std::fmt::Error> {
        match self {
            CharError::StrParseError(pe) => pe.fmt(f),
            CharError::PercentParseError(pe) => pe.fmt(f),
            CharError::InvalidStrength(i) => write!(f, "invalid strength:{}", i),
            CharError::InvalidPercentile(i) => write!(f, "invalid strength percentile:{:?}", i),
            CharError::CharacterClassParseError(i) => write!(f, "invalid class:{}", i),
        }
    }
}

impl Error for CharError {}
