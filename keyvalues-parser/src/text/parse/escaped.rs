// !!GENERATED CODE!! DO NOT EDIT MANUALLY. edit through the `grammar_generator` tests
use super::*;
use pest::Parser as _;
pub type PestError = pest::error::Error<Rule>;
struct Parser;
common_parsing!(Parser, Rule, true);
#[allow(dead_code, non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Rule {
    ///End-of-input
    EOI,
    r#WHITESPACE,
    r#COMMENT,
    r#vdf,
    r#base_macro,
    r#quoted_raw_string,
    r#quoted_raw_inner,
    r#pairs,
    r#pair,
    r#key,
    r#value,
    r#obj,
    r#quoted_string,
    r#quoted_inner,
    r#char,
    r#unquoted_string,
    r#unquoted_char,
}
impl Rule {
    pub fn all_rules() -> &'static [Rule] {
        &[
            Rule::r#WHITESPACE,
            Rule::r#COMMENT,
            Rule::r#vdf,
            Rule::r#base_macro,
            Rule::r#quoted_raw_string,
            Rule::r#quoted_raw_inner,
            Rule::r#pairs,
            Rule::r#pair,
            Rule::r#key,
            Rule::r#value,
            Rule::r#obj,
            Rule::r#quoted_string,
            Rule::r#quoted_inner,
            Rule::r#char,
            Rule::r#unquoted_string,
            Rule::r#unquoted_char,
        ]
    }
}
#[allow(clippy::all)]
impl ::pest::Parser<Rule> for Parser {
    fn parse<'i>(
        rule: Rule,
        input: &'i str,
    ) -> ::std::result::Result<
        ::pest::iterators::Pairs<'i, Rule>,
        ::pest::error::Error<Rule>,
    > {
        mod rules {
            #![allow(clippy::upper_case_acronyms)]
            pub mod hidden {
                use super::super::Rule;
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn skip(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    if state.atomicity() == ::pest::Atomicity::NonAtomic {
                        state
                            .sequence(|state| {
                                state
                                    .repeat(|state| super::visible::WHITESPACE(state))
                                    .and_then(|state| {
                                        state
                                            .repeat(|state| {
                                                state
                                                    .sequence(|state| {
                                                        super::visible::COMMENT(state)
                                                            .and_then(|state| {
                                                                state.repeat(|state| super::visible::WHITESPACE(state))
                                                            })
                                                    })
                                            })
                                    })
                            })
                    } else {
                        Ok(state)
                    }
                }
            }
            pub mod visible {
                use super::super::Rule;
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#WHITESPACE(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .atomic(
                            ::pest::Atomicity::Atomic,
                            |state| {
                                state
                                    .match_string(" ")
                                    .or_else(|state| { state.match_string("\t") })
                                    .or_else(|state| { state.match_string("\r") })
                                    .or_else(|state| { state.match_string("\n") })
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#COMMENT(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .atomic(
                            ::pest::Atomicity::Atomic,
                            |state| {
                                state
                                    .sequence(|state| {
                                        state
                                            .match_string("//")
                                            .and_then(|state| {
                                                state
                                                    .repeat(|state| {
                                                        state
                                                            .sequence(|state| {
                                                                state
                                                                    .lookahead(false, |state| { state.match_string("\n") })
                                                                    .and_then(|state| { self::r#ANY(state) })
                                                            })
                                                    })
                                            })
                                    })
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#vdf(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .sequence(|state| {
                            self::r#SOI(state)
                                .and_then(|state| { super::hidden::skip(state) })
                                .and_then(|state| {
                                    state
                                        .sequence(|state| {
                                            state
                                                .optional(|state| {
                                                    self::r#base_macro(state)
                                                        .and_then(|state| {
                                                            state
                                                                .repeat(|state| {
                                                                    state
                                                                        .sequence(|state| {
                                                                            super::hidden::skip(state)
                                                                                .and_then(|state| { self::r#base_macro(state) })
                                                                        })
                                                                })
                                                        })
                                                })
                                        })
                                })
                                .and_then(|state| { super::hidden::skip(state) })
                                .and_then(|state| { self::r#pair(state) })
                                .and_then(|state| { super::hidden::skip(state) })
                                .and_then(|state| {
                                    state.optional(|state| { state.match_string("\0") })
                                })
                                .and_then(|state| { super::hidden::skip(state) })
                                .and_then(|state| { self::r#EOI(state) })
                        })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#base_macro(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#base_macro,
                            |state| {
                                state
                                    .sequence(|state| {
                                        state
                                            .match_string("#base")
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| {
                                                self::r#quoted_raw_string(state)
                                                    .or_else(|state| { self::r#unquoted_string(state) })
                                            })
                                    })
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#quoted_raw_string(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .atomic(
                            ::pest::Atomicity::CompoundAtomic,
                            |state| {
                                state
                                    .rule(
                                        Rule::r#quoted_raw_string,
                                        |state| {
                                            state
                                                .sequence(|state| {
                                                    state
                                                        .match_string("\"")
                                                        .and_then(|state| { self::r#quoted_raw_inner(state) })
                                                        .and_then(|state| { state.match_string("\"") })
                                                })
                                        },
                                    )
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#quoted_raw_inner(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#quoted_raw_inner,
                            |state| {
                                state
                                    .atomic(
                                        ::pest::Atomicity::Atomic,
                                        |state| {
                                            let strings = ["\""];
                                            state.skip_until(&strings)
                                        },
                                    )
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#pairs(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .sequence(|state| {
                            state
                                .optional(|state| {
                                    self::r#pair(state)
                                        .and_then(|state| {
                                            state
                                                .repeat(|state| {
                                                    state
                                                        .sequence(|state| {
                                                            super::hidden::skip(state)
                                                                .and_then(|state| { self::r#pair(state) })
                                                        })
                                                })
                                        })
                                })
                        })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#pair(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#pair,
                            |state| {
                                state
                                    .sequence(|state| {
                                        self::r#key(state)
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { self::r#value(state) })
                                    })
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#key(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    self::r#quoted_string(state)
                        .or_else(|state| { self::r#unquoted_string(state) })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#value(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    self::r#quoted_string(state)
                        .or_else(|state| { self::r#obj(state) })
                        .or_else(|state| { self::r#unquoted_string(state) })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#obj(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#obj,
                            |state| {
                                state
                                    .sequence(|state| {
                                        state
                                            .match_string("{")
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { self::r#pairs(state) })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { state.match_string("}") })
                                    })
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#quoted_string(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .atomic(
                            ::pest::Atomicity::CompoundAtomic,
                            |state| {
                                state
                                    .rule(
                                        Rule::r#quoted_string,
                                        |state| {
                                            state
                                                .sequence(|state| {
                                                    state
                                                        .match_string("\"")
                                                        .and_then(|state| { self::r#quoted_inner(state) })
                                                        .and_then(|state| { state.match_string("\"") })
                                                })
                                        },
                                    )
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#quoted_inner(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#quoted_inner,
                            |state| {
                                state
                                    .atomic(
                                        ::pest::Atomicity::Atomic,
                                        |state| { state.repeat(|state| { self::r#char(state) }) },
                                    )
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#char(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#char,
                            |state| {
                                state
                                    .sequence(|state| {
                                        state
                                            .lookahead(
                                                false,
                                                |state| {
                                                    state
                                                        .match_string("\"")
                                                        .or_else(|state| { state.match_string("\\") })
                                                },
                                            )
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { self::r#ANY(state) })
                                    })
                                    .or_else(|state| {
                                        state
                                            .sequence(|state| {
                                                state
                                                    .match_string("\\")
                                                    .and_then(|state| { super::hidden::skip(state) })
                                                    .and_then(|state| {
                                                        state
                                                            .match_string("\"")
                                                            .or_else(|state| { state.match_string("\\") })
                                                            .or_else(|state| { state.match_string("n") })
                                                            .or_else(|state| { state.match_string("r") })
                                                            .or_else(|state| { state.match_string("t") })
                                                    })
                                            })
                                    })
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#unquoted_string(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#unquoted_string,
                            |state| {
                                state
                                    .atomic(
                                        ::pest::Atomicity::Atomic,
                                        |state| {
                                            state
                                                .sequence(|state| {
                                                    self::r#unquoted_char(state)
                                                        .and_then(|state| {
                                                            state.repeat(|state| { self::r#unquoted_char(state) })
                                                        })
                                                })
                                        },
                                    )
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#unquoted_char(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#unquoted_char,
                            |state| {
                                state
                                    .sequence(|state| {
                                        state
                                            .lookahead(
                                                false,
                                                |state| {
                                                    state
                                                        .match_string("\"")
                                                        .or_else(|state| { state.match_string("{") })
                                                        .or_else(|state| { state.match_string("}") })
                                                        .or_else(|state| { self::r#WHITESPACE(state) })
                                                },
                                            )
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { self::r#ANY(state) })
                                    })
                            },
                        )
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ANY(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state.skip(1)
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn EOI(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state.rule(Rule::EOI, |state| state.end_of_input())
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn SOI(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state.start_of_input()
                }
            }
            pub use self::visible::*;
        }
        ::pest::state(
            input,
            |state| {
                match rule {
                    Rule::r#WHITESPACE => rules::r#WHITESPACE(state),
                    Rule::r#COMMENT => rules::r#COMMENT(state),
                    Rule::r#vdf => rules::r#vdf(state),
                    Rule::r#base_macro => rules::r#base_macro(state),
                    Rule::r#quoted_raw_string => rules::r#quoted_raw_string(state),
                    Rule::r#quoted_raw_inner => rules::r#quoted_raw_inner(state),
                    Rule::r#pairs => rules::r#pairs(state),
                    Rule::r#pair => rules::r#pair(state),
                    Rule::r#key => rules::r#key(state),
                    Rule::r#value => rules::r#value(state),
                    Rule::r#obj => rules::r#obj(state),
                    Rule::r#quoted_string => rules::r#quoted_string(state),
                    Rule::r#quoted_inner => rules::r#quoted_inner(state),
                    Rule::r#char => rules::r#char(state),
                    Rule::r#unquoted_string => rules::r#unquoted_string(state),
                    Rule::r#unquoted_char => rules::r#unquoted_char(state),
                    Rule::EOI => rules::EOI(state),
                }
            },
        )
    }
}
