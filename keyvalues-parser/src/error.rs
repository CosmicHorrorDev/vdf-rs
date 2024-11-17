//! All error information for parsing and rendering

use std::fmt;

/// An alias for `Result` with an [`RenderError`]
pub type RenderResult<T> = std::result::Result<T, RenderError>;

/// Errors encountered while rendering VDF text
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RenderError {
    RenderError(fmt::Error),
    RawRenderError { invalid_char: char },
}

impl From<fmt::Error> for RenderError {
    fn from(e: fmt::Error) -> Self {
        Self::RenderError(e)
    }
}

impl fmt::Display for RenderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RenderError(e) => write!(f, "Failed rendering input Error: {e}"),
            Self::RawRenderError { invalid_char } => write!(
                f,
                "Encountered invalid character in raw string: {invalid_char:?}"
            ),
        }
    }
}

impl std::error::Error for RenderError {}

/// An alias for `Result` with an [`Error`]
pub type ParseResult<T> = std::result::Result<T, ParseError>;

#[derive(Clone, Debug)]
pub struct ParseError {
    pub kind: ParseErrorKind,
    pub span: Span,
    line: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!();
    }
}

impl std::error::Error for ParseError {}

/// Errors encountered while parsing VDF text
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParseErrorKind {
    LingeringBytes,
    InvalidMacro,
    MissingTopLevelPair,
    EoiParsingString,
    ExpectedUnquotedString,
    InvalidEscapedCharacter,
    EoiParsingMap,
    InvalidComment,
}

impl fmt::Display for ParseErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LingeringBytes => f.write_str("Bytes remained after parsed pair"),
            Self::InvalidMacro => f.write_str("Invalid macro"),
            Self::MissingTopLevelPair => f.write_str("Missing top-level pair"),
            Self::EoiParsingString => {
                f.write_str("Encountered the end-of-input while pasing a string")
            }
            Self::ExpectedUnquotedString => f.write_str("Expected unquoted string"),
            Self::InvalidEscapedCharacter => f.write_str("Invalid escaped string character"),
            Self::EoiParsingMap => f.write_str("Encountered the end-of-input while pasing a map"),
            Self::InvalidComment => f.write_str("Invalid character in comment"),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Span {
    Single(usize),
    Run { index: usize, len: usize },
}

impl Span {
    pub(crate) fn run_with_len(index: usize, len: usize) -> Self {
        Span::Run { index, len }
    }
}

impl From<usize> for Span {
    fn from(index: usize) -> Self {
        Self::Single(index)
    }
}
