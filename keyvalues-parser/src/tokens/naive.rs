use std::{
    borrow::Cow,
    convert::TryFrom,
    iter::Peekable,
    ops::{Deref, DerefMut},
};

use crate::{
    core::{Key, Obj, Value, Vdf},
    error::{Error, Result, TokenContext},
};

// Used to easily deal with serializing VDF. The serializer spits out a `NaiveTokenStream` that can
// then be converted to a `Vdf` object which can handle all the rendering functions.
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

// The conversion from `NaiveTokenStream` to `Vdf` leverages all the `proccess_*` functions which
// pass off an owned iterator through all of them to deal with the borrow checker
impl<'a> TryFrom<&'a NaiveTokenStream> for Vdf<'a> {
    type Error = Error;

    fn try_from(naive_token_stream: &'a NaiveTokenStream) -> Result<Self> {
        // Just some helper functions for munching through tokens
        fn process_key_values<'a, I>(
            mut tokens: Peekable<I>,
        ) -> Result<(Peekable<I>, Key<'a>, Vec<Value<'a>>)>
        where
            I: Iterator<Item = &'a NaiveToken>,
        {
            match tokens.next() {
                Some(NaiveToken::Str(s)) => {
                    let key = Cow::from(s);

                    let res = process_values(tokens)?;
                    tokens = res.0;
                    let values = res.1;

                    Ok((tokens, key, values))
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
                            if let Some(val) = res.1 {
                                values.push(val);
                            }
                        }
                    }

                    Ok((tokens, values))
                }
                // VDF represents `Null` as omitting the value
                Some(NaiveToken::Null) => Ok((tokens, Vec::new())),
                Some(_) => Err(Error::from(TokenContext::ExpectedSomeVal)),
                None => Err(Error::from(TokenContext::EofWhileParsingVal)),
            }
        }

        fn process_non_seq_value<'a, I>(
            mut tokens: Peekable<I>,
        ) -> Result<(Peekable<I>, Option<Value<'a>>)>
        where
            I: Iterator<Item = &'a NaiveToken>,
        {
            match tokens.next() {
                Some(NaiveToken::Str(s)) => Ok((tokens, Some(Value::Str(Cow::from(s))))),
                Some(NaiveToken::ObjBegin) => {
                    let res = process_obj(tokens)?;
                    Ok((res.0, Some(res.1)))
                }
                // VDF represents `Null` as omitting the value
                Some(NaiveToken::Null) => Ok((tokens, None)),
                Some(_) => Err(Error::from(TokenContext::ExpectedNonSeqVal)),
                None => Err(Error::from(TokenContext::EofWhileParsingSeq)),
            }
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
                    None => {
                        return Err(Error::from(TokenContext::EofWhileParsingObj));
                    }
                }
            }

            Ok((tokens, Value::Obj(obj)))
        }

        let tokens = naive_token_stream.iter().peekable();
        let (mut tokens, key, mut values) = process_key_values(tokens)?;

        if tokens.next().is_none() {
            match values.len() {
                0 => Err(Error::from(TokenContext::ExpectedNonSeqVal)),
                1 => Ok(Self::new(key, values.pop().expect("Length was checked"))),
                _two_or_more => Err(Error::from(TokenContext::TrailingTokens)),
            }
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
    Null,
}

impl NaiveToken {
    pub fn str<S: ToString>(s: S) -> Self {
        Self::Str(s.to_string())
    }
}
