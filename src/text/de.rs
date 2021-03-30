use pest::{error::Error as PestError, iterators::Pair as PestPair, Parser};
use pest_derive::Parser;

use std::convert::TryFrom;

use crate::common::{Pair, Value, Vdf};

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
    fn from(pest_pair: PestPair<'a, Rule>) -> Self {
        // Structure: pairs
        //            \ pair* <- Desired
        if let Rule::pairs = pest_pair.as_rule() {
            Self(pest_pair.into_inner().map(Pair::from).collect())
        } else {
            unreachable!("Prevented by grammar");
        }
    }
}

impl<'a> From<PestPair<'a, Rule>> for Pair<'a> {
    fn from(pest_pair: PestPair<'a, Rule>) -> Self {
        // Structure: pair
        //            \ string (key) <- Desired
        //            \ value        <- Desired
        if let Rule::pair = pest_pair.as_rule() {
            let mut inner_rules = pest_pair.into_inner();
            let key = inner_rules
                .next()
                .unwrap()
                .into_inner()
                .next()
                .unwrap()
                .as_str();
            let value = Value::from(inner_rules.next().unwrap());

            Self(key, value)
        } else {
            unreachable!("Prevented by grammar");
        }
    }
}

impl<'a> From<PestPair<'a, Rule>> for Value<'a> {
    fn from(pest_pair: PestPair<'a, Rule>) -> Self {
        match pest_pair.as_rule() {
            // Structure: string
            //            \ inner <- Desired
            Rule::string => Self::Str(pest_pair.into_inner().next().unwrap().as_str()),
            // Structure: obj
            //            \ pair* <- Desired
            Rule::obj => Self::Obj(pest_pair.into_inner().map(Pair::from).collect()),
            _ => unreachable!("Prevented by grammar"),
        }
    }
}
