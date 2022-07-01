//! Internal conversion from the [`NaiveTokenStream`] to [`Vdf`]s
//!
//! WARN: This logic relies on the representation of [`NaiveTokenStream`]s infallibly matching the
//! layout of a [`Vdf`]. The implementation here must remain internal and the `Serializer` must
//! output to match this format.

use std::{
    borrow::Cow,
    iter::Peekable,
    ops::{Deref, DerefMut},
};

use keyvalues_parser::{Key, Obj, Value, Vdf};

#[cfg(doc)]
use crate::tokens::Token;

/// A stream of [`NaiveToken`]s that do not encode what is a key vs a value
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
/// associated value, all markers for multi-token structures make sense, and that there can't be a
/// sequence as a value in another sequence.
#[derive(Debug, Default)]
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
impl<'a> From<&'a NaiveTokenStream> for Vdf<'a> {
    fn from(naive_token_stream: &'a NaiveTokenStream) -> Self {
        // Just some helper functions for munching through tokens
        fn process_key_values<'a, I>(
            mut tokens: Peekable<I>,
        ) -> (Peekable<I>, Key<'a>, Vec<Value<'a>>)
        where
            I: Iterator<Item = &'a NaiveToken>,
        {
            if let Some(NaiveToken::Str(s)) = tokens.next() {
                let key = Cow::from(s);

                let res = process_values(tokens);
                tokens = res.0;
                let values = res.1;

                (tokens, key, values)
            } else {
                unreachable!("`Serializer` outputs valid `Vdf` structure");
            }
        }

        fn process_values<'a, I>(mut tokens: Peekable<I>) -> (Peekable<I>, Vec<Value<'a>>)
        where
            I: Iterator<Item = &'a NaiveToken>,
        {
            match tokens.next() {
                // A `Str` is a single value
                Some(NaiveToken::Str(s)) => (tokens, vec![Value::Str(Cow::from(s.clone()))]),
                Some(NaiveToken::ObjBegin) => {
                    let (tokens, value) = process_obj(tokens);
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
                            let res = process_non_seq_value(tokens);
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
                _ => unreachable!("`Serializer` outputs valid `Vdf` structure"),
            }
        }

        fn process_non_seq_value<'a, I>(mut tokens: Peekable<I>) -> (Peekable<I>, Option<Value<'a>>)
        where
            I: Iterator<Item = &'a NaiveToken>,
        {
            match tokens.next() {
                Some(NaiveToken::Str(s)) => (tokens, Some(Value::Str(Cow::from(s)))),
                Some(NaiveToken::ObjBegin) => {
                    let (tokens, value) = process_obj(tokens);
                    (tokens, Some(value))
                }
                // VDF represents `Null` as omitting the value
                Some(NaiveToken::Null) => (tokens, None),
                _ => unreachable!("`Serializer` outputs valid `Vdf` structure"),
            }
        }

        fn process_obj<'a, I>(mut tokens: Peekable<I>) -> (Peekable<I>, Value<'a>)
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
                        let res = process_key_values(tokens);
                        tokens = res.0;
                        let key = res.1;
                        let values = res.2;
                        obj.insert(key, values);
                    }
                    _ => unreachable!("`Serializer` outputs valid `Vdf` structure"),
                }
            }

            (tokens, Value::Obj(obj))
        }

        let tokens = naive_token_stream.iter().peekable();
        let (mut tokens, key, mut values) = process_key_values(tokens);

        assert!(
            tokens.next().is_none(),
            "`Serializer` outputs valid `Vdf` structure"
        );
        let value = values
            .pop()
            .expect("`Serializer` outputs valid `Vdf` structure");
        Self::new(key, value)
    }
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
