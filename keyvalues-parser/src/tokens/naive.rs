use crate::core::{Key, Value, Vdf};

use std::{
    borrow::Cow,
    collections::BTreeMap,
    convert::TryFrom,
    iter::Peekable,
    ops::{Deref, DerefMut},
};

use crate::error::{Error, Result, TokenContext};

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

fn process_key_values<'a, I>(
    mut tokens: Peekable<I>,
) -> Result<(Peekable<I>, (Key<'a>, Vec<Value<'a>>))>
where
    I: Iterator<Item = &'a NaiveToken>,
{
    match tokens.next() {
        Some(NaiveToken::Str(s)) => {
            let key = Cow::from(s);

            let res = process_values(tokens)?;
            tokens = res.0;
            let values = res.1;

            Ok((tokens, (key, values)))
        }
        Some(_) => Err(Error::from(TokenContext::ExpectedSomeVal)),
        None => Err(Error::from(TokenContext::EofWhileParsingKey)),
    }
}

fn process_values<'a, I>(mut tokens: Peekable<I>) -> Result<(Peekable<I>, Vec<Value<'a>>)>
where
    I: Iterator<Item = &'a NaiveToken>,
{
    match tokens.next() {
        // A `Str` is a single value
        Some(NaiveToken::Str(s)) => {
            let values = vec![Value::Str(Cow::from(s.clone()))];
            Ok((tokens, values))
        }
        Some(NaiveToken::ObjBegin) => {
            let res = process_obj(tokens)?;
            Ok((res.0, vec![res.1]))
        }
        // Sequences are a series of values that can't contain a sequence (vdf limitation)
        Some(NaiveToken::SeqBegin) => {
            let mut values = Vec::new();
            loop {
                // TODO: match for EOF here
                if let Some(NaiveToken::SeqEnd) = tokens.peek() {
                    // Pop off the marker
                    tokens.next();
                    break;
                } else {
                    let res = process_non_seq_value(tokens)?;
                    tokens = res.0;
                    values.push(res.1);
                }
            }

            Ok((tokens, values))
        }
        Some(_) => Err(Error::from(TokenContext::ExpectedSomeVal)),
        None => Err(Error::from(TokenContext::EofWhileParsingVal)),
    }
}

fn process_non_seq_value<'a, I>(mut tokens: Peekable<I>) -> Result<(Peekable<I>, Value<'a>)>
where
    I: Iterator<Item = &'a NaiveToken>,
{
    match tokens.next() {
        Some(NaiveToken::Str(s)) => Ok((tokens, Value::Str(Cow::from(s)))),
        Some(NaiveToken::ObjBegin) => process_obj(tokens),
        Some(_) => Err(Error::from(TokenContext::ExpectedNonSeqVal)),
        None => Err(Error::from(TokenContext::EofWhileParsingSeq)),
    }
}

fn process_obj<'a, I>(mut tokens: Peekable<I>) -> Result<(Peekable<I>, Value<'a>)>
where
    I: Iterator<Item = &'a NaiveToken>,
{
    let mut inner = BTreeMap::new();
    loop {
        match tokens.peek() {
            Some(NaiveToken::ObjEnd) => {
                tokens.next();
                break;
            }
            // An object is a series of key-value pairs
            Some(_) => {
                let res = process_key_values(tokens)?;
                tokens = res.0;
                let key = res.1 .0;
                let values = res.1 .1;
                inner.insert(key, values);
            }
            None => {
                return Err(Error::from(TokenContext::EofWhileParsingObj));
            }
        }
    }

    Ok((tokens, Value::Obj(Vdf(inner))))
}

impl<'a> TryFrom<&'a NaiveTokenStream> for Vdf<'a> {
    type Error = Error;

    fn try_from(naive_token_stream: &'a NaiveTokenStream) -> Result<Self> {
        let mut inner = BTreeMap::new();
        let tokens = naive_token_stream.iter().peekable();
        let (mut tokens, (key, values)) = process_key_values(tokens)?;
        inner.insert(key, values);

        if let None = tokens.next() {
            Ok(Self(inner))
        } else {
            Err(Error::from(TokenContext::TrailingTokens))
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
