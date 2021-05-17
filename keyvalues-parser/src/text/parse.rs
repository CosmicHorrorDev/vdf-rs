use pest::{iterators::Pair as PestPair, Parser};
use pest_derive::Parser;

use std::{borrow::Cow, convert::TryFrom};

use crate::{
    core::{KeyValues, Value, Vdf},
    error::{Error, Result},
};

#[derive(Parser)]
#[grammar = "text/grammar.pest"]
struct VdfParser;

pub(crate) type PestError = pest::error::Error<Rule>;

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
        //            \ pairs <- Desired
        //            \ EOI
        let unparsed = VdfParser::parse(Rule::vdf, s)?.next().unwrap();
        Ok(Self::from(unparsed))
    }
}

impl<'a> From<PestPair<'a, Rule>> for Vdf<'a> {
    fn from(grammar_pairs: PestPair<'a, Rule>) -> Self {
        // Structure: pairs
        //            \ pair* <- Desired
        if let Rule::pairs = grammar_pairs.as_rule() {
            let mut container = KeyValues::new();
            for grammar_pair in grammar_pairs.into_inner() {
                // Structure: pair
                //            \ key   <- Desired
                //            \ value <- Desired
                if let Rule::pair = grammar_pair.as_rule() {
                    // Parse out the key and value
                    let mut grammar_pair_innards = grammar_pair.into_inner();
                    let key = grammar_pair_innards
                        .next()
                        .unwrap()
                        .into_inner()
                        .next()
                        .unwrap()
                        .as_str();
                    let grammar_value = grammar_pair_innards.next().unwrap();
                    let value = Value::from(grammar_value);

                    // Insert pair into `KeyValues`
                    let entry = container.entry(Cow::from(key)).or_insert(Vec::new());
                    (*entry).push(value);
                } else {
                    unreachable!("Prevented by grammar");
                }
            }

            Self(container)
        } else {
            unreachable!("Prevented by grammar");
        }
    }
}

impl<'a> From<PestPair<'a, Rule>> for Value<'a> {
    fn from(grammar_value: PestPair<'a, Rule>) -> Self {
        // Structure: value is ( string | obj )
        match grammar_value.as_rule() {
            // Structure: string
            //            \ inner <- Desired
            Rule::string => {
                let value = grammar_value.into_inner().next().unwrap().as_str();
                Self::Str(Cow::from(value))
            }
            // Structure: obj
            //            \ pairs <- Desired
            Rule::obj => Self::Obj(Vdf::from(grammar_value.into_inner().next().unwrap())),
            _ => unreachable!("Prevented by grammar"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "just for helping develop things"]
    fn checking() {
        let sample_vdf = r#"
"Outer Key" "Outer Value"
"Outer Key"
{
    "Inner Key" "Inner Value"
}
        "#;
        let mut vdf = Vdf::parse(sample_vdf).unwrap();
        vdf.get_mut("Key").map(|values| {
            if let Value::Str(s) = &mut values[1] {
                *s = Cow::from(s.to_mut().to_uppercase());
            }
        });
        println!("{:#?}", vdf);
        panic!();
    }
}
