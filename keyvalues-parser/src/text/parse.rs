// Due to Pest generating variants that are all uppercase
#![allow(clippy::upper_case_acronyms)]

use pest::{iterators::Pair as PestPair, Parser};
use pest_derive::Parser;

use std::{borrow::Cow, convert::TryFrom};

use crate::{
    core::{Obj, Value, Vdf},
    error::{Error, Result},
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
    let s = match grammar_string.as_rule() {
        // Structure: quoted_string
        //            \ "
        //            \ quoted_inner <- Desired
        //            \ "
        Rule::quoted_string => grammar_string.into_inner().next().unwrap(),
        // Structure: unquoted_string <- Desired
        Rule::unquoted_string => grammar_string,
        _ => unreachable!("Prevented by grammar"),
    }
    .as_str();

    Cow::from(s)
}

impl<'a> Vdf<'a> {
    // TODO: implement this as fromstr instead?
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
        println!("{:#?}", unparsed);
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
            //            \ pairs <- Desired
            Rule::obj => {
                let grammar_pairs = grammar_value.into_inner().next().unwrap();

                // Structure: pairs
                //            \ pair* <- Desired
                if let Rule::pairs = grammar_pairs.as_rule() {
                    // Parse out each pair and add them to the `Obj`
                    let mut obj = Obj::new();
                    for grammar_pair in grammar_pairs.into_inner() {
                        let (key, value) = parse_pair(grammar_pair);
                        let entry = obj.entry(key).or_default();
                        (*entry).push(value);
                    }

                    Self::Obj(obj)
                } else {
                    unreachable!("Prevented by grammar");
                }
            }
            _ => unreachable!("Prevented by grammar"),
        }
    }
}
