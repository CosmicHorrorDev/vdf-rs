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
    push_tokens_from_value(&mut tokens, value);
    tokens
}

fn push_tokens_from_value<'text>(tokens: &mut Vec<Token<'text>>, value: Value<'text>) {
    match value {
        Value::Str(s) => tokens.push(Token::Str(s)),
        Value::Obj(obj) => {
            tokens.push(Token::ObjBegin);
            push_tokens_from_obj(tokens, obj);
            tokens.push(Token::ObjEnd);
        }
    }
}

fn push_tokens_from_obj<'text>(tokens: &mut Vec<Token<'text>>, obj: Obj<'text>) {
    for (key, values) in obj.into_inner().into_iter() {
        tokens.push(Token::Key(key));

        // For ease of use a sequence is only marked when len != 1
        let num_values = values.len();
        if num_values != 1 {
            tokens.push(Token::SeqBegin);
        }

        for value in values {
            push_tokens_from_value(tokens, value);
        }

        if num_values != 1 {
            tokens.push(Token::SeqEnd);
        }
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
