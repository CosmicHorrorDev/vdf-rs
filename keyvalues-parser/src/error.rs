//! All error information for parsing and rendering

use std::fmt;

use crate::text::parse::{EscapedPestError, RawPestError};

/// Just a type alias for `Result` with a [`Error`]
pub type Result<T> = std::result::Result<T, Error>;

// TODO: Swap out the `EscapedParseError` and `RawParseError` for an opaque `Error::Parse` variant
// that handles displaying the error
// TODO: should this whole thing be overhauled (future me here: yes)
// TODO: split the `Error` into a separate parse and render error

/// All possible errors when parsing or rendering VDF text
///
/// Currently the two variants are parse errors which currently only occurs when `pest` encounters
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    EscapedParseError(EscapedPestError),
    RawParseError(RawPestError),
    RenderError(fmt::Error),
    RawRenderError { invalid_char: char },
}

impl From<EscapedPestError> for Error {
    fn from(e: EscapedPestError) -> Self {
        Self::EscapedParseError(e)
    }
}

impl From<RawPestError> for Error {
    fn from(e: RawPestError) -> Self {
        Self::RawParseError(e)
    }
}

impl From<std::fmt::Error> for Error {
    fn from(e: std::fmt::Error) -> Self {
        Self::RenderError(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EscapedParseError(e) => write!(f, "Failed parsing input Error: {e}"),
            Self::RawParseError(e) => write!(f, "Failed parsing input Error: {e}"),
            Self::RenderError(e) => write!(f, "Failed rendering input Error: {e}"),
            Self::RawRenderError { invalid_char } => write!(
                f,
                "Encountered invalid character in raw string: {invalid_char:?}"
            ),
        }
    }
}

impl std::error::Error for Error {}
