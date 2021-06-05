use keyvalues_parser::error::Error as ParserError;
use serde::{de, ser};

use std::{
    fmt::Display,
    io,
    num::{ParseFloatError, ParseIntError},
};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Message(String),

    #[error("Failed parsing VDF text")]
    Parse(#[from] ParserError),

    #[error("Encountered I/O Error: {0}")]
    Io(#[from] io::Error),

    #[error("Only finite f32 values are allowed. Instead got: {0}")]
    NonFiniteFloat(f32),

    #[error("EOF while parsing unknown type")]
    EofWhileParsingAny,
    #[error("EOF while parsing key")]
    EofWhileParsingKey,
    #[error("EOF while parsing a value")]
    EofWhileParsingValue,
    #[error("EOF while parsing key or value")]
    EofWhileParsingKeyOrValue,
    #[error("EOF while parsing an object")]
    EofWhileParsingObject,
    #[error("EOF while parsing a sequence")]
    EofWhileParsingSequence,

    #[error("Expected a valid token for object start")]
    ExpectedObjectStart,
    #[error("Expected some valid value")]
    ExpectedSomeValue,
    #[error("Expected a non-sequence value")]
    ExpectedSomeNonSeqValue,
    #[error("Expected some valid ident")]
    ExpectedSomeIdent,

    #[error("Tried parsing an invalid boolean")]
    InvalidBoolean,
    #[error("Tried parsing an invalid char")]
    InvalidChar,
    #[error("Tried parsing an invalid number")]
    InvalidNumber,

    #[error("Tokens remain after deserializing")]
    TrailingTokens,

    #[error("Unexpected end of object")]
    UnexpectedEndOfObject,
    #[error("Unexpected end of sequence")]
    UnexpectedEndOfSequence,

    #[error("Tried using unsupported type: {0}")]
    Unsupported(&'static str),
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Self::Message(msg.to_string())
    }
}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Self::Message(msg.to_string())
    }
}

impl From<ParseIntError> for Error {
    fn from(_: ParseIntError) -> Self {
        Self::InvalidNumber
    }
}

impl From<ParseFloatError> for Error {
    fn from(_: ParseFloatError) -> Self {
        Self::InvalidNumber
    }
}
