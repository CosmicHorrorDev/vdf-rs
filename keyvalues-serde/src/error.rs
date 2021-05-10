use serde::{de, ser};

use std::fmt::Display;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Clone, Debug, PartialEq)]
pub enum Error {
    #[error("Just for developing")]
    Placeholder,
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        println!("{}", msg);
        todo!()
    }
}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        todo!()
    }
}
