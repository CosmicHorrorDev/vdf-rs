use keyvalues_parser::tokens::{Token, TokenStream};

use std::{
    borrow::Cow,
    convert::TryFrom,
    iter::Peekable,
    ops::{Deref, DerefMut},
};

use crate::error::{Error, Result};

/// `NaiveTokenStream` is the token-stream that is emitted by the serialization process. This is
/// done so that
///
/// 1. The tokens can be owned values since there is no lifetime to tie the borrowed values to.
/// 2. There isn't context about what are keys vs. values
/// 3. Validation can be done in a separate step
///
/// From there a `NaiveTokenStream` can be converted to a `TokenStream` where the position of the
/// keys is inferred from the general structure. This also performs validation that all keys have
/// an associated value, all markers for mutli-token structures make sense, and that there can't
/// be a sequence as a value in another sequence.
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
    match tokens.next() {
        Some(NaiveToken::Str(s)) => {
            processed.push(Token::Key(Cow::from(s)));
            process_value(tokens, processed)
        }
        Some(_) => Err(Error::ExpectedSomeValue),
        None => Err(Error::EofWhileParsingKey),
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
                // TODO: match for EOF here
                if let Some(NaiveToken::SeqEnd) = tokens.peek() {
                    tokens.next();
                    processed.push(Token::SeqEnd);
                    break;
                } else {
                    let res = process_non_seq_value(tokens, processed)?;
                    tokens = res.0;
                    processed = res.1;
                }
            }

            Ok((tokens, processed))
        }
        Some(NaiveToken::ObjBegin) => process_obj(tokens, processed),
        Some(_) => Err(Error::ExpectedSomeValue),
        None => Err(Error::EofWhileParsingValue),
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
        Some(_) => Err(Error::ExpectedSomeNonSeqValue),
        None => Err(Error::EofWhileParsingValue),
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
        match tokens.peek() {
            Some(NaiveToken::ObjEnd) => {
                tokens.next();
                processed.push(Token::ObjEnd);
                break;
            }
            // An object is a series of key-value pairs
            Some(_) => {
                let res = process_key_value(tokens, processed)?;
                tokens = res.0;
                processed = res.1;
            }
            None => {
                return Err(Error::EofWhileParsingObject);
            }
        }
    }

    Ok((tokens, processed))
}

impl<'a> TryFrom<&'a NaiveTokenStream> for TokenStream<'a> {
    type Error = Error;

    fn try_from(naive_token_stream: &'a NaiveTokenStream) -> Result<Self> {
        let (mut unprocessed_tokens, tokens) = process_key_value(
            naive_token_stream.iter().peekable(),
            Vec::with_capacity(naive_token_stream.len()),
        )?;

        if let None = unprocessed_tokens.next() {
            Ok(TokenStream(tokens))
        } else {
            Err(Error::TrailingTokens)
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
