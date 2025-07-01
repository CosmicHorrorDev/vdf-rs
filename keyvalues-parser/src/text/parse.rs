use std::{borrow::Cow, str::Chars};

use crate::{
    error::{Error, Result},
    Key, Obj, PartialVdf, Value, Vdf,
};

// TODO: rename `PartialVdf` to `TopLevelVdf` and have it hold a `Vdf` instead of flattening it out

impl<'a> PartialVdf<'a> {
    /// Attempts to parse VDF text to a [`PartialVdf`]
    pub fn parse(s: &'a str) -> Result<Self> {
        escaped_parse(s)
    }

    pub fn parse_raw(s: &'a str) -> Result<Self> {
        raw_parse(s)
    }
}

impl<'a> Vdf<'a> {
    /// Attempts to parse VDF text to a [`Vdf`]
    pub fn parse(s: &'a str) -> Result<Self> {
        Ok(Vdf::from(PartialVdf::parse(s)?))
    }

    pub fn parse_raw(s: &'a str) -> Result<Self> {
        Ok(Vdf::from(PartialVdf::parse_raw(s)?))
    }
}

pub fn raw_parse(s: &str) -> Result<PartialVdf<'_>> {
    parse_(s, false)
}

pub fn escaped_parse(s: &str) -> Result<PartialVdf<'_>> {
    parse_(s, true)
}

pub fn parse_(s: &str, escape_chars: bool) -> Result<PartialVdf<'_>> {
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
        Err(Error::Todo) // Lingering bytes
    } else {
        Ok(PartialVdf { bases, key, value })
    }
}

fn parse_macros<'text>(chars: &mut CharIter<'text>) -> Result<Vec<&'text str>> {
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
) -> Result<Option<()>> {
    if !chars.next_n_if_eq(['#', 'b', 'a', 's', 'e']) {
        return Ok(None);
    }

    if !eat_whitespace(chars) {
        return Err(Error::Todo);
    }

    let macro_ = parse_quoted_raw_or_unquoted_string(chars)?;
    macros.push(macro_);

    eat_comments_and_whitespace(chars)?;

    if eat_newlines(chars) {
        Ok(Some(()))
    } else {
        Err(Error::Todo)
    }
}

fn parse_quoted_raw_or_unquoted_string<'text>(chars: &mut CharIter<'text>) -> Result<&'text str> {
    if chars.peek() == Some('"') {
        parse_quoted_raw_string(chars)
    } else {
        parse_unquoted_string(chars)
    }
}

// TODO: error on `\r` or `\n` in quoted str (wait no i think that's valid)
fn parse_quoted_raw_string<'text>(chars: &mut CharIter<'text>) -> Result<&'text str> {
    assert!(chars.ensure_next('"'));
    let start_idx = chars.index();
    while chars.next().ok_or(Error::Todo)? != '"' {}
    let end_idx = chars.index() - '"'.len_utf8();
    Ok(&chars.original_str()[start_idx..end_idx])
}

fn parse_unquoted_string<'text>(chars: &mut CharIter<'text>) -> Result<&'text str> {
    let start_idx = chars.index();

    match chars.next().ok_or(Error::Todo)? {
        '"' | '{' | '}' | ' ' | '\t' | '\r' | '\n' => return Err(Error::Todo),
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

fn parse_pair<'text>(chars: &mut CharIter<'text>, escape_chars: bool) -> Result<Vdf<'text>> {
    let key = parse_string(chars, escape_chars)?;
    eat_comments_whitespace_and_newlines(chars)?;
    let value = parse_value(chars, escape_chars)?;
    Ok(Vdf { key, value })
}

fn parse_string<'text>(chars: &mut CharIter<'text>, escape_chars: bool) -> Result<Key<'text>> {
    if chars.peek() == Some('"') {
        parse_quoted_string(chars, escape_chars)
    } else {
        let s = parse_unquoted_string(chars)?;
        Ok(Cow::Borrowed(s))
    }
}

fn parse_quoted_string<'text>(
    chars: &mut CharIter<'text>,
    escape_chars: bool,
) -> Result<Key<'text>> {
    assert!(chars.ensure_next('"'));

    let start_idx = chars.index();
    loop {
        let peeked = chars.peek().ok_or(Error::Todo)?;
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
    let key = if chars.peek().ok_or(Error::Todo)? == '"' {
        assert!(chars.ensure_next('"'));
        Cow::Borrowed(text_chunk)
    } else {
        assert!(chars.peek().unwrap() == '\\');
        let mut escaped = text_chunk.to_owned();
        loop {
            let ch = chars.next().ok_or(Error::Todo)?;
            match ch {
                '"' => break Cow::Owned(escaped),
                '\\' => match chars.next().ok_or(Error::Todo)? {
                    'n' => escaped.push('\n'),
                    'r' => escaped.push('\r'),
                    't' => escaped.push('\t'),
                    '\\' => escaped.push('\\'),
                    '\"' => escaped.push('\"'),
                    _ => return Err(Error::Todo),
                },
                regular => escaped.push(regular),
            }
        }
    };

    Ok(key)
}

fn parse_value<'text>(chars: &mut CharIter<'text>, escape_chars: bool) -> Result<Value<'text>> {
    let value = match chars.peek() {
        Some('{') => {
            let obj = parse_obj(chars, escape_chars)?;
            Value::Obj(obj)
        }
        _ => {
            let s = parse_string(chars, escape_chars)?;
            Value::Str(s)
        }
    };
    Ok(value)
}

fn parse_obj<'text>(chars: &mut CharIter<'text>, escape_chars: bool) -> Result<Obj<'text>> {
    assert!(chars.ensure_next('{'));
    eat_comments_whitespace_and_newlines(chars)?;

    let mut obj = Obj::new();

    while chars.peek().ok_or(Error::Todo)? != '}' {
        let Vdf { key, value } = parse_pair(chars, escape_chars)?;
        obj.0.entry(key).or_default().push(value);

        eat_comments_whitespace_and_newlines(chars)?;
    }
    assert!(chars.ensure_next('}'));

    Ok(obj)
}

fn eat_comments_whitespace_and_newlines(chars: &mut CharIter<'_>) -> Result<bool> {
    let mut ate = false;
    while eat_whitespace_and_newlines(chars) || eat_comments(chars)? {
        ate = true;
    }

    Ok(ate)
}

fn eat_comments_and_whitespace(chars: &mut CharIter<'_>) -> Result<bool> {
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
fn eat_comments(chars: &mut CharIter<'_>) -> Result<bool> {
    if !chars.next_n_if_eq(['/', '/']) {
        Ok(false)
    } else {
        loop {
            match chars.peek() {
                Some('\r') => {
                    chars.unwrap_next();
                    match chars.next() {
                        Some('\n') => break,
                        _ => return Err(Error::Todo),
                    }
                }
                None | Some('\n') => break,
                // Various control characters
                Some('\u{00}'..='\u{08}' | '\u{0A}'..='\u{1F}' | '\u{7F}') => {
                    return Err(Error::Todo)
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
}

impl Iterator for CharIter<'_> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.it.next()?;
        self.idx += c.len_utf8();
        Some(c)
    }
}
