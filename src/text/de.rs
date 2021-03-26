use pest::{error::Error as PestError, iterators::Pair, Parser};

use std::convert::TryFrom;

use crate::common::{Vdf, VdfPair, VdfValue};

#[derive(Parser)]
#[grammar = "grammars/text.pest"]
struct VdfParser;

impl<'a> TryFrom<&'a str> for Vdf<'a> {
    type Error = PestError<Rule>;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        VdfPair::try_from(s).map(Self)
    }
}

impl<'a> TryFrom<&'a str> for VdfPair<'a> {
    type Error = PestError<Rule>;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let unparsed = VdfParser::parse(Rule::vdf, s)?.next().unwrap();
        Ok(Self::from(unparsed))
    }
}

impl<'a> From<Pair<'a, Rule>> for VdfPair<'a> {
    fn from(pair: Pair<'a, Rule>) -> Self {
        if let Rule::pair = pair.as_rule() {
            let mut inner_rules = pair.into_inner();
            let key = inner_rules
                .next()
                .unwrap()
                .into_inner()
                .next()
                .unwrap()
                .as_str();
            let value = VdfValue::from(inner_rules.next().unwrap());

            Self(key, value)
        } else {
            unreachable!("Prevented by grammar")
        }
    }
}

impl<'a> From<Pair<'a, Rule>> for VdfValue<'a> {
    fn from(pair: Pair<'a, Rule>) -> Self {
        match pair.as_rule() {
            Rule::string => VdfValue::Str(pair.into_inner().next().unwrap().as_str()),
            Rule::obj => VdfValue::Obj(pair.into_inner().map(VdfPair::from).collect()),
            _ => unreachable!("Prevented by grammar"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let sample_vdf = r#"
"Outer Key"
{
    "Inner Key1" "Val"
    "Inner Key2"
    {
    }
}
"#;

        let vdf = Vdf::try_from(sample_vdf).unwrap();
        println!("{:#?}", vdf);
        assert!(false);
    }
}
