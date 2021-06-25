// Due to Pest generating variants that are all uppercase
#![allow(renamed_and_removed_lints)]
#![allow(clippy::unknown_clippy_lints)]
#![allow(clippy::upper_case_acronyms)]

use pest::{iterators::Pair as PestPair, Parser};
use pest_derive::Parser;

use std::{borrow::Cow, convert::TryFrom};

use crate::{
    error::{Error, Result},
    Obj, Value, Vdf,
};

#[derive(Parser)]
#[grammar = "text/grammar.pest"]
struct VdfParser;

pub(crate) type PestError = pest::error::Error<Rule>;

fn parse_pair(grammar_pair: PestPair<'_, Rule>) -> (Cow<'_, str>, Value<'_>) {
    // Structure: pair
    //            \ key   <- Desired
    //            \ value <- Desired
    if let Rule::pair = grammar_pair.as_rule() {
        // Parse out the key and value
        let mut grammar_pair_innards = grammar_pair.into_inner();
        let grammar_string = grammar_pair_innards.next().unwrap();
        let key = parse_string(grammar_string);

        let grammar_value = grammar_pair_innards.next().unwrap();
        let value = Value::from(grammar_value);

        (key, value)
    } else {
        unreachable!("Prevented by grammar");
    }
}

fn parse_string(grammar_string: PestPair<'_, Rule>) -> Cow<'_, str> {
    match grammar_string.as_rule() {
        // Structure: quoted_string
        //            \ "
        //            \ quoted_inner <- Desired
        //            \ "
        Rule::quoted_string => {
            let quoted_inner = grammar_string.into_inner().next().unwrap();
            parse_escaped_string(quoted_inner)
        }
        // Structure: unquoted_string <- Desired
        Rule::unquoted_string => {
            let s = grammar_string.as_str();
            Cow::from(s)
        }
        _ => unreachable!("Prevented by grammar"),
    }
}

// Note: there can be a slight performance win here by having the grammar skip capturing
// quoted_inner and instead just slice off the starting and ending '"', but I'm going to pass since
// it seems like a hack for a ~4% improvement
fn parse_escaped_string(inner: PestPair<'_, Rule>) -> Cow<'_, str> {
    let s = inner.as_str();

    if s.contains('\\') {
        // Escaped version won't be quite as long, but it will likely be close
        let mut escaped = String::with_capacity(s.len());
        let mut it = s.chars();

        while let Some(ch) = it.next() {
            if ch == '\\' {
                // Character is escaped so check the next character to figure out the full
                // character
                match it.next() {
                    Some('n') => escaped.push('\n'),
                    Some('r') => escaped.push('\r'),
                    Some('t') => escaped.push('\t'),
                    Some('\\') => escaped.push('\\'),
                    Some('\"') => escaped.push('\"'),
                    _ => unreachable!("Prevented by grammar"),
                }
            } else {
                escaped.push(ch)
            }
        }

        Cow::from(escaped)
    } else {
        Cow::from(s)
    }
}

impl<'a> Vdf<'a> {
    /// TODO
    pub fn parse(s: &'a str) -> Result<Self> {
        Self::try_from(s)
    }
}

impl<'a> TryFrom<&'a str> for Vdf<'a> {
    type Error = Error;

    fn try_from(s: &'a str) -> Result<Self> {
        // Structure: vdf
        //            \ SOI
        //            \ pair <- Desired
        //            \ EOI
        let unparsed = VdfParser::parse(Rule::vdf, s)?.next().unwrap();
        Ok(Self::from(unparsed))
    }
}

impl<'a> From<PestPair<'a, Rule>> for Vdf<'a> {
    fn from(grammar_pair: PestPair<'a, Rule>) -> Self {
        let (key, value) = parse_pair(grammar_pair);
        Self::new(key, value)
    }
}

impl<'a> From<PestPair<'a, Rule>> for Value<'a> {
    fn from(grammar_value: PestPair<'a, Rule>) -> Self {
        // Structure: value is ( obj | quoted_string | unquoted_string )
        match grammar_value.as_rule() {
            // Structure: ( quoted_string | unquoted_string )
            Rule::quoted_string | Rule::unquoted_string => Self::Str(parse_string(grammar_value)),
            // Structure: obj
            //            \ pair* <- Desired
            Rule::obj => {
                let mut obj = Obj::new();
                for grammar_pair in grammar_value.into_inner() {
                    let (key, value) = parse_pair(grammar_pair);
                    let entry = obj.entry(key).or_default();
                    (*entry).push(value);
                }

                Self::Obj(obj)
            }
            _ => unreachable!("Prevented by grammar"),
        }
    }
}
