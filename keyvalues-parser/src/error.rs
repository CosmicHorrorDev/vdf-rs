//! All error information for parsing and rendering

use std::fmt;

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
    RenderError(fmt::Error),
    RawRenderError { invalid_char: char },
    Todo,
}

impl From<fmt::Error> for Error {
    fn from(e: fmt::Error) -> Self {
        Self::RenderError(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RenderError(e) => write!(f, "Failed rendering input Error: {e}"),
            Self::RawRenderError { invalid_char } => write!(
                f,
                "Encountered invalid character in raw string: {invalid_char:?}"
            ),
            Self::Todo => write!(f, "TODO"),
        }
    }
}

impl std::error::Error for Error {}
