use crate::text::parse::Error as ParseError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Clone, Debug, PartialEq)]
pub enum Error {
    #[error("Failed parsing input")]
    ParseError(#[from] ParseError),
    #[error("Invalid token stream: {0:?}")]
    InvalidTokenStream(TokenContext),
}

impl From<TokenContext> for Error {
    fn from(context: TokenContext) -> Self {
        Self::InvalidTokenStream(context)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenContext {
    EofWhileParsingKey,
    EofWhileParsingVal,
    EofWhileParsingSeq,
    EofWhileParsingObj,
    ExpectedSomeVal,
    ExpectedNonSeqVal,
    TrailingTokens,
}
