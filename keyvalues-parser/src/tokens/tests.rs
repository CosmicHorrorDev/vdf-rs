use std::{borrow::Cow, collections::BTreeMap, convert::TryFrom};

use crate::{
    core::{Value, Vdf},
    tokens::{
        naive::{NaiveToken, NaiveTokenStream},
        Token, TokenStream,
    },
};

#[test]
fn vdf_from_token_stream_basics() {
    let naive_token_stream = NaiveTokenStream(vec![
        NaiveToken::str("outer"),
        NaiveToken::ObjBegin,
        NaiveToken::str("sequence start"),
        NaiveToken::SeqBegin,
        NaiveToken::ObjBegin,
        NaiveToken::str("inner key"),
        NaiveToken::str("inner val"),
        NaiveToken::ObjEnd,
        NaiveToken::str("some other inner val"),
        NaiveToken::SeqEnd,
        NaiveToken::ObjEnd,
    ]);

    let ideal = {
        let mut sequence_obj = BTreeMap::new();
        sequence_obj.insert(
            Cow::from("inner key"),
            vec![Value::Str(Cow::from("inner val"))],
        );

        let mut outer_val = BTreeMap::new();
        outer_val.insert(
            Cow::from("sequence start"),
            vec![
                Value::Obj(Vdf(sequence_obj)),
                Value::Str(Cow::from("some other inner val")),
            ],
        );

        let mut outer = BTreeMap::new();
        outer.insert(Cow::from("outer"), vec![Value::Obj(Vdf(outer_val))]);

        Vdf(outer)
    };

    assert_eq!(Vdf::try_from(&naive_token_stream), Ok(ideal));
}

#[test]
fn invalid_vdf_nested_seq() {
    let naive_token_stream = NaiveTokenStream(vec![
        NaiveToken::str("outer"),
        NaiveToken::ObjBegin,
        NaiveToken::str("nested sequence"),
        NaiveToken::SeqBegin,
        NaiveToken::str("the calm before the storm"),
        NaiveToken::SeqBegin,
        NaiveToken::SeqEnd,
        NaiveToken::SeqEnd,
        NaiveToken::ObjEnd,
    ]);

    assert!(Vdf::try_from(&naive_token_stream).is_err());
}

#[test]
fn invalid_vdf_obj_key() {
    let naive_token_stream = NaiveTokenStream(vec![
        NaiveToken::str("outer"),
        NaiveToken::ObjBegin,
        NaiveToken::ObjBegin,
        NaiveToken::ObjEnd,
        NaiveToken::ObjEnd,
    ]);

    assert!(Vdf::try_from(&naive_token_stream).is_err());
}

#[test]
fn invalid_vdf_seq_key() {
    let naive_token_stream = NaiveTokenStream(vec![
        NaiveToken::str("outer"),
        NaiveToken::ObjBegin,
        NaiveToken::SeqBegin,
        NaiveToken::SeqEnd,
        NaiveToken::ObjEnd,
    ]);

    assert!(Vdf::try_from(&naive_token_stream).is_err());
}

#[test]
fn token_stream_from_vdf() {
    let s = r#"
"Outer Key" "Outer Value"
"Outer Key"
{
    "Inner Key" "Inner Value"
}
        "#;
    let vdf = Vdf::parse(s).unwrap();
    let token_stream = TokenStream::from(vdf);
    assert_eq!(
        token_stream,
        TokenStream(vec![
            Token::Key(Cow::from("Outer Key")),
            Token::SeqBegin,
            Token::Str(Cow::from("Outer Value")),
            Token::ObjBegin,
            Token::Key(Cow::from("Inner Key")),
            Token::Str(Cow::from("Inner Value")),
            Token::ObjEnd,
            Token::SeqEnd,
        ])
    );
}
