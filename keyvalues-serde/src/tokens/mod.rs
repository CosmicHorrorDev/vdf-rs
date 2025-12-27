// TODO: replace with some kind of iterator that decomposes the original structure instead of using
// an intermediate layer

pub(crate) mod naive;
#[cfg(test)]
mod tests;

use keyvalues_parser::{Obj, Value, Vdf};

use std::borrow::Cow;

pub use crate::tokens::naive::NaiveToken;

pub(crate) fn tokens_from_vdf(vdf: Vdf<'_>) -> Vec<Token<'_>> {
    let Vdf { key, value } = vdf;

    let mut tokens = vec![Token::Key(key)];
    tokens.extend(tokens_from_value(value));
    tokens
}

// TODO: pass through a `&mut Vec<_>` instead of allocating new ones
fn tokens_from_value(value: Value<'_>) -> Vec<Token<'_>> {
    let mut tokens = Vec::new();

    match value {
        Value::Str(s) => tokens.push(Token::Str(s)),
        Value::Obj(obj) => {
            tokens.push(Token::ObjBegin);
            tokens.extend(tokens_from_obj(obj));
            tokens.push(Token::ObjEnd);
        }
    }

    tokens
}

fn tokens_from_obj(obj: Obj<'_>) -> Vec<Token<'_>> {
    let mut tokens = Vec::new();

    for (key, values) in obj.into_inner().into_iter() {
        tokens.push(Token::Key(key));

        // For ease of use a sequence is only marked when len != 1
        let num_values = values.len();
        if num_values != 1 {
            tokens.push(Token::SeqBegin);
        }

        for value in values {
            tokens.extend(tokens_from_value(value));
        }

        if num_values != 1 {
            tokens.push(Token::SeqEnd);
        }
    }

    tokens
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
