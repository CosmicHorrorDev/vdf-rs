use super::*;

pub type PestError = pest::error::Error<Rule>;
type BoxedState<'a> = Box<pest::ParserState<'a, Rule>>;
type ParseResult<'a> = pest::ParseResult<BoxedState<'a>>;

common_parsing!(pest_parse, Rule, true);

#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Rule {
    ///End-of-input
    EOI,
    WHITESPACE,
    COMMENT,
    vdf,
    base_macro,
    quoted_raw_string,
    quoted_raw_inner,
    pairs,
    pair,
    key,
    value,
    obj,
    quoted_string,
    quoted_inner,
    char,
    unquoted_string,
    unquoted_char,
}

mod rules {
    #![allow(non_snake_case)]

    use super::{any, skip, soi, whitespace, BoxedState, ParseResult, Rule};

    use pest::Atomicity;

    #[inline]
    pub fn vdf(s: BoxedState<'_>) -> ParseResult<'_> {
        s.sequence(|s| {
            soi(s)
                .and_then(skip)
                .and_then(|s| {
                    s.sequence(|s| {
                        s.optional(|s| {
                            base_macro(s).and_then(|s| {
                                s.repeat(|s| s.sequence(|s| skip(s).and_then(base_macro)))
                            })
                        })
                    })
                })
                .and_then(skip)
                .and_then(pair)
                .and_then(skip)
                .and_then(|s| s.optional(|s| s.match_string("\0")))
                .and_then(skip)
                .and_then(EOI)
        })
    }
    pub fn base_macro(s: BoxedState<'_>) -> ParseResult<'_> {
        s.rule(Rule::base_macro, |s| {
            s.sequence(|s| {
                s.match_string("#base")
                    .and_then(skip)
                    .and_then(|s| quoted_raw_string(s).or_else(unquoted_string))
            })
        })
    }
    #[inline]
    pub fn quoted_raw_string(s: BoxedState<'_>) -> ParseResult<'_> {
        s.atomic(Atomicity::CompoundAtomic, |s| {
            s.rule(Rule::quoted_raw_string, |s| {
                s.sequence(|s| {
                    s.match_string("\"")
                        .and_then(quoted_raw_inner)
                        .and_then(|s| s.match_string("\""))
                })
            })
        })
    }
    #[inline]
    pub fn quoted_raw_inner(s: BoxedState<'_>) -> ParseResult<'_> {
        s.rule(Rule::quoted_raw_inner, |s| {
            s.atomic(Atomicity::Atomic, |s| s.skip_until(&["\""]))
        })
    }
    #[inline]
    pub fn pairs(s: BoxedState<'_>) -> ParseResult<'_> {
        s.sequence(|s| {
            s.optional(|s| {
                pair(s).and_then(|s| s.repeat(|s| s.sequence(|s| skip(s).and_then(pair))))
            })
        })
    }
    #[inline]
    pub fn pair(s: BoxedState<'_>) -> ParseResult<'_> {
        s.rule(Rule::pair, |s| {
            s.sequence(|s| key(s).and_then(skip).and_then(value))
        })
    }
    #[inline]
    pub fn key(s: BoxedState<'_>) -> ParseResult<'_> {
        quoted_string(s).or_else(unquoted_string)
    }
    #[inline]
    pub fn value(s: BoxedState<'_>) -> ParseResult<'_> {
        quoted_string(s).or_else(obj).or_else(unquoted_string)
    }
    #[inline]
    pub fn obj(s: BoxedState<'_>) -> ParseResult<'_> {
        s.rule(Rule::obj, |s| {
            s.sequence(|s| {
                s.match_string("{")
                    .and_then(skip)
                    .and_then(pairs)
                    .and_then(skip)
                    .and_then(|s| s.match_string("}"))
            })
        })
    }
    #[inline]
    pub fn quoted_string(s: BoxedState<'_>) -> ParseResult<'_> {
        s.atomic(Atomicity::CompoundAtomic, |s| {
            s.rule(Rule::quoted_string, |s| {
                s.sequence(|s| {
                    s.match_string("\"")
                        .and_then(quoted_inner)
                        .and_then(|s| s.match_string("\""))
                })
            })
        })
    }
    #[inline]
    pub fn quoted_inner(s: BoxedState<'_>) -> ParseResult<'_> {
        s.rule(Rule::quoted_inner, |s| {
            s.atomic(Atomicity::Atomic, |s| s.repeat(char))
        })
    }
    #[inline]
    pub fn char(s: BoxedState<'_>) -> ParseResult<'_> {
        s.rule(Rule::char, |s| {
            s.sequence(|s| {
                s.lookahead(false, |s| {
                    s.match_string("\"").or_else(|s| s.match_string("\\"))
                })
                .and_then(skip)
                .and_then(any)
            })
            .or_else(|s| {
                s.sequence(|s| {
                    s.match_string("\\").and_then(skip).and_then(|s| {
                        s.match_string("\"")
                            .or_else(|s| s.match_string("\\"))
                            .or_else(|s| s.match_string("n"))
                            .or_else(|s| s.match_string("r"))
                            .or_else(|s| s.match_string("t"))
                    })
                })
            })
        })
    }
    #[inline]
    pub fn unquoted_string(s: BoxedState<'_>) -> ParseResult<'_> {
        s.rule(Rule::unquoted_string, |s| {
            s.atomic(Atomicity::Atomic, |s| {
                s.sequence(|s| unquoted_char(s).and_then(|s| s.repeat(unquoted_char)))
            })
        })
    }
    #[inline]
    pub fn unquoted_char(s: BoxedState<'_>) -> ParseResult<'_> {
        s.rule(Rule::unquoted_char, |s| {
            s.sequence(|s| {
                s.lookahead(false, |s| {
                    s.match_string("\"")
                        .or_else(|s| s.match_string("{"))
                        .or_else(|s| s.match_string("}"))
                        .or_else(whitespace)
                })
                .and_then(skip)
                .and_then(any)
            })
        })
    }
    pub fn EOI(s: BoxedState<'_>) -> ParseResult<'_> {
        s.rule(Rule::EOI, |s| s.end_of_input())
    }
}

pub fn pest_parse(input: &str) -> std::result::Result<pest::iterators::Pairs<'_, Rule>, PestError> {
    pest::state(input, rules::vdf)
}
