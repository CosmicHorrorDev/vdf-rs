// Due to Pest generating variants that are all uppercase
#![allow(renamed_and_removed_lints)]
#![allow(clippy::unknown_clippy_lints)]
#![allow(clippy::upper_case_acronyms)]

use pest::{iterators::Pair as PestPair, Parser};
use pest_derive::Parser;

use std::borrow::Cow;

use crate::{error::Result, Obj, Value, Vdf};

macro_rules! common_parsing {
    ($parser:ty, $rule:ty, $parse_escaped:expr) => {
        fn parse_pair(grammar_pair: PestPair<'_, $rule>) -> (Cow<'_, str>, Value<'_>) {
            // Structure: pair
            //            \ key   <- Desired
            //            \ value <- Desired
            if let <$rule>::pair = grammar_pair.as_rule() {
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

        fn parse_string(grammar_string: PestPair<'_, $rule>) -> Cow<'_, str> {
            match grammar_string.as_rule() {
                // Structure: quoted_string
                //            \ "
                //            \ quoted_inner <- Desired
                //            \ "
                <$rule>::quoted_string => {
                    let quoted_inner = grammar_string.into_inner().next().unwrap();
                    if $parse_escaped {
                        parse_escaped_string(quoted_inner)
                    } else {
                        Cow::from(quoted_inner.as_str())
                    }
                }
                // Structure: unquoted_string <- Desired
                <$rule>::unquoted_string => {
                    let s = grammar_string.as_str();
                    Cow::from(s)
                }
                _ => unreachable!("Prevented by grammar"),
            }
        }

        // Note: there can be a slight performance win here by having the grammar skip capturing
        // quoted_inner and instead just slice off the starting and ending '"', but I'm going to pass since
        // it seems like a hack for a ~4% improvement
        fn parse_escaped_string(inner: PestPair<'_, $rule>) -> Cow<'_, str> {
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

        /// Attempts to parse VDF text to a [`Vdf`][crate::Vdf]
        pub fn parse<'a>(s: &'a str) -> Result<Vdf<'a>> {
            let unparsed = <$parser>::parse(<$rule>::vdf, s)?.next().unwrap();
            Ok(Vdf::from(unparsed))
        }

        impl<'a> From<PestPair<'a, $rule>> for Vdf<'a> {
            fn from(grammar_pair: PestPair<'a, $rule>) -> Self {
                let (key, value) = parse_pair(grammar_pair);
                Self::new(key, value)
            }
        }

        impl<'a> From<PestPair<'a, $rule>> for Value<'a> {
            fn from(grammar_value: PestPair<'a, $rule>) -> Self {
                // Structure: value is ( obj | quoted_string | unquoted_string )
                match grammar_value.as_rule() {
                    // Structure: ( quoted_string | unquoted_string )
                    <$rule>::quoted_string | <$rule>::unquoted_string => {
                        Self::Str(parse_string(grammar_value))
                    }
                    // Structure: obj
                    //            \ pair* <- Desired
                    <$rule>::obj => {
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
    };
}

pub use escaped::{parse as escaped_parse, PestError as EscapedPestError};
pub use raw::{parse as raw_parse, PestError as RawPestError};

pub struct Opts {
    pub parse_escaped_characters: bool,
}

impl Default for Opts {
    fn default() -> Self {
        // For now I'm gonna default this to true since I'm hoping that new VDF generally respects
        // escaped characters
        Self {
            parse_escaped_characters: true,
        }
    }
}

impl<'a> Vdf<'a> {
    /// Attempts to parse VDF text to a [`Vdf`][crate::Vdf]
    pub fn parse(s: &'a str) -> Result<Self> {
        escaped_parse(s)
    }

    // FIXME: How should rendering be handled now? It's fallible depending on the characters
    // included in strings if it's not escaped
    pub fn parse_with_opts(s: &'a str, opts: Opts) -> Result<Self> {
        if opts.parse_escaped_characters {
            escaped_parse(s)
        } else {
            raw_parse(s)
        }
    }
}

mod escaped {
    use super::*;

    #[derive(Parser)]
    #[grammar = "grammars/escaped.pest"]
    struct EscapedParser;

    pub type PestError = pest::error::Error<Rule>;

    common_parsing!(EscapedParser, Rule, true);
}

mod raw {
    use super::*;

    #[derive(Parser)]
    #[grammar = "grammars/raw.pest"]
    struct RawParser;

    pub type PestError = pest::error::Error<Rule>;

    common_parsing!(RawParser, Rule, false);
}
