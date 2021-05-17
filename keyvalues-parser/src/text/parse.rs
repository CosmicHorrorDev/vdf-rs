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

fn parse_pair<'a>(grammar_pair: PestPair<'a, Rule>) -> (Cow<'a, str>, Value<'a>) {
    // Structure: pair
    //            \ key   <- Desired
    //            \ value <- Desired
    if let Rule::pair = grammar_pair.as_rule() {
        // Parse out the key and value
        let mut grammar_pair_innards = grammar_pair.into_inner();
        let key_str = grammar_pair_innards
            .next()
            .unwrap()
            .into_inner()
            .next()
            .unwrap()
            .as_str();
        let key = Cow::from(key_str);

        let grammar_value = grammar_pair_innards.next().unwrap();
        let value = Value::from(grammar_value);

        (key, value)
    } else {
        unreachable!("Prevented by grammar");
    }
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
        Ok(Self::from(unparsed))
    }
}

impl<'a> From<PestPair<'a, Rule>> for Vdf<'a> {
    fn from(grammar_pair: PestPair<'a, Rule>) -> Self {
        let (key, value) = parse_pair(grammar_pair);
        Self { key, value }
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

                    Value::Obj(obj)
                } else {
                    unreachable!("Prevented by grammar");
                }
            }
            _ => unreachable!("Prevented by grammar"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // #[ignore = "just for helping develop things"]
    fn checking() {
        let sample_vdf = r#"
"Outer Key"
{
    "Inner Key" "Inner Value"
    "Seq" "1"
    "Seq" "2"
    "Inner Obj"
    {
        "Inner Inner Key" "Inner Inner Val"
    }
    "Empty Obj"
    {
    }
}
        "#;
        let mut vdf = Vdf::parse(sample_vdf).unwrap();
        // vdf.get_mut("Key").map(|values| {
        //     if let Value::Str(s) = &mut values[1] {
        //         *s = Cow::from(s.to_mut().to_uppercase());
        //     }
        // });
        println!("{}", vdf);
        panic!();
    }
}
