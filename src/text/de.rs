use pest::{error::Error as PestError, iterators::Pair as PestPair, Parser};

use std::convert::TryFrom;

use crate::common::{Pair, Value, Vdf};

#[derive(Parser)]
#[grammar = "grammars/text.pest"]
struct VdfParser;

impl<'a> TryFrom<&'a str> for Vdf<'a> {
    type Error = PestError<Rule>;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        Pair::try_from(s).map(Self)
    }
}

impl<'a> TryFrom<&'a str> for Pair<'a> {
    type Error = PestError<Rule>;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let unparsed = VdfParser::parse(Rule::vdf, s)?.next().unwrap();
        Ok(Self::from(unparsed))
    }
}

impl<'a> From<PestPair<'a, Rule>> for Pair<'a> {
    fn from(pair: PestPair<'a, Rule>) -> Self {
        if let Rule::pair = pair.as_rule() {
            let mut inner_rules = pair.into_inner();
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
            unreachable!("Prevented by grammar")
        }
    }
}

impl<'a> From<PestPair<'a, Rule>> for Value<'a> {
    fn from(pair: PestPair<'a, Rule>) -> Self {
        match pair.as_rule() {
            Rule::string => Value::Str(pair.into_inner().next().unwrap().as_str()),
            Rule::obj => Value::Obj(pair.into_inner().map(Pair::from).collect()),
            _ => unreachable!("Prevented by grammar"),
        }
    }
}
