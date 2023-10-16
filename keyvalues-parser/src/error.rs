//! All error information for parsing and rendering

// TODO: is this still true?
// This library supports an MSRV of 1.42.0 which is before the addition of
// clippy::nonstandard_macro_braces. This lint is used within `thiserror` which in turn gets
// expanded out here causing clippy to throw out an unknown lint warning which fails CI. Until this
// gets resolved upstream I'm going to allow `unknown_clippy_lints` as a stopgap. Relevant:
// https://github.com/dtolnay/thiserror/issues/140
// https://github.com/dtolnay/thiserror/issues/141
#![allow(renamed_and_removed_lints)]
#![allow(clippy::unknown_clippy_lints)]

use thiserror::Error as ThisError;

use crate::text::parse::{EscapedPestError, RawPestError};

/// Just a type alias for `Result` with a [`Error`]
pub type Result<T> = std::result::Result<T, Error>;

// TODO: Swap out the `EscapedParseError` and `RawParseError` for an opaque `Error::Parse` variant
// that handles displaying the error
// TODO: should this whole thing be overhauled
// TODO: Sort out new MSRV

/// All possible errors when parsing or rendering VDF text
///
/// Currently the two variants are parse errors which currently only occurs when `pest` encounters
#[derive(ThisError, Clone, Debug, PartialEq, Eq)]
pub enum Error {
    #[error("Failed parsing input Error: {0}")]
    EscapedParseError(#[from] EscapedPestError),
    #[error("Failed parsing input Error: {0}")]
    RawParseError(#[from] RawPestError),
    #[error("Failed rendering input Error: {0}")]
    RenderError(#[from] std::fmt::Error),
    #[error("Encountered invalid character in raw string: {invalid_char:?}")]
    RawRenderError { invalid_char: char },
}
