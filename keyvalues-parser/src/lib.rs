//! `keyvalues-parser` uses [`pest`](https://lib.rs/crates/pest) to parse
//! [VDF text v1 and v2](https://developer.valvesoftware.com/wiki/KeyValues)
//! files to an untyped Rust structure to ease manipulation and navigation. The
//! parser provides an untyped `Vdf` representation as well as a linear
//! `TokenStream`
//!
//! The library is primarily used in conjunction with
//! [`keyvalues-serde`](https://github.com/LovecraftianHorror/vdf-rs/tree/main/keyvalues-serde)
//! which provides a more ergonommic (yet more limiting) means of dealing with VDF
//! text
//!
//! # Installation
//!
//! **Note: this requires at least Rust `1.42.0`**
//!
//! Just add the library to your `Cargo.toml`
//!
//! ```toml
//! [dependencies]
//! keyvalues-parser = "0.1.0"
//! ```
//!
//! # Usage
//!
//! There is documentation available
//! [here](https://docs.rs/keyvalues-parser/0.1.0/keyvalues_parser/) and there are
//! examples available in the
//! [examples directory](https://github.com/LovecraftianHorror/vdf-rs/tree/main/keyvalues-parser/examples)
//!
//! ## Quickstart
//!
//! `loginusers.vdf`
//!
//! ```vdf
//! "users"
//! {
//!     "12345678901234567"
//!     {
//!         "AccountName"        "ACCOUNT_NAME"
//!         "PersonaName"        "PERSONA_NAME"
//!         "RememberPassword"    "1"
//!         "MostRecent"        "1"
//!         "Timestamp"        "1234567890"
//!     }
//! }
//! ```
//!
//! `main.rs`
//!
//! ```no_run
//! use keyvalues_parser::Vdf;
//!
//! let vdf_text = std::fs::read_to_string("loginusers.vdf")?;
//! let vdf = Vdf::parse(&vdf_text)?;
//! assert_eq!(
//!     "12345678901234567",
//!     vdf.value.unwrap_obj().keys().next().unwrap()
//! );
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! # Limitations
//!
//! VDF text is drastically underspecified. This leads to the following liberties
//! being taken
//!
//! - Not respecting the ordering of key-value pairs, where the pairs are stored in a `BTreeMap` that sorts the values based on the key
//! - Because of limitations in representing sequences, an empty `Vec` of values will be rendered as a missing keyvalue pair
//!
//! # Benchmarks
//!
//! A set of basic benchmarks can be found in the
//! [benches directory](https://github.com/LovecraftianHorror/vdf-rs/tree/main/keyvalues-parser/benches)
//!
//! These just test timing and throughput for both parsing and rendering of a
//! fairly typical VDF file

use std::{
    borrow::Cow,
    collections::{btree_map::IntoIter, BTreeMap},
    iter::FromIterator,
    ops::{Deref, DerefMut},
};

pub mod error;
#[cfg(test)]
mod tests;
pub mod text;

/// A Key is simply an alias for `Cow<str>`
pub type Key<'a> = Cow<'a, str>;

/// A loosely typed representation of VDF text
///
/// `Vdf` is represented as a single [`Key`][Key] mapped to a single [`Value`][Value]
///
/// ## Parse
///
/// `Vdf`s will generally be created through the use of [`Vdf::parse()`][Vdf::parse] which takes a
/// string representing VDF text and attempts to parse it to a `Vdf` representation.
///
/// ## Mutate
///
/// From there you can manipulate/extract from the representation as desired by using the standard
/// conventions on the internal types (plain old `BTreeMap`s, `Vec`s, and `Cow`s all the way down)
///
/// ## Render
///
/// The `Vdf` can also be rendered back to its text form through its `Display` implementation
///
/// ## Example
///
/// ```
/// use keyvalues_parser::Vdf;
///
/// // Parse
/// let vdf_text = r#"
/// "Outer Key"
/// {
///     "Inner Key" "Inner Value"
///     "Inner Key"
///     {
///     }
/// }
/// "#;
/// let mut parsed = Vdf::parse(vdf_text)?;
///
/// // Mutate: i.e. remove the last "Inner Key" pair
/// parsed
///     .value
///     .get_mut_obj()
///     .unwrap()
///     .get_mut("Inner Key")
///     .unwrap()
///     .pop();
///
/// // Render: prints
/// // "Outer Key"
/// // {
/// //     "Inner Key" "Inner Value"
/// // }
/// println!("{}", parsed);
/// # Ok::<(), keyvalues_parser::error::Error>(())
/// ```
#[cfg_attr(test, derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Vdf<'a> {
    pub key: Key<'a>,
    pub value: Value<'a>,
}

impl<'a> From<PartialVdf<'a>> for Vdf<'a> {
    fn from(partial: PartialVdf<'a>) -> Self {
        Self {
            key: partial.key,
            value: partial.value,
        }
    }
}

#[cfg_attr(test, derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PartialVdf<'a> {
    pub key: Key<'a>,
    pub value: Value<'a>,
    pub bases: Vec<Cow<'a, str>>,
}

impl<'a> Vdf<'a> {
    /// Creates a [`Vdf`][Vdf] using a provided key and value
    ///
    /// ```
    /// use keyvalues_parser::{Vdf, Value};
    /// use std::borrow::Cow;
    ///
    /// let vdf = Vdf::new(Cow::from("Much Key"), Value::Str(Cow::from("Such Wow")));
    /// // prints
    /// // "Much Key"  "Such Wow"
    /// println!("{}", vdf);
    /// ```
    pub fn new(key: Key<'a>, value: Value<'a>) -> Self {
        Self { key, value }
    }
}

type ObjInner<'a> = BTreeMap<Key<'a>, Vec<Value<'a>>>;
type ObjInnerPair<'a> = (Key<'a>, Vec<Value<'a>>);

#[cfg_attr(test, derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Clone, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Obj<'a>(ObjInner<'a>);

impl<'a> Obj<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn into_inner(self) -> ObjInner<'a> {
        self.0
    }

    pub fn into_vdfs(self) -> IntoVdfs<'a> {
        IntoVdfs::new(self)
    }
}

impl<'a> FromIterator<ObjInnerPair<'a>> for Obj<'a> {
    fn from_iter<T: IntoIterator<Item = ObjInnerPair<'a>>>(iter: T) -> Self {
        let mut inner = BTreeMap::new();
        for (key, values) in iter {
            inner.insert(key, values);
        }

        Self(inner)
    }
}

impl<'a> Deref for Obj<'a> {
    type Target = ObjInner<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for Obj<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct IntoVdfs<'a> {
    current_entry: Option<ObjInnerPair<'a>>,
    it: IntoIter<Key<'a>, Vec<Value<'a>>>,
}

impl<'a> IntoVdfs<'a> {
    fn new(obj: Obj<'a>) -> Self {
        Self {
            current_entry: None,
            it: obj.into_inner().into_iter(),
        }
    }
}

impl<'a> Iterator for IntoVdfs<'a> {
    type Item = Vdf<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        // Iteration will pop the first pair off `current_entry` if it's set and then falls back to
        // reading in a new `current_entry` from `it`. If `it` is exhausted then we're done
        loop {
            match self.current_entry.take() {
                // There is a pair to return
                Some((key, mut values)) if !values.is_empty() => {
                    let value = values.pop().expect("values isn't empty");
                    self.current_entry = Some((key.clone(), values));
                    return Some(Vdf::new(key, value));
                }
                _ => match self.it.next() {
                    // Store the next entry. Flip the values so that `pop`ing returns correct order
                    Some((key, values)) => {
                        self.current_entry = Some((key, values.into_iter().rev().collect()));
                    }
                    // Fin
                    None => {
                        return None;
                    }
                },
            }
        }
    }
}

/// Enum representing all valid VDF values
///
/// VDF is composed of [`Key`s][Key] and their respective [`Value`s][Value] where this represents
/// the latter. A value is either going to be a `Str(Cow<str>)`, or an `Obj(Obj)` that contains a
/// list of keys and values.
#[cfg_attr(test, derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Value<'a> {
    Str(Cow<'a, str>),
    Obj(Obj<'a>),
}

impl<'a> Value<'a> {
    /// Returns if the current value is the `Str` variant
    pub fn is_str(&self) -> bool {
        self.get_str().is_some()
    }

    /// Returns if the current value is the `Obj` variant
    pub fn is_obj(&self) -> bool {
        self.get_obj().is_some()
    }

    /// Gets the inner `&str` if this is a `Value::Str`
    pub fn get_str(&self) -> Option<&str> {
        if let Self::Str(s) = self {
            Some(s)
        } else {
            None
        }
    }

    /// Gets the inner `&Obj` if this value is a `Value::Obj`
    pub fn get_obj(&self) -> Option<&Obj> {
        if let Self::Obj(obj) = self {
            Some(obj)
        } else {
            None
        }
    }

    /// Gets the inner `&mut str` if this is a `Value::Str`
    pub fn get_mut_str(&mut self) -> Option<&mut Cow<'a, str>> {
        if let Self::Str(s) = self {
            Some(s)
        } else {
            None
        }
    }

    /// Gets the inner `&mut Obj` if this is a `Value::Obj`
    pub fn get_mut_obj(&mut self) -> Option<&mut Obj<'a>> {
        if let Self::Obj(obj) = self {
            Some(obj)
        } else {
            None
        }
    }

    /// Unwraps the `Cow<str>` from the `Value::Str`
    ///
    /// # Panics
    ///
    /// If the variant was `Value::Obj`
    ///
    /// # Examples
    ///
    /// ```
    /// use keyvalues_parser::Value;
    /// use std::borrow::Cow;
    ///
    /// let value = Value::Str(Cow::from("Sample text"));
    /// assert_eq!(value.unwrap_str(), "Sample text");
    /// ```
    ///
    /// ```should_panic
    /// use keyvalues_parser::{Value, Obj};
    ///
    /// let value = Value::Obj(Obj::new());
    /// value.unwrap_str(); // <-- panics
    /// ```
    pub fn unwrap_str(self) -> Cow<'a, str> {
        self.expect_str("Called `unwrap_str` on a `Value::Obj` variant")
    }

    /// Unwraps the [`Obj`][Obj] from the `Value::Obj`
    ///
    /// # Panics
    ///
    /// If the variant was `Value::Str`
    ///
    /// # Examples
    ///
    /// ```
    /// use keyvalues_parser::{Obj, Value};
    ///
    /// let value = Value::Obj(Obj::new());
    /// assert_eq!(value.unwrap_obj(), Obj::new());
    /// ```
    ///
    /// ```should_panic
    /// use keyvalues_parser::Value;
    /// use std::borrow::Cow;
    ///
    /// let value = Value::Str(Cow::from("D'Oh"));
    /// value.unwrap_obj(); // <-- panics
    /// ```
    pub fn unwrap_obj(self) -> Obj<'a> {
        self.expect_obj("Called `unwrap_obj` on a `Value::Str` variant")
    }

    /// Refer to [Value::unwrap_str]. Same situation, but with a custom message
    pub fn expect_str(self, msg: &str) -> Cow<'a, str> {
        if let Self::Str(s) = self {
            s
        } else {
            panic!("{}", msg)
        }
    }

    /// Refer to [Value::unwrap_obj]. Same situation, but with a custom message
    pub fn expect_obj(self, msg: &str) -> Obj<'a> {
        if let Self::Obj(obj) = self {
            obj
        } else {
            panic!("{}", msg)
        }
    }
}
