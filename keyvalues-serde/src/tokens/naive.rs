// TODO(cosmic): replace this with a builder to incrementally create the vdf without going through
// this extra layer

use std::{borrow::Cow, iter::Peekable};

#[cfg(doc)]
use crate::tokens::Token;
use crate::{Error, Result};

use keyvalues_parser::{Key, Obj, Value, Vdf};
use serde_core::ser::Error as _;

// The conversion leverages all the `process_*` functions which pass off an owned iterator through
// all of them to deal with the borrow checker
pub(crate) fn vdf_from_naive_tokens(naive_tokens: &[NaiveToken]) -> Result<Vdf<'_>> {
    // Just some helper functions for munching through tokens
    fn process_key_values<'a, I>(
        mut tokens: Peekable<I>,
    ) -> Result<(Peekable<I>, Key<'a>, Vec<Value<'a>>)>
    where
        I: Iterator<Item = &'a NaiveToken>,
    {
        let key = match tokens.peek() {
            Some(NaiveToken::Str(s)) => {
                // Pop off the peeked token
                let _ = tokens.next().unwrap();
                Cow::from(s)
            }
            // Infer an empty key when we see an obj while expecting a key
            Some(NaiveToken::ObjBegin) => Cow::from(""),
            other => {
                // TODO: this shouldn't really be a custom error, but we need a better base
                // error type
                return Err(Error::custom(format!("Expected key, found: {other:?}")));
            }
        };

        let res = process_values(tokens)?;
        tokens = res.0;
        let values = res.1;

        Ok((tokens, key, values))
    }

    fn process_values<'a, I>(mut tokens: Peekable<I>) -> Result<(Peekable<I>, Vec<Value<'a>>)>
    where
        I: Iterator<Item = &'a NaiveToken>,
    {
        let pair = match tokens.next() {
            // A `Str` is a single value
            Some(NaiveToken::Str(s)) => (tokens, vec![Value::Str(Cow::from(s.clone()))]),
            Some(NaiveToken::ObjBegin) => {
                let (tokens, value) = process_obj(tokens)?;
                (tokens, vec![value])
            }
            // Sequences are a series of values that can't contain a sequence (vdf limitation)
            Some(NaiveToken::SeqBegin) => {
                let mut values = Vec::new();
                loop {
                    if let Some(NaiveToken::SeqEnd) = tokens.peek() {
                        // Pop off the marker
                        tokens.next();
                        break;
                    } else {
                        let res = process_non_seq_value(tokens)?;
                        tokens = res.0;
                        if let Some(val) = res.1 {
                            values.push(val);
                        }
                    }
                }

                (tokens, values)
            }
            // VDF represents `Null` as omitting the value
            Some(NaiveToken::Null) => (tokens, Vec::new()),
            _ => return Err(Error::ExpectedSomeValue),
        };

        Ok(pair)
    }

    fn process_non_seq_value<'a, I>(
        mut tokens: Peekable<I>,
    ) -> Result<(Peekable<I>, Option<Value<'a>>)>
    where
        I: Iterator<Item = &'a NaiveToken>,
    {
        let pair = match tokens.next() {
            Some(NaiveToken::Str(s)) => (tokens, Some(Value::Str(Cow::from(s)))),
            Some(NaiveToken::ObjBegin) => {
                let (tokens, value) = process_obj(tokens)?;
                (tokens, Some(value))
            }
            // VDF represents `Null` as omitting the value
            Some(NaiveToken::Null) => (tokens, None),
            _ => return Err(Error::ExpectedSomeNonSeqValue),
        };

        Ok(pair)
    }

    fn process_obj<'a, I>(mut tokens: Peekable<I>) -> Result<(Peekable<I>, Value<'a>)>
    where
        I: Iterator<Item = &'a NaiveToken>,
    {
        let mut obj = Obj::new();
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
                    let key = res.1;
                    let values = res.2;
                    obj.insert(key, values);
                }
                _ => return Err(Error::ExpectedObjectStart),
            }
        }

        Ok((tokens, Value::Obj(obj)))
    }

    let tokens = naive_tokens.iter().peekable();
    let (mut tokens, key, mut values) = process_key_values(tokens)?;

    if tokens.next().is_some() {
        return Err(Error::TrailingTokens);
    }
    let value = values.pop().ok_or_else(|| {
        Error::custom("Syntax error: Serialized multiple values when there should only be one")
    })?;
    Ok(Vdf::new(key, value))
}

/// A naive version of a [`Token`]
///
/// It is identical to [`Token`] except that
///
/// - It is owned instead of tied to a lifetime
/// - There is no `Key` where instead a key _should_ be a `Str`
/// - There is a `Null` variant that is needed to retain ordering to know what is a key or value
#[derive(Debug)]
pub enum NaiveToken {
    Str(String),
    ObjBegin,
    ObjEnd,
    SeqBegin,
    SeqEnd,
    Null,
}

impl NaiveToken {
    pub fn str<S: ToString>(s: S) -> Self {
        Self::Str(s.to_string())
    }
}
