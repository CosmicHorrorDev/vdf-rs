pub mod naive;

use std::{
    borrow::Cow,
    collections::BTreeMap,
    fmt,
    iter::Peekable,
    ops::{Deref, DerefMut},
};

use crate::core::{Key, Value, Vdf};

// I've been struggling to get serde to play nice with using a more complex internal structure in a
// `Deserializer`. I think the easiest solution I can come up with is to flatten out the `Vdf` into
// a stream of tokens that serde can consume. In this way the Deserializer can just work on
// munching through all the tokens instead of trying to mutate a more complex nested structure
// containing different types
/// A stream of tokens representing vdf. I think an example is the easiest way to understand the
/// structure so something like
/// ```no_test
/// "Outer Key" "Outer Value"
/// "Outer Key"
/// {
///     "Inner Key" "Inner Value"
/// }
/// ```
/// will be transformed into
/// ```no_test
/// Vdf({
///     "Outer Key": [
///         Str("Outer Value"),
///         Obj(
///             Vdf({
///                 "Inner Key": [
///                     Str("Inner Value")
///                 ]
///             })
///         )
///     ]
/// })
/// ```
/// which has the following token stream
/// ```no_test
/// TokenStream([
///     Key("Outer Key"),
///     SeqBegin,
///     Str("Outer Value"),
///     ObjBegin,
///     Key("Inner Key"),
///     Str("Inner Value"),
///     ObjEnd,
///     SeqEnd,
/// )]
/// ```
/// So in this way it's a linear sequence of keys and values where the value is either a str or
/// an object.
#[derive(Clone, Debug, PartialEq)]
pub struct TokenStream<'a>(pub Vec<Token<'a>>);

impl<'a> TokenStream<'a> {
    pub fn peek(&self) -> Option<&Token<'a>> {
        self.get(0)
    }

    pub fn peek_is_key(&self) -> bool {
        matches!(self.peek(), Some(Token::Key(_)))
    }

    pub fn peek_is_str(&self) -> bool {
        matches!(self.peek(), Some(Token::Str(_)))
    }

    pub fn peek_is_value(&self) -> bool {
        matches!(
            self.peek(),
            Some(Token::ObjBegin) | Some(Token::SeqBegin) | Some(Token::Str(_))
        )
    }

    // This is pretty bad for performance. If it's an issue we can flip the direction of the tokens
    // when we store it so that we can pop off the back instead
    pub fn next(&mut self) -> Option<Token<'a>> {
        if self.is_empty() {
            None
        } else {
            Some(self.remove(0))
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
        let mut inner = Vec::new();

        for (key, values) in vdf.0.into_iter() {
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

// TODO: have this return an error
fn process_key_value<'a, I>(
    mut tokens: Peekable<I>,
) -> Option<(Peekable<I>, (Key<'a>, Vec<Value<'a>>))>
where
    I: Iterator<Item = &'a Token<'a>>,
{
    if let Some(Token::Key(s)) = tokens.peek() {
        tokens.next();
        let key = s.clone();
        // TODO: don't unwrap here
        let res = process_value(tokens).unwrap();
        tokens = res.0;
        let value = res.1;
        Some((tokens, (key, value)))
    } else {
        todo!()
    }
}

fn process_value<'a, I>(mut tokens: Peekable<I>) -> Option<(Peekable<I>, Vec<Value<'a>>)>
where
    I: Iterator<Item = &'a Token<'a>>,
{
    match tokens.next() {
        Some(Token::Str(s)) => {
            let values = vec![Value::Str(s.clone())];
            Some((tokens, values))
        }
        Some(Token::ObjBegin) => {
            let res = process_obj(tokens).unwrap();
            Some((res.0, vec![res.1]))
        }
        Some(Token::SeqBegin) => {
            let mut values = Vec::new();
            loop {
                if let Some(Token::SeqEnd) = tokens.peek() {
                    tokens.next();
                    break;
                } else {
                    let res = process_non_seq_value(tokens).unwrap();
                    tokens = res.0;
                    values.push(res.1);
                }
            }

            Some((tokens, values))
        }
        _ => todo!(),
    }
}

fn process_non_seq_value<'a, I>(mut tokens: Peekable<I>) -> Option<(Peekable<I>, Value<'a>)>
where
    I: Iterator<Item = &'a Token<'a>>,
{
    match tokens.next() {
        Some(Token::Str(s)) => Some((tokens, Value::Str(s.clone()))),
        Some(Token::ObjBegin) => process_obj(tokens),
        _ => todo!(),
    }
}

fn process_obj<'a, I>(mut tokens: Peekable<I>) -> Option<(Peekable<I>, Value<'a>)>
where
    I: Iterator<Item = &'a Token<'a>>,
{
    let mut inner = BTreeMap::new();
    loop {
        match tokens.peek() {
            Some(Token::ObjEnd) => {
                tokens.next();
                break;
            }
            // An object is a series of key-value pairs
            Some(_) => {
                // ) -> Option<(Peekable<I>, (Key<'a>, Vec<Value<'a>>))>
                let res = process_key_value(tokens).unwrap();
                tokens = res.0;
                let key = res.1 .0;
                let values = res.1 .1;
                inner.insert(key, values);
            }
            _ => todo!(),
        }
    }

    Some((tokens, Value::Obj(Vdf(inner))))
}

// TODO: this should be fallible
// TODO: basically all of this is the same as `NaiveTokenStream` -> `TokenStream`. Can we switch
// the conversions to `NaiveTokenStream` -> `Vdf` and then `TokenStream` -> `NaiveTokenStream`
// instead which should cover all the possible options still and be much simpler instead
impl<'a> From<&'a TokenStream<'a>> for Vdf<'a> {
    fn from(token_stream: &'a TokenStream) -> Self {
        let mut inner = BTreeMap::new();
        let tokens = token_stream.iter().peekable();
        let (tokens, (key, value)) = process_key_value(tokens).unwrap();
        inner.insert(key, value);

        Self(inner)
    }
}

// TODO: Don't implement this directly since it's fallible
impl<'a> fmt::Display for TokenStream<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let vdf = Vdf::from(self);
        write!(f, "{}", vdf)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Token<'a> {
    Key(Cow<'a, str>),
    Str(Cow<'a, str>),
    ObjBegin,
    ObjEnd,
    SeqBegin,
    SeqEnd,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dev_checking() {
        let token_stream = TokenStream(vec![
            Token::Key(Cow::from("Outer Key")),
            Token::ObjBegin,
            Token::Key(Cow::from("Inner Key")),
            Token::Str(Cow::from("Inner Value")),
            Token::Key(Cow::from("Inner Seq")),
            Token::SeqBegin,
            Token::Str(Cow::from("Seq val")),
            Token::ObjBegin,
            Token::Key(Cow::from("Seq Val Key")),
            Token::Str(Cow::from("Seq Val Value")),
            Token::ObjEnd,
            Token::SeqEnd,
            Token::ObjEnd,
        ]);

        let vdf = Vdf::from(&token_stream);
        println!("{}", vdf);
        todo!();
    }

    #[test]
    fn token_stream() {
        let s = r#"
"Outer Key" "Outer Value"
"Outer Key"
{
    "Inner Key" "Inner Value"
}
        "#;
        let vdf = Vdf::parse(s).unwrap();
        let token_stream = TokenStream::from(vdf);
        assert_eq!(
            token_stream,
            TokenStream(vec![
                Token::Key(Cow::from("Outer Key")),
                Token::SeqBegin,
                Token::Str(Cow::from("Outer Value")),
                Token::ObjBegin,
                Token::Key(Cow::from("Inner Key")),
                Token::Str(Cow::from("Inner Value")),
                Token::ObjEnd,
                Token::SeqEnd,
            ])
        );
    }
}
