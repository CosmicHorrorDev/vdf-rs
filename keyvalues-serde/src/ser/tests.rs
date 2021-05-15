use keyvalues_parser::tokens::Token;

use std::borrow::Cow;

use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
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

        assert_eq!(
            TokenStream::try_from(&naive_token_stream),
            Ok(TokenStream(vec![
                Token::Key(Cow::from("outer")),
                Token::ObjBegin,
                Token::Key(Cow::from("sequence start")),
                Token::SeqBegin,
                Token::ObjBegin,
                Token::Key(Cow::from("inner key")),
                Token::Str(Cow::from("inner val")),
                Token::ObjEnd,
                Token::Str(Cow::from("some other inner val")),
                Token::SeqEnd,
                Token::ObjEnd,
            ]))
        );
    }

    #[test]
    fn invalid_nested_seq() {
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

        assert!(TokenStream::try_from(&naive_token_stream).is_err());
    }

    #[test]
    fn invalid_obj_key() {
        let naive_token_stream = NaiveTokenStream(vec![
            NaiveToken::str("outer"),
            NaiveToken::ObjBegin,
            NaiveToken::ObjBegin,
            NaiveToken::ObjEnd,
            NaiveToken::ObjEnd,
        ]);

        assert!(TokenStream::try_from(&naive_token_stream).is_err());
    }

    #[test]
    fn invalid_seq_key() {
        let naive_token_stream = NaiveTokenStream(vec![
            NaiveToken::str("outer"),
            NaiveToken::ObjBegin,
            NaiveToken::SeqBegin,
            NaiveToken::SeqEnd,
            NaiveToken::ObjEnd,
        ]);

        assert!(TokenStream::try_from(&naive_token_stream).is_err());
    }
}
