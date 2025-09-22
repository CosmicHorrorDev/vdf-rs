//! Handles generating the parsers from the grammar files

use std::{
    env, fs,
    path::{Path, PathBuf},
};

use pest_generator::derive_parser;
use quote::quote;
use syn::Item;

#[test]
fn escaped() {
    snapshot_or_update_parser(Parser::Escaped);
}

#[test]
fn raw() {
    snapshot_or_update_parser(Parser::Raw);
}

#[derive(Clone, Copy, PartialEq)]
enum Parser {
    Escaped,
    Raw,
}

impl Parser {
    fn grammar_path(self) -> &'static str {
        match self {
            Self::Escaped => "grammars/escaped.pest",
            Self::Raw => "grammars/raw.pest",
        }
    }

    fn parser_path(self) -> PathBuf {
        Path::new("src")
            .join("text")
            .join("parse")
            .join(match self {
                Self::Escaped => "escaped.rs",
                Self::Raw => "raw.rs",
            })
    }
}

#[track_caller]
fn snapshot_or_update_parser(parser: Parser) {
    let parser_file = generate_file(parser);

    if env::var_os("UPDATE_PARSERS").is_some() {
        fs::write(parser.parser_path(), parser_file).unwrap();
    } else {
        // otherwise we check the output against the existing parser
        let existing_file = fs::read_to_string(parser.parser_path()).unwrap();
        if parser_file != existing_file {
            panic!(
                "existing parser is out of sync! update the parsers by running the test suite with \
                the env var `UPDATE_PARSERS` set to anything"
            );
        }
    }
}

#[track_caller]
fn generate_file(parser: Parser) -> String {
    let grammar_path = parser.grammar_path();
    let derive_tokens = quote! {
        #[grammar = #grammar_path]
        struct Parser;
    };
    let expanded_tokens = derive_parser(derive_tokens.clone(), false);
    let is_escaped = parser == Parser::Escaped;
    let file = quote! {
        use super::*;
        use pest::Parser as _;
        pub type PestError = pest::error::Error<Rule>;
        struct Parser;
        crate::common_parsing!(Parser, Rule, #is_escaped);
        #expanded_tokens
    };
    let mut file = syn::parse_file(&file.to_string()).unwrap();
    cleanup_file(&mut file);
    let formatted = prettyplease::unparse(&file);
    format!(
        "// !!GENERATED CODE!! DO NOT EDIT MANUALLY. edit through the `grammar_generator` tests\n\
        {formatted}"
    )
}

fn cleanup_file(file: &mut syn::File) {
    file.items.retain_mut(|mut item| match &mut item {
        // `pest` includes the grammar with an absolute file path. at the very least the path
        // would need to be normalized, but the grammar doesn't seem to actually be used for
        // anything
        Item::Const(c) => !c.ident.to_string().starts_with("_PEST_GRAMMAR"),
        _ => true,
    });
}
