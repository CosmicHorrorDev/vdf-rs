//! All error information for parsing and rendering

use std::{
    fmt,
    ops::{RangeFrom, RangeInclusive},
};

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

/// An alias for `Result` with a [`ParseError`]
pub type ParseResult<T> = std::result::Result<T, ParseError>;

#[derive(Clone, Debug)]
pub struct ParseError {
    // TODO: move resolving the error into here and just pass in the original string instead of
    // having all of these be `pub(crate)` and constructing the error outside of this module
    pub(crate) inner: ParseErrorInner,
    pub(crate) index_span: Span<usize>,
    pub(crate) display_span: Span<LineCol>,
    pub(crate) lines: String,
    pub(crate) lines_start: usize,
}

impl ParseError {
    pub fn inner(&self) -> ParseErrorInner {
        self.inner
    }

    /// The span indicating where the error is in the original text
    ///
    /// # Example
    ///
    /// ```
    /// # use keyvalues_parser::Vdf;
    /// let vdf_text = "key value >:V extra bytes";
    /// let err = Vdf::parse(vdf_text).unwrap_err();
    /// let error_snippet = err.index_span().slice(vdf_text);
    /// assert_eq!(error_snippet, ">:V extra bytes");
    /// ```
    pub fn index_span(&self) -> Span<usize> {
        self.index_span.clone()
    }

    pub fn line_col_span(&self) -> Span<LineCol> {
        self.display_span.clone()
    }

    pub fn lines(&self) -> &str {
        &self.lines
    }

    pub fn error_snippet(&self) -> &str {
        let (mut start, end) = self.index_span.clone().into_inner();
        start -= self.lines_start;
        match end {
            Some(mut end) => {
                end -= self.lines_start;
                &self.lines[start..=end]
            }
            None => &self.lines[start..],
        }
    }
}

// TODO: we could avoid virtually all of the allocations done in here
// TODO: could use loooots of wrappers to clean up the display code
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            inner,
            display_span,
            lines,
            ..
        } = self;
        let (display_start, display_end) = display_span.clone().into_inner();

        writeln!(f, "error: {inner}")?;
        writeln!(f, "at: {display_span}")?;

        let mut lines_iter = lines.lines().zip(display_start.line..).peekable();
        while let Some((line, line_idx)) = lines_iter.next() {
            let line = line.replace('\n', " ").replace('\r', " ");
            let line_idx_str = line_idx.to_string();
            writeln!(f, "{line_idx_str} | {line}")?;

            let on_start_line = line_idx == display_start.line;
            let num_before = if on_start_line {
                display_start.col.saturating_sub(1)
            } else {
                0
            };
            let padding_before = " ".repeat(num_before);

            let (num_after, append_extra_arrow) = if let Some(display_end) = display_end {
                let num_after = line.len().checked_sub(display_end.col).unwrap();
                (num_after, false)
            } else {
                let is_last_line = lines_iter.peek().is_none();
                (0, is_last_line)
            };

            let num_arrows = line.len().checked_sub(num_before + num_after).unwrap()
                + append_extra_arrow as usize;
            let arrows = "^".repeat(num_arrows);

            let blank_idx = " ".repeat(line_idx_str.len());

            writeln!(f, "{blank_idx} | {padding_before}{arrows}")?;
        }

        Ok(())
    }
}

impl std::error::Error for ParseError {}

/// Errors encountered while parsing VDF text
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParseErrorInner {
    /// Indicates that there were significant bytes found after the top-level pair
    ///
    /// # Example
    ///
    /// ```
    /// # use keyvalues_parser::{Vdf, error::ParseErrorInner};
    /// let err = Vdf::parse("key value >:V extra bytes").unwrap_err();
    /// assert_eq!(err.inner(), ParseErrorInner::LingeringBytes);
    /// # print!("{err}");
    /// let expected = r#"
    /// error: Found bytes after the top-level pair
    /// at: 1:11 to the end of input
    /// 1 | key value >:V extra bytes
    ///   |           ^^^^^^^^^^^^^^^^
    /// "#.trim_start();
    /// assert_eq!(err.to_string(), expected);
    /// ```
    LingeringBytes,
    /// There was required whitespace that wasn't present
    ///
    /// There are very few places where whitespace is strictly _required_
    ///
    /// # Example
    ///
    /// ```
    /// # use keyvalues_parser::{Vdf, error::ParseErrorInner};
    /// let err = Vdf::parse("#baseBAD").unwrap_err();
    /// assert_eq!(err.inner(), ParseErrorInner::ExpectedWhitespace);
    /// # print!("{err}");
    /// let expected = r#"
    /// error: Expected whitespace
    /// at: 1:6
    /// 1 | #baseBAD
    ///   |      ^
    /// "#.trim_start();
    /// assert_eq!(err.to_string(), expected);
    /// ```
    ExpectedWhitespace,
    /// The required top-level key-value pair was missing
    ///
    /// # Example
    ///
    /// ```
    /// # use keyvalues_parser::{Vdf, error::ParseErrorInner};
    /// let err = Vdf::parse("#base robot_standard.pop").unwrap_err();
    /// assert_eq!(err.inner(), ParseErrorInner::ExpectedNewlineAfterMacro);
    /// # print!("{err}");
    /// let expected = r#"
    /// error: Expected newline after macro definition
    /// at: 1:25 to the end of input
    /// 1 | #base robot_standard.pop
    ///   |                         ^
    /// "#.trim_start();
    /// assert_eq!(err.to_string(), expected);
    /// ```
    ExpectedNewlineAfterMacro,
    /// Encountered the end of input while parsing a string
    ///
    /// # Example
    ///
    /// ```
    /// # use keyvalues_parser::{Vdf, error::ParseErrorInner};
    /// let err = Vdf::parse("key \"incomplete ...").unwrap_err();
    /// assert_eq!(err.inner(), ParseErrorInner::EoiParsingQuotedString);
    /// # print!("{err}");
    /// let expected = r#"
    /// error: Encountered the end of input while parsing a quoted string
    /// at: 1:5 to the end of input
    /// 1 | key "incomplete ...
    ///   |     ^^^^^^^^^^^^^^^^
    /// "#.trim_start();
    /// assert_eq!(err.to_string(), expected);
    /// ```
    EoiParsingQuotedString,
    EoiExpectedMacroPath,
    InvalidMacroPath,
    // TODO: remove this error variant in favor of a MissingTopLevelPair and
    //       ExpectedPairKeyOrMapClose
    EoiExpectedPairKey,
    InvalidPairKey,
    EoiExpectedPairValue,
    InvalidPairValue,
    /// Encountered an invalid escape character in a quoted string
    ///
    /// # Example
    ///
    /// ```
    /// # use keyvalues_parser::{Vdf, error::ParseErrorInner};
    /// let err = Vdf::parse(r#"key "invalid -> \u""#).unwrap_err();
    /// # print!("{err}");
    /// let expected = r#"
    /// error: Invalid escaped string character \u
    /// at: 1:17 to 1:18
    /// 1 | key "invalid -> \u"
    ///   |                 ^^
    /// "#.trim_start();
    /// assert_eq!(err.to_string(), expected);
    /// ```
    InvalidEscapedCharacter {
        invalid: char,
    },
    /// Encountered the end of input while parsing a map
    ///
    /// # Example
    ///
    /// ```
    /// # use keyvalues_parser::{Vdf, error::ParseErrorInner};
    /// let err = Vdf::parse("key {\n  foo {}").unwrap_err();
    /// assert_eq!(err.inner(), ParseErrorInner::EoiParsingMap);
    /// # print!("{err}");
    /// let expected = r#"
    /// error: Encountered the end of input while parsing a map
    /// at: 1:5 to the end of input
    /// 1 | key {
    ///   |     ^
    /// 2 |   foo {}
    ///   | ^^^^^^^^^
    /// "#.trim_start();
    /// assert_eq!(err.to_string(), expected);
    /// ```
    EoiParsingMap,
    /// Encountered an invalid control character while parsing a comment
    ///
    /// # Example
    ///
    /// ```
    /// # use keyvalues_parser::{Vdf, error::ParseErrorInner};
    /// let err = Vdf::parse("// \0 is invalid").unwrap_err();
    /// assert_eq!(err.inner(), ParseErrorInner::CommentControlCharacter);
    /// # print!("{err}");
    /// let expected = "
    /// error: Encountered an invalid control character while parsing a comment
    /// at: 1:4
    /// 1 | // \0 is invalid
    ///   |    ^
    /// ".trim_start();
    /// assert_eq!(err.to_string(), expected);
    /// ```
    ///
    /// ```
    /// # use keyvalues_parser::{Vdf, error::ParseErrorInner};
    /// let err = Vdf::parse("// Only valid before a newline -> \r uh oh").unwrap_err();
    /// assert_eq!(err.inner(), ParseErrorInner::CommentControlCharacter);
    /// # print!("{err}");
    /// let expected = r#"
    /// error: Encountered an invalid control character while parsing a comment
    /// at: 1:35
    /// 1 | // Only valid before a newline ->   uh oh
    ///   |                                   ^
    /// "#.trim_start();
    /// assert_eq!(err.to_string(), expected);
    /// ```
    // TODO: pretty up the display of `\r` akin to how we did in sd?
    // TODO: store the invalid character
    CommentControlCharacter,
}

pub enum Component {
    MacroPath,
    PairKey,
    PairValue,
}

impl fmt::Display for ParseErrorInner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LingeringBytes => f.write_str("Found bytes after the top-level pair"),
            Self::ExpectedWhitespace => f.write_str("Expected whitespace"),
            Self::ExpectedNewlineAfterMacro => {
                f.write_str("Expected newline after macro definition")
            }
            Self::EoiExpectedMacroPath => {
                f.write_str("Found the end of input while looking for a macro path")
            }
            Self::InvalidMacroPath => {
                f.write_str("Encountered an invalid character while parsing a macro path")
            }
            Self::EoiExpectedPairKey => {
                f.write_str("Encountered the end of input while looking for a pair's key")
            }
            Self::InvalidPairKey => {
                f.write_str("Encountered an invalid character while looking for a pair's key")
            }
            Self::EoiExpectedPairValue => {
                f.write_str("Encountered the end of input while looking for a pair's key")
            }
            Self::InvalidPairValue => {
                f.write_str("Encountered an invalid character while looking for a pair's key")
            }
            Self::EoiParsingQuotedString => {
                f.write_str("Encountered the end of input while parsing a quoted string")
            }
            Self::InvalidEscapedCharacter { invalid } => {
                write!(f, "Invalid escaped string character \\{invalid}")
            }
            Self::EoiParsingMap => f.write_str("Encountered the end of input while parsing a map"),
            Self::CommentControlCharacter => {
                f.write_str("Encountered an invalid control character while parsing a comment")
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct LineCol {
    pub line: usize,
    pub col: usize,
}

impl Default for LineCol {
    fn default() -> Self {
        Self { line: 1, col: 1 }
    }
}
impl fmt::Display for LineCol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { line, col } = self;
        write!(f, "{line}:{col}")
    }
}

// TODO: Hide internals so that we can make changes without breaking the public API later
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Span<T> {
    Inclusive(RangeInclusive<T>),
    ToEoi(RangeFrom<T>),
}

impl<T> Span<T> {
    pub fn new(start: T, maybe_end: Option<T>) -> Self {
        match maybe_end {
            Some(end) => Self::Inclusive(start..=end),
            None => Self::ToEoi(start..),
        }
    }

    pub fn into_inner(self) -> (T, Option<T>) {
        match self {
            Self::Inclusive(r) => {
                let (start, end) = r.into_inner();
                (start, Some(end))
            }
            Self::ToEoi(r) => (r.start, None),
        }
    }
}

impl Span<usize> {
    pub fn slice<'span, 'text>(&'span self, s: &'text str) -> &'text str {
        match self.to_owned() {
            Self::Inclusive(r) => &s[r],
            Self::ToEoi(r) => &s[r],
        }
    }
}

impl<T> From<RangeInclusive<T>> for Span<T> {
    fn from(r: RangeInclusive<T>) -> Self {
        Self::Inclusive(r)
    }
}

impl<T> From<RangeFrom<T>> for Span<T> {
    fn from(r: RangeFrom<T>) -> Self {
        Self::ToEoi(r)
    }
}

impl<T: fmt::Display + PartialEq> fmt::Display for Span<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Inclusive(r) => {
                if r.start() == r.end() {
                    write!(f, "{}", r.start())
                } else {
                    write!(f, "{} to {}", r.start(), r.end())
                }
            }
            Self::ToEoi(r) => write!(f, "{} to the end of input", r.start),
        }
    }
}
