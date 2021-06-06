use std::{
    borrow::Cow,
    convert::TryFrom,
    iter::Peekable,
    ops::{Deref, DerefMut},
};

use crate::{
    error::{Error, Result, TokenContext},
    tokens::{Token, TokenStream},
    Key, Obj, Value, Vdf,
};

/// A stream of [`NaiveToken`][NaiveToken]s that do not encode what is a key vs a value
///
/// This is primarily provided to simplify serialization so that a serializer can emit a naive
/// token stream that can later be used to create a VDF. This is due to the following reasons
///
/// 1. The tokens can be owned values since there is no lifetime to tie the borrowed values to.
/// 2. There isn't context about what are keys vs. values
/// 3. Validation can be done in a separate step
///
/// From there a `NaiveTokenStream` can be converted to a `Vdf` where the position of the keys is
/// inferred from the general structure. This also performs validation that all keys have an
/// associated value, all markers for mutli-token structures make sense, and that there can't be a
/// sequence as a value in another sequence.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

// The conversion from `NaiveTokenStream` to `Vdf` leverages all the `process_*` functions which
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

impl<'a> From<TokenStream<'a>> for NaiveTokenStream {
    fn from(token_stream: TokenStream<'a>) -> Self {
        let inner = token_stream.map(NaiveToken::from).collect();
        Self(inner)
    }
}

/// A naive version of a [`Token`][crate::tokens::Token]
///
/// It is identical to [`Token`][crate::tokens::Token] except that
///
/// - It is owned instead of tied to a lifetime
/// - There is no `Key` where instead a key _should_ be a `Str`
/// - There is a `Null` variant that is needed to retain ordering to know what is a key or value
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

impl<'a> From<Token<'a>> for NaiveToken {
    fn from(token: Token<'a>) -> Self {
        match token {
            Token::Key(s) | Token::Str(s) => Self::Str(s.into_owned()),
            Token::ObjBegin => Self::ObjBegin,
            Token::ObjEnd => Self::ObjEnd,
            Token::SeqBegin => Self::SeqBegin,
            Token::SeqEnd => Self::SeqEnd,
        }
    }
}
