//! Contains error information for `keyvalues-serde`

// TODO: figure this out before the next breaking release
#![allow(clippy::large_enum_variant)]

use keyvalues_parser::error::ParseError;
use serde::{de, ser};

use std::{
    fmt, io,
    num::{ParseFloatError, ParseIntError},
};

/// Alias for the result with [`Error`] as the error type
pub type Result<T> = std::result::Result<T, Error>;

/// All the possible errors that can be encountered when (de)serializing VDF text
#[derive(Debug)]
pub enum Error {
    Message(String),
    Parse(ParseError),
    Io(io::Error),
    NonFiniteFloat(f32),
    EofWhileParsingAny,
    EofWhileParsingKey,
    EofWhileParsingValue,
    EofWhileParsingKeyOrValue,
    EofWhileParsingObject,
    EofWhileParsingSequence,
    ExpectedObjectStart,
    ExpectedSomeValue,
    ExpectedSomeNonSeqValue,
    ExpectedSomeIdent,
    InvalidBoolean,
    InvalidChar,
    InvalidNumber,
    TrailingTokens,
    UnexpectedEndOfObject,
    UnexpectedEndOfSequence,
    Unsupported(&'static str),
}

impl de::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Self::Message(msg.to_string())
    }
}

impl ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
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

impl From<ParseError> for Error {
    fn from(e: ParseError) -> Self {
        Self::Parse(e)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Message(msg) => f.write_str(msg),
            Self::Parse(_) => f.write_str("Failed parsing VDF text"),
            Self::Io(e) => write!(f, "Encountered I/O Error: {e}"),
            Self::NonFiniteFloat(non_finite) => {
                write!(
                    f,
                    "Only finite f32 values are allowed. Instead got: {non_finite}"
                )
            }
            Self::EofWhileParsingAny => f.write_str("EOF while parsing unknown type"),
            Self::EofWhileParsingKey => f.write_str("EOF while parsing key"),
            Self::EofWhileParsingValue => f.write_str("EOF while parsing a value"),
            Self::EofWhileParsingKeyOrValue => f.write_str("EOF while parsing key or value"),
            Self::EofWhileParsingObject => f.write_str("EOF while parsing an object"),
            Self::EofWhileParsingSequence => f.write_str("EOF while parsing a sequence"),
            Self::ExpectedObjectStart => f.write_str("Expected a valid token for object start"),
            Self::ExpectedSomeValue => f.write_str("Expected some valid value"),
            Self::ExpectedSomeNonSeqValue => f.write_str("Expected a non-sequence value"),
            Self::ExpectedSomeIdent => f.write_str("Expected some valid ident"),
            Self::InvalidBoolean => f.write_str("Tried parsing an invalid boolean"),
            Self::InvalidChar => f.write_str("Tried parsing an invalid char"),
            Self::InvalidNumber => f.write_str("Tried parsing an invalid number"),
            Self::TrailingTokens => f.write_str("Tokens remain after deserializing"),
            Self::UnexpectedEndOfObject => f.write_str("Unexpected end of object"),
            Self::UnexpectedEndOfSequence => f.write_str("Unexpected end of sequence"),
            Self::Unsupported(type_name) => write!(f, "Tried using unsupported type: {type_name}"),
        }
    }
}

impl std::error::Error for Error {}
