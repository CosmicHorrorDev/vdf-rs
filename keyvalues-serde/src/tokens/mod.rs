// TODO: a lot of this can probably be slimmed down at this point
// TODO: implement a validate function
// TODO: make a note that this has invariants that must be upheld, so it is only exposed internally

mod naive;
#[cfg(test)]
mod tests;

use keyvalues_parser::{Obj, Value, Vdf};

use std::{
    borrow::Cow,
    ops::{Deref, DerefMut},
};

pub use crate::tokens::naive::{NaiveToken, NaiveTokenStream};

// I've been struggling to get serde to play nice with using a more complex internal structure in a
// `Deserializer`. I think the easiest solution I can come up with is to flatten out the `Vdf` into
// a stream of tokens that serde can consume. In this way the Deserializer can just work on
// munching through all the tokens instead of trying to mutate a more complex nested structure
// containing different types
/// A stream of [`Token`]s representing a [`Vdf`]
///
/// I think an example is the easiest way to understand the structure so something like
///
/// ```vdf
/// "Outer Key"
/// {
///     "Inner Key" "Inner Value"
///     "Inner Key"
///     {
///     }
/// }
/// ```
///
/// will be transformed into
///
/// ```ron
/// Vdf(
///     key: "Outer Key",
///     value: Obj({
///         "Inner Key": [
///             Str("Inner Value"),
///             Obj({})
///         ]
///     })
/// )
/// ```
///
/// which has the following token stream
///
/// ```ron
/// TokenStream([
///     Key("Outer Key"),
///     ObjBegin,
///     Key("Inner Key"),
///     SeqBegin,
///     Str("Inner Value"),
///     ObjBegin,
///     ObjEnd,
///     SeqEnd,
///     ObjEnd,
/// )]
/// ```
///
/// So in this way it's a linear sequence of keys and values where the value is either a str or an
/// object.
#[derive(Debug, PartialEq, Eq)]
pub struct TokenStream<'a>(pub Vec<Token<'a>>);

impl<'a> Deref for TokenStream<'a> {
    type Target = Vec<Token<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for TokenStream<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> From<Vdf<'a>> for TokenStream<'a> {
    fn from(vdf: Vdf<'a>) -> Self {
        let Vdf { key, value } = vdf;

        let mut inner = vec![Token::Key(key)];
        inner.extend(TokenStream::from(value).0);

        Self(inner)
    }
}

impl<'a> From<Value<'a>> for TokenStream<'a> {
    fn from(value: Value<'a>) -> Self {
        let mut inner = Vec::new();

        match value {
            Value::Str(s) => inner.push(Token::Str(s)),
            Value::Obj(obj) => {
                inner.push(Token::ObjBegin);
                inner.extend(Self::from(obj).0);
                inner.push(Token::ObjEnd);
            }
        }

        Self(inner)
    }
}

impl<'a> From<Obj<'a>> for TokenStream<'a> {
    fn from(obj: Obj<'a>) -> Self {
        let mut inner = Vec::new();

        for (key, values) in obj.into_inner().into_iter() {
            inner.push(Token::Key(key));

            // For ease of use a sequence is only marked when len != 1
            let num_values = values.len();
            if num_values != 1 {
                inner.push(Token::SeqBegin);
            }

            for value in values {
                inner.extend(TokenStream::from(value).0);
            }

            if num_values != 1 {
                inner.push(Token::SeqEnd);
            }
        }

        Self(inner)
    }
}

/// A single VDF token
#[derive(Debug, PartialEq, Eq)]
pub enum Token<'a> {
    Key(Cow<'a, str>),
    Str(Cow<'a, str>),
    ObjBegin,
    ObjEnd,
    SeqBegin,
    SeqEnd,
}
