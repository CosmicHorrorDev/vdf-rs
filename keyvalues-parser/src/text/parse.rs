use std::{borrow::Cow, str::Chars};

use crate::{
    error::{LineCol, ParseError, ParseErrorInner, ParseResult, Span},
    Key, Obj, PartialVdf, Value, Vdf,
};

// TODO: rename `PartialVdf` to `TopLevelVdf` and have it hold a `Vdf` instead of flattening it out

impl<'a> PartialVdf<'a> {
    /// Attempts to parse VDF text to a [`PartialVdf`]
    pub fn parse(s: &'a str) -> ParseResult<Self> {
        escaped_parse(s)
    }

    pub fn parse_raw(s: &'a str) -> ParseResult<Self> {
        raw_parse(s)
    }
}

impl<'a> Vdf<'a> {
    /// Attempts to parse VDF text to a [`Vdf`]
    pub fn parse(s: &'a str) -> ParseResult<Self> {
        Ok(Vdf::from(PartialVdf::parse(s)?))
    }

    pub fn parse_raw(s: &'a str) -> ParseResult<Self> {
        Ok(Vdf::from(PartialVdf::parse_raw(s)?))
    }
}

pub fn raw_parse(s: &str) -> ParseResult<PartialVdf> {
    parse_(s, false)
}

pub fn escaped_parse(s: &str) -> ParseResult<PartialVdf> {
    parse_(s, true)
}

pub fn parse_(s: &str, escape_chars: bool) -> ParseResult<PartialVdf> {
    let mut chars = CharIter::new(s);

    let bases = parse_macros(&mut chars)?;
    // TODO: this can store strs instead of cows
    let bases = bases.into_iter().map(Cow::Borrowed).collect();
    let Vdf { key, value } = parse_pair(&mut chars, escape_chars)?;

    eat_comments_whitespace_and_newlines(&mut chars)?;
    // Some VDF files are terminated with a null byte. Just C things I guess :shrug:
    let _ = chars.next_if_eq('\x00');
    eat_comments_whitespace_and_newlines(&mut chars)?;

    if chars.peek().is_some() {
        Err(chars.err(ParseErrorInner::LingeringBytes, chars.index()..))
    } else {
        Ok(PartialVdf { bases, key, value })
    }
}

fn parse_macros<'text>(chars: &mut CharIter<'text>) -> ParseResult<Vec<&'text str>> {
    let mut macros = Vec::new();
    loop {
        eat_comments_whitespace_and_newlines(chars)?;

        if parse_maybe_macro(chars, &mut macros)?.is_none() {
            break Ok(macros);
        }

        eat_comments_and_whitespace(chars)?;
    }
}

fn parse_maybe_macro<'text>(
    chars: &mut CharIter<'text>,
    macros: &mut Vec<&'text str>,
) -> ParseResult<Option<()>> {
    // FIXME: this should also support `#include` too
    if !chars.next_n_if_eq(['#', 'b', 'a', 's', 'e']) {
        return Ok(None);
    }

    if !eat_whitespace(chars) {
        let start_idx = chars.index();
        let err_span: Span<_> = match chars.next() {
            None => (start_idx..).into(),
            Some(_) => (start_idx..=start_idx).into(),
        };
        return Err(chars.err(ParseErrorInner::ExpectedWhitespace, err_span));
    }

    let macro_ = parse_quoted_raw_or_unquoted_string(chars)?;
    macros.push(macro_);

    eat_comments_and_whitespace(chars)?;

    if eat_newlines(chars) {
        Ok(Some(()))
    } else {
        Err(chars.err(ParseErrorInner::ExpectedNewlineAfterMacro, chars.index()..))
    }
}

fn parse_quoted_raw_or_unquoted_string<'text>(
    chars: &mut CharIter<'text>,
) -> ParseResult<&'text str> {
    if chars.peek() == Some('"') {
        parse_quoted_raw_string(chars)
    } else {
        parse_unquoted_string(chars).map_err(|(kind, span)| {
            let kind = match kind {
                InvalidUnquotedString::Eoi => ParseErrorInner::EoiExpectedMacroPath,
                InvalidUnquotedString::InvalidChar => ParseErrorInner::InvalidMacroPath,
            };
            chars.err(kind, span)
        })
    }
}

// TODO: error on `\r` or `\n` in quoted str (wait no i think that's valid)
fn parse_quoted_raw_string<'text>(chars: &mut CharIter<'text>) -> ParseResult<&'text str> {
    assert!(chars.ensure_next('"'));
    let start_idx = chars.index();
    while chars
        .next()
        .ok_or_else(|| chars.err(ParseErrorInner::EoiParsingQuotedString, start_idx..))?
        != '"'
    {}
    let end_idx = chars.index() - '"'.len_utf8();
    Ok(&chars.original_str()[start_idx..end_idx])
}

// Unquoted strings are often used as the fallthrough for various alternations. This can lead to
// confusing error messages when treating them with a global error type, so instead we force a
// vague local error that gets translated and bubbled up depending on where it's called
enum InvalidUnquotedString {
    Eoi,
    InvalidChar,
}

fn parse_unquoted_string<'text>(
    chars: &mut CharIter<'text>,
) -> Result<&'text str, (InvalidUnquotedString, Span<usize>)> {
    let start_idx = chars.index();

    match chars
        .next()
        .ok_or((InvalidUnquotedString::Eoi, (0..).into()))?
    {
        '"' | '{' | '}' | ' ' | '\t' | '\r' | '\n' => {
            return Err((InvalidUnquotedString::InvalidChar, (0..).into()));
        }
        _ => {}
    }

    loop {
        match chars.peek() {
            // The wiki page just states that an unquoted string ends with ", {, }, or any
            // whitespace which I feel is likely missing several cases, but for now I will follow
            // that information
            None | Some('"' | '{' | '}' | ' ' | '\t' | '\r' | '\n') => {
                let s = chars.original_str();
                let end_idx = chars.index();
                break Ok(&s[start_idx..end_idx]);
            }
            _ => _ = chars.next(),
        }
    }
}

fn parse_pair<'text>(chars: &mut CharIter<'text>, escape_chars: bool) -> ParseResult<Vdf<'text>> {
    let key = if chars.peek() == Some('"') {
        parse_quoted_string(chars, escape_chars)
    } else {
        match parse_unquoted_string(chars) {
            Ok(s) => Ok(Cow::Borrowed(s)),
            Err((kind, span)) => {
                let kind = match kind {
                    InvalidUnquotedString::Eoi => ParseErrorInner::EoiExpectedPairKey,
                    InvalidUnquotedString::InvalidChar => ParseErrorInner::InvalidPairKey,
                };
                Err(chars.err(kind, span))
            }
        }
    }?;
    eat_comments_whitespace_and_newlines(chars)?;
    let value = parse_value(chars, escape_chars)?;
    Ok(Vdf { key, value })
}

fn parse_quoted_string<'text>(
    chars: &mut CharIter<'text>,
    escape_chars: bool,
) -> ParseResult<Key<'text>> {
    let str_start_index = chars.index();
    assert!(chars.ensure_next('"'));

    let start_idx = chars.index();
    loop {
        let peeked = chars
            .peek()
            .ok_or_else(|| chars.err(ParseErrorInner::EoiParsingQuotedString, str_start_index..))?;
        // We only care about potential escaped characters if `escape_chars` is set. Otherwise we
        // only break on " for a quoted string
        if peeked == '"' || (peeked == '\\' && escape_chars) {
            break;
        }
        chars.unwrap_next();
    }

    let end_idx = chars.index();
    let text_chunk = &chars.original_str()[start_idx..end_idx];
    // If our string contains escaped characters then it has to be a `Cow::Owned` otherwise it can
    // be `Cow::Borrowed`
    let key = if chars
        .peek()
        .ok_or_else(|| chars.err(ParseErrorInner::EoiParsingQuotedString, str_start_index..))?
        == '"'
    {
        assert!(chars.ensure_next('"'));
        Cow::Borrowed(text_chunk)
    } else {
        assert!(chars.peek().unwrap() == '\\');
        let mut escaped = text_chunk.to_owned();
        loop {
            let ch = chars.next().ok_or_else(|| {
                chars.err(ParseErrorInner::EoiParsingQuotedString, str_start_index..)
            })?;
            match ch {
                '"' => break Cow::Owned(escaped),
                '\\' => match chars.next().ok_or_else(|| {
                    chars.err(ParseErrorInner::EoiParsingQuotedString, str_start_index..)
                })? {
                    'n' => escaped.push('\n'),
                    'r' => escaped.push('\r'),
                    't' => escaped.push('\t'),
                    '\\' => escaped.push('\\'),
                    '\"' => escaped.push('\"'),
                    invalid => {
                        // Backtrack to reconstruct the error span since this is a hot loop and we
                        // don't want to track a span for every character
                        let err_span_end = chars.index() - invalid.len_utf8();
                        let err_span_start = err_span_end - '\\'.len_utf8();
                        return Err(chars.err(
                            ParseErrorInner::InvalidEscapedCharacter { invalid },
                            err_span_start..=err_span_end,
                        ));
                    }
                },
                regular => escaped.push(regular),
            }
        }
    };

    Ok(key)
}

fn parse_value<'text>(
    chars: &mut CharIter<'text>,
    escape_chars: bool,
) -> ParseResult<Value<'text>> {
    let value = match chars.peek() {
        Some('{') => {
            let obj = parse_obj(chars, escape_chars)?;
            Value::Obj(obj)
        }
        Some('"') => {
            let s = parse_quoted_string(chars, escape_chars)?;
            Value::Str(s)
        }
        _ => match parse_unquoted_string(chars) {
            Ok(s) => Value::Str(Cow::Borrowed(s)),
            Err((kind, span)) => {
                let kind = match kind {
                    InvalidUnquotedString::Eoi => ParseErrorInner::EoiExpectedPairValue,
                    InvalidUnquotedString::InvalidChar => ParseErrorInner::InvalidPairValue,
                };
                return Err(chars.err(kind, span));
            }
        },
    };
    Ok(value)
}

fn parse_obj<'text>(chars: &mut CharIter<'text>, escape_chars: bool) -> ParseResult<Obj<'text>> {
    let err_span_start = chars.index();
    assert!(chars.ensure_next('{'));
    eat_comments_whitespace_and_newlines(chars)?;

    let mut obj = Obj::new();

    while chars
        .peek()
        // TODO: Switch error to Expected pair key or end of map
        .ok_or_else(|| chars.err(ParseErrorInner::EoiParsingMap, err_span_start..))?
        != '}'
    {
        // TODO: assert that the error isn't eoi parsing key because that should be represented in
        // the error above
        let Vdf { key, value } = parse_pair(chars, escape_chars)?;
        obj.0.entry(key).or_default().push(value);

        eat_comments_whitespace_and_newlines(chars)?;
    }
    assert!(chars.ensure_next('}'));

    Ok(obj)
}

fn eat_comments_whitespace_and_newlines(chars: &mut CharIter<'_>) -> ParseResult<bool> {
    let mut ate = false;
    while eat_whitespace_and_newlines(chars) || eat_comments(chars)? {
        ate = true;
    }

    Ok(ate)
}

fn eat_comments_and_whitespace(chars: &mut CharIter<'_>) -> ParseResult<bool> {
    let mut ate = false;
    while eat_comments(chars)? || eat_whitespace(chars) {
        ate = true;
    }

    Ok(ate)
}

fn eat_whitespace_and_newlines(chars: &mut CharIter<'_>) -> bool {
    let mut ate = false;
    while eat_whitespace(chars) || eat_newlines(chars) {
        ate = true;
    }

    ate
}

// All characters other than some control characters are permitted
fn eat_comments(chars: &mut CharIter<'_>) -> ParseResult<bool> {
    if !chars.next_n_if_eq(['/', '/']) {
        Ok(false)
    } else {
        loop {
            match chars.peek() {
                Some('\r') => {
                    let err_index = chars.index();
                    chars.unwrap_next();
                    match chars.next() {
                        Some('\n') => break,
                        _ => {
                            return Err(chars.err(
                                ParseErrorInner::CommentControlCharacter,
                                err_index..=err_index,
                            ))
                        }
                    }
                }
                None | Some('\n') => break,
                // Various control characters
                Some('\u{00}'..='\u{08}' | '\u{0A}'..='\u{1F}' | '\u{7F}') => {
                    let err_index = chars.index();
                    return Err(chars.err(
                        ParseErrorInner::CommentControlCharacter,
                        err_index..=err_index,
                    ));
                }
                _ => _ = chars.unwrap_next(),
            }
        }

        Ok(true)
    }
}

fn eat_whitespace(chars: &mut CharIter<'_>) -> bool {
    let mut ate = false;
    while ['\t', ' '].map(Some).contains(&chars.peek()) {
        chars.unwrap_next();
        ate = true;
    }

    ate
}

fn eat_newlines(chars: &mut CharIter<'_>) -> bool {
    let mut ate = false;
    loop {
        match chars.peek_n() {
            [Some('\n'), _] => {
                chars.unwrap_next();
                ate = true;
            }
            [Some('\r'), Some('\n')] => {
                chars.unwrap_next_n::<2>();
                ate = true;
            }
            _ => break,
        }
    }

    ate
}

/// Convenience wrapper around `Chars`
#[derive(Clone)]
struct CharIter<'text> {
    it: Chars<'text>,
    idx: usize,
    text: &'text str,
}

impl<'text> CharIter<'text> {
    fn new(text: &'text str) -> Self {
        Self {
            it: text.chars(),
            idx: 0,
            text,
        }
    }

    #[must_use]
    fn original_str(&self) -> &'text str {
        self.text
    }

    #[must_use]
    fn index(&self) -> usize {
        self.idx
    }

    #[must_use]
    fn ensure_next(&mut self, c: char) -> bool {
        self.ensure_next_n([c])
    }

    #[must_use]
    fn ensure_next_n<const N: usize>(&mut self, ensure: [char; N]) -> bool {
        ensure
            .iter()
            .all(|&ensure_elem| self.next() == Some(ensure_elem))
    }

    fn peek(&self) -> Option<char> {
        let [maybe_c] = self.peek_n();
        maybe_c
    }

    #[must_use]
    fn peek_n<const N: usize>(&self) -> [Option<char>; N] {
        let mut lookahead = self.clone();
        lookahead.next_n()
    }

    #[must_use]
    fn next_n<const N: usize>(&mut self) -> [Option<char>; N] {
        let mut arr = [None; N];
        for elem in arr.iter_mut() {
            *elem = self.next();
        }
        arr
    }

    #[must_use]
    fn next_if_eq(&mut self, c: char) -> bool {
        self.next_n_if_eq([c])
    }

    #[must_use]
    fn next_n_if_eq<const N: usize>(&mut self, cs: [char; N]) -> bool {
        if self.peek_n() == cs.map(Some) {
            self.unwrap_next_n::<N>();
            true
        } else {
            false
        }
    }

    fn unwrap_next(&mut self) -> char {
        self.next().unwrap()
    }

    fn unwrap_next_n<const N: usize>(&mut self) -> [char; N] {
        let mut arr = ['\0'; N];
        for elem in arr.iter_mut() {
            *elem = self.next().unwrap();
        }
        arr
    }

    /// Emit an error given its kind and span
    ///
    /// We don't keep track of line/col during parsing to keep the happy path fast. Instead we track
    /// indices into the original string and only translate them to line/col and resolve the full
    /// lines that the error lies on when we go to emit the error while we still have access to the
    /// original text
    #[must_use]
    fn err(&self, inner: ParseErrorInner, index_span: impl Into<Span<usize>>) -> ParseError {
        let index_span = index_span.into();

        let (start, end) = index_span.clone().into_inner();

        // TODO: switch this to be peekable, so that we can peek instead of breaking so eagerly?
        // Hopefully that removes the need to manually track `last_index` and `last_c`
        let mut chars = self.original_str().char_indices();
        let mut line_col = LineCol::default();
        let mut current_line_start = 0;

        // Resolve the line/col for the start of the span and find the start of the line
        let mut last_index = 0;
        let mut last_c = '\0';
        let (error_lines_start, error_span_start) = loop {
            let Some((i, c)) = chars.next() else {
                break (current_line_start, line_col);
            };
            last_index = i;
            last_c = c;

            if i >= start {
                break (current_line_start, line_col);
            }

            if c == '\n' {
                current_line_start = i + '\n'.len_utf8();
                line_col.col = 1;
                line_col.line += 1;
            } else {
                line_col.col += 1;
            }
        };

        let maybe_end = end.map(|end| {
            assert!(start <= end, "No backwards error span");
            // Resolve the line/col for the end of the span and maybe we happened to be at the end of
            // the final line
            let (maybe_error_lines_end, error_span_end) = if last_index >= end {
                let maybe_error_lines_end = (last_c == '\n').then_some(current_line_start);
                (maybe_error_lines_end, line_col)
            } else {
                loop {
                    let Some((i, c)) = chars.next() else {
                        break (None, line_col);
                    };
                    last_index = i;

                    if c == '\n' {
                        current_line_start = i + '\n'.len_utf8();
                        line_col.col = 1;
                        line_col.line += 1;
                    } else {
                        line_col.col += 1;
                    }

                    if i >= end {
                        let maybe_error_lines_end = (c == '\n').then_some(current_line_start);
                        break (maybe_error_lines_end, line_col);
                    }
                }
            };

            // Find the end of the last error line
            let error_lines_end = maybe_error_lines_end.unwrap_or_else(|| loop {
                match chars.next() {
                    Some((i, '\n')) => break i,
                    Some((i, _)) => last_index = i,
                    // FIXME if we run up to the EOI trying to find the end of the error line then
                    // we should have it be `None` instead of the final index
                    None => break last_index,
                }
            });

            (error_lines_end, error_span_end)
        });
        let (error_lines_end, error_span_end) = match maybe_end {
            Some((lines, span)) => (Some(lines), Some(span)),
            None => (None, None),
        };

        let lines_start = error_lines_start;
        let display_span = Span::new(error_span_start, error_span_end);
        let lines = Span::new(error_lines_start, error_lines_end)
            .slice(self.original_str())
            .to_owned();

        ParseError {
            inner,
            index_span,
            display_span,
            lines,
            lines_start,
        }
    }
}

impl Iterator for CharIter<'_> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.it.next()?;
        self.idx += c.len_utf8();
        Some(c)
    }
}
