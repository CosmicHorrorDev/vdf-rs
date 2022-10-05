#![doc = include_str!("../README.md")]
#![allow(unknown_lints)]
#![allow(clippy::result_large_err)]
// TODO: resolve this ^^

pub mod de;
pub mod error;
pub mod ser;

// The tokenstreams are fuzzed even though they are not exposed publicly
#[cfg(fuzzing)]
pub mod tokens;
#[cfg(not(fuzzing))]
mod tokens;

pub use keyvalues_parser as parser;

#[doc(inline)]
pub use de::{
    from_reader, from_reader_with_key, from_str, from_str_with_key, from_vdf, from_vdf_with_key,
    Deserializer,
};
#[doc(inline)]
pub use error::{Error, Result};
#[doc(inline)]
pub use ser::{to_string, to_string_with_key, to_writer, to_writer_with_key, Serializer};
