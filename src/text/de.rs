use pest::{error::Error as PestError, iterators::Pair as PestPair, Parser};
use pest_derive::Parser;

use std::convert::TryFrom;

use crate::common::{KeyValues, Value, Vdf};

#[derive(Parser)]
#[grammar = "grammars/text.pest"]
struct VdfParser;

impl<'a> Vdf<'a> {
    pub fn parse(s: &'a str) -> Result<Self, PestError<Rule>> {
        Self::try_from(s)
    }
}

impl<'a> TryFrom<&'a str> for Vdf<'a> {
    type Error = PestError<Rule>;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
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
                    let entry = container.entry(key).or_insert(Vec::new());
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
            Rule::string => Self::Str(grammar_value.into_inner().next().unwrap().as_str()),
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
    fn checking() {
        let sample_vdf = r#"
"Key" "Value"
"Key" "Value2"
"Key"
{
    "Inner Key" "Inner Value"
    "Inner Key"
    {
        "Rar" "Bar"
    }
}
        "#;
        let vdf = Vdf::parse(sample_vdf).unwrap();
        // let desired_value = vdf["Key"][2]
        //     .get_obj()
        //     .and_then(|obj| obj["Inner Key"][0].get_str());
        println!("{}", vdf["Key"][0]);
        panic!();
    }
}
