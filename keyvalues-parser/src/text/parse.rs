use crate::{error::Result, PartialVdf as Vdf};

pub use crate::text::new_parse::parse as escaped_parse;

impl<'a> Vdf<'a> {
    /// Attempts to parse VDF text to a [`Vdf`]
    pub fn parse(s: &'a str) -> Result<Self> {
        escaped_parse(s)
    }

    pub fn parse_raw(s: &'a str) -> Result<Self> {
        raw_parse(s)
    }
}

impl<'a> crate::Vdf<'a> {
    /// Attempts to parse VDF text to a [`Vdf`]
    pub fn parse(s: &'a str) -> Result<Self> {
        Ok(crate::Vdf::from(Vdf::parse(s)?))
    }

    pub fn parse_raw(s: &'a str) -> Result<Self> {
        Ok(crate::Vdf::from(Vdf::parse_raw(s)?))
    }
}

pub fn raw_parse(s: &str) -> Result<Vdf> {
    crate::text::new_parse::parse_(s, false)
}
