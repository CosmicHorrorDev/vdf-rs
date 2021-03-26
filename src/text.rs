use pest::{error::Error as PestError, iterators::Pair, Parser};

use std::convert::TryFrom;

#[derive(Debug)]
enum VdfValue<'a> {
    String(&'a str),
    Map(Vec<VdfPair<'a>>),
}

impl<'a> From<Pair<'a, Rule>> for VdfValue<'a> {
    fn from(pair: Pair<'a, Rule>) -> Self {
        match pair.as_rule() {
            Rule::string => VdfValue::String(pair.into_inner().next().unwrap().as_str()),
            Rule::map => VdfValue::Map(pair.into_inner().map(VdfPair::from).collect()),
            _ => unreachable!("Prevented by grammar"),
        }
    }
}

#[derive(Debug)]
struct Vdf<'a>(VdfPair<'a>);

impl<'a> TryFrom<&'a str> for Vdf<'a> {
    type Error = PestError<Rule>;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        VdfPair::try_from(s).map(|vdf_pair| Vdf(vdf_pair))
    }
}

#[derive(Debug)]
struct VdfPair<'a>(&'a str, VdfValue<'a>);

impl<'a> TryFrom<&'a str> for VdfPair<'a> {
    type Error = PestError<Rule>;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let unparsed = VdfParser::parse(Rule::vdf, s)?.next().unwrap();

        if let Rule::pair = unparsed.as_rule() {
            Ok(VdfPair::from(unparsed))
        } else {
            unreachable!("Prevented by grammar")
        }
    }
}

impl<'a> From<Pair<'a, Rule>> for VdfPair<'a> {
    fn from(pair: Pair<'a, Rule>) -> Self {
        if let Rule::pair = pair.as_rule() {
            let mut inner_rules = pair.into_inner();
            let name = inner_rules
                .next()
                .unwrap()
                .into_inner()
                .next()
                .unwrap()
                .as_str();
            let value = VdfValue::from(inner_rules.next().unwrap());

            VdfPair(name, value)
        } else {
            unreachable!("Prevented by grammar")
        }
    }
}

#[derive(Parser)]
#[grammar = "vdf.pest"]
struct VdfParser;
