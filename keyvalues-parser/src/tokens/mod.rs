pub mod naive;
#[cfg(test)]
mod tests;

use std::{
    borrow::Cow,
    ops::{Deref, DerefMut},
};

use crate::core::{Obj, Value, Vdf};

// I've been struggling to get serde to play nice with using a more complex internal structure in a
// `Deserializer`. I think the easiest solution I can come up with is to flatten out the `Vdf` into
// a stream of tokens that serde can consume. In this way the Deserializer can just work on
// munching through all the tokens instead of trying to mutate a more complex nested structure
// containing different types
/// A stream of tokens representing vdf. I think an example is the easiest way to understand the
/// structure so something like
/// ```no_test
/// "Outer Key"
/// {
///     "Inner Key" "Inner Value"
///     "Inner Key"
///     {
///     }
/// }
/// ```
/// will be transformed into
/// ```no_test
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
/// which has the following token stream
/// ```no_test
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
/// So in this way it's a linear sequence of keys and values where the value is either a str or
/// an object.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TokenStream<'a>(pub Vec<Token<'a>>);

impl<'a> TokenStream<'a> {
    pub fn peek(&self) -> Option<&Token<'a>> {
        self.get(0)
    }

    pub fn peek_is_key(&self) -> bool {
        match self.peek() {
            Some(Token::Key(_)) => true,
            _ => false,
        }
    }

    pub fn peek_is_str(&self) -> bool {
        match self.peek() {
            Some(Token::Str(_)) => true,
            _ => false,
        }
    }

    pub fn peek_is_value(&self) -> bool {
        match self.peek() {
            Some(Token::ObjBegin) | Some(Token::SeqBegin) | Some(Token::Str(_)) => true,
            _ => false,
        }
    }

    pub fn next_key(&mut self) -> Option<Cow<'a, str>> {
        if self.peek_is_key() {
            if let Some(Token::Key(s)) = self.next() {
                Some(s)
            } else {
                unreachable!("Key was peeked");
            }
        } else {
            None
        }
    }

    pub fn next_str(&mut self) -> Option<Cow<'a, str>> {
        if self.peek_is_str() {
            if let Some(Token::Str(s)) = self.next() {
                Some(s)
            } else {
                unreachable!("Str was peeked");
            }
        } else {
            None
        }
    }

    pub fn next_key_or_str(&mut self) -> Option<Cow<'a, str>> {
        self.next_key().or_else(|| self.next_str())
    }
}

impl<'a> Iterator for TokenStream<'a> {
    type Item = Token<'a>;

    // This is pretty bad for performance. If it's an issue we can flip the direction of the tokens
    // when we store it so that we can pop off the back instead
    fn next(&mut self) -> Option<Self::Item> {
        if self.is_empty() {
            None
        } else {
            Some(self.remove(0))
        }
    }
}

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
            Value::Str(s) => {
                inner.push(Token::Str(s));
            }
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

        for (key, values) in obj.into_iter() {
            inner.push(Token::Key(key));

            // For ease of use a sequence is only marked for keys that have
            // more than one values (zero shouldn't be allowed)
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

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Token<'a> {
    Key(Cow<'a, str>),
    Str(Cow<'a, str>),
    ObjBegin,
    ObjEnd,
    SeqBegin,
    SeqEnd,
}
