use keyvalues_parser::tokens::{Token, TokenStream};

use std::{
    borrow::Cow,
    convert::TryFrom,
    iter::Peekable,
    ops::{Deref, DerefMut},
};

use crate::error::{Error, Result};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct NaiveTokenStream(pub Vec<NaiveToken>);

impl Deref for NaiveTokenStream {
    type Target = Vec<NaiveToken>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for NaiveTokenStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn process_key_value<'a, I>(
    mut tokens: Peekable<I>,
    mut processed: Vec<Token<'a>>,
) -> Result<(Peekable<I>, Vec<Token<'a>>)>
where
    I: Iterator<Item = &'a NaiveToken>,
{
    if let Some(NaiveToken::Str(s)) = tokens.next() {
        processed.push(Token::Key(Cow::from(s)));
        process_value(tokens, processed)
    } else {
        Err(Error::InvalidTokenStream)
    }
}

fn process_value<'a, I>(
    mut tokens: Peekable<I>,
    mut processed: Vec<Token<'a>>,
) -> Result<(Peekable<I>, Vec<Token<'a>>)>
where
    I: Iterator<Item = &'a NaiveToken>,
{
    match tokens.next() {
        // A `Str` is a single value
        Some(NaiveToken::Str(s)) => {
            processed.push(Token::Str(Cow::from(s)));
            Ok((tokens, processed))
        }
        // Sequences are a series of values that can't contain a sequence (vdf limitation)
        Some(NaiveToken::SeqBegin) => {
            processed.push(Token::SeqBegin);
            loop {
                if let Some(NaiveToken::SeqEnd) = tokens.peek() {
                    tokens.next();
                    processed.push(Token::SeqEnd);
                    break;
                } else {
                    // Nested sequences aren't allowed
                    let res = process_non_seq_value(tokens, processed)?;
                    tokens = res.0;
                    processed = res.1;
                }
            }

            Ok((tokens, processed))
        }
        // An object is a series of key-value pairs
        Some(NaiveToken::ObjBegin) => process_obj(tokens, processed),
        _ => Err(Error::InvalidTokenStream),
    }
}

fn process_non_seq_value<'a, I>(
    mut tokens: Peekable<I>,
    mut processed: Vec<Token<'a>>,
) -> Result<(Peekable<I>, Vec<Token<'a>>)>
where
    I: Iterator<Item = &'a NaiveToken>,
{
    match tokens.next() {
        Some(NaiveToken::Str(s)) => {
            processed.push(Token::Str(Cow::from(s)));
            Ok((tokens, processed))
        }
        Some(NaiveToken::ObjBegin) => process_obj(tokens, processed),
        _ => Err(Error::InvalidTokenStream),
    }
}

fn process_obj<'a, I>(
    mut tokens: Peekable<I>,
    mut processed: Vec<Token<'a>>,
) -> Result<(Peekable<I>, Vec<Token<'a>>)>
where
    I: Iterator<Item = &'a NaiveToken>,
{
    processed.push(Token::ObjBegin);
    loop {
        if let Some(NaiveToken::ObjEnd) = tokens.peek() {
            tokens.next();
            processed.push(Token::ObjEnd);
            break;
        } else {
            let res = process_key_value(tokens, processed)?;
            tokens = res.0;
            processed = res.1;
        }
    }

    Ok((tokens, processed))
}

impl<'a> TryFrom<&'a NaiveTokenStream> for TokenStream<'a> {
    type Error = Error;

    // This conversion isn't done recursively so multi-token structures like objects and sequences
    // are tracked using a `context_stack`
    fn try_from(naive_token_stream: &'a NaiveTokenStream) -> Result<Self> {
        let (mut unprocessed_tokens, tokens) = process_key_value(
            naive_token_stream.iter().peekable(),
            Vec::with_capacity(naive_token_stream.len()),
        )?;

        if let None = unprocessed_tokens.next() {
            Ok(TokenStream(tokens))
        } else {
            Err(Error::InvalidTokenStream)
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum NaiveToken {
    Str(String),
    ObjBegin,
    ObjEnd,
    SeqBegin,
    SeqEnd,
}

impl NaiveToken {
    pub fn str<S: ToString>(s: S) -> Self {
        Self::Str(s.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        let naive_token_stream = NaiveTokenStream(vec![
            NaiveToken::str("outer"),
            NaiveToken::ObjBegin,
            NaiveToken::str("sequence start"),
            NaiveToken::SeqBegin,
            NaiveToken::ObjBegin,
            NaiveToken::str("inner key"),
            NaiveToken::str("inner val"),
            NaiveToken::ObjEnd,
            NaiveToken::str("some other inner val"),
            NaiveToken::SeqEnd,
            NaiveToken::ObjEnd,
        ]);

        assert_eq!(
            TokenStream::try_from(&naive_token_stream),
            Ok(TokenStream(vec![
                Token::Key(Cow::from("outer")),
                Token::ObjBegin,
                Token::Key(Cow::from("sequence start")),
                Token::SeqBegin,
                Token::ObjBegin,
                Token::Key(Cow::from("inner key")),
                Token::Str(Cow::from("inner val")),
                Token::ObjEnd,
                Token::Str(Cow::from("some other inner val")),
                Token::SeqEnd,
                Token::ObjEnd,
            ]))
        );
    }

    #[test]
    fn invalid_nested_seq() {
        let naive_token_stream = NaiveTokenStream(vec![
            NaiveToken::str("outer"),
            NaiveToken::ObjBegin,
            NaiveToken::str("nested sequence"),
            NaiveToken::SeqBegin,
            NaiveToken::str("the calm before the storm"),
            NaiveToken::SeqBegin,
            NaiveToken::SeqEnd,
            NaiveToken::SeqEnd,
            NaiveToken::ObjEnd,
        ]);

        assert!(TokenStream::try_from(&naive_token_stream).is_err());
    }

    #[test]
    fn invalid_obj_key() {
        let naive_token_stream = NaiveTokenStream(vec![
            NaiveToken::str("outer"),
            NaiveToken::ObjBegin,
            NaiveToken::ObjBegin,
            NaiveToken::ObjEnd,
            NaiveToken::ObjEnd,
        ]);

        assert!(TokenStream::try_from(&naive_token_stream).is_err());
    }

    #[test]
    fn invalid_seq_key() {
        let naive_token_stream = NaiveTokenStream(vec![
            NaiveToken::str("outer"),
            NaiveToken::ObjBegin,
            NaiveToken::SeqBegin,
            NaiveToken::SeqEnd,
            NaiveToken::ObjEnd,
        ]);

        assert!(TokenStream::try_from(&naive_token_stream).is_err());
    }
}
