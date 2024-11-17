#![doc = include_str!("../README.md")]
#![allow(unknown_lints)]
#![allow(clippy::result_large_err)]
// TODO: resolve this ^^

use std::{
    borrow::Cow,
    collections::{btree_map::IntoIter, BTreeMap},
    ops::{Deref, DerefMut},
};

pub mod error;
pub mod text;

/// A Key is simply an alias for `Cow<str>`
pub type Key<'text> = Cow<'text, str>;

/// A loosely typed representation of VDF text
///
/// `Vdf` is represented as a single [`Key`] mapped to a single [`Value`]
///
/// ## Parse
///
/// `Vdf`s will generally be created through the use of [`Vdf::parse()`] which takes a string
/// representing VDF text and attempts to parse it to a `Vdf` representation.
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
/// # Ok::<(), keyvalues_parser::error::ParseError>(())
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Vdf<'text> {
    pub key: Key<'text>,
    pub value: Value<'text>,
}

impl<'text> From<PartialVdf<'text>> for Vdf<'text> {
    fn from(partial: PartialVdf<'text>) -> Self {
        Self {
            key: partial.key,
            value: partial.value,
        }
    }
}

// TODO: Just store a `Vdf` internally?
// TODO: don't expose these publicly?
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PartialVdf<'text> {
    pub bases: Vec<Cow<'text, str>>,
    pub key: Key<'text>,
    pub value: Value<'text>,
}

impl<'text> Vdf<'text> {
    /// Creates a [`Vdf`] using a provided key and value
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
    pub fn new(key: Key<'text>, value: Value<'text>) -> Self {
        Self { key, value }
    }
}

// TODO: why is this type alias a thing if it's not private but the usage of it inside `Obj` is?
type ObjInner<'text> = BTreeMap<Key<'text>, Vec<Value<'text>>>;
type ObjInnerPair<'text> = (Key<'text>, Vec<Value<'text>>);

#[derive(Debug, Clone, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Obj<'text>(pub ObjInner<'text>);

impl<'text> Obj<'text> {
    /// Creates an empty object value
    ///
    /// Internally This is just a [`BTreeMap`] that maps [`Key`]s to a [`Vec`] of [`Value`]s
    ///
    /// ```
    /// # use keyvalues_parser::{Obj, Value};
    /// # use std::borrow::Cow;
    /// let mut obj = Obj::new();
    /// obj.insert(
    ///     Cow::from("key"),
    ///     vec![]
    /// );
    /// obj.insert(
    ///     Cow::from("earlier key"),
    ///     vec![Value::Obj(Obj::default())]
    /// );
    ///
    /// // It's a b-tree map so the entries are sorted by keys
    /// assert_eq!(
    ///     obj.keys().collect::<Vec<_>>(),
    ///     ["earlier key", "key"]
    /// );
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the inner [`BTreeMap`]
    ///
    /// ```
    /// # use keyvalues_parser::{Obj, Value};
    /// # use std::{borrow::Cow, collections::BTreeMap};
    /// let mut obj = Obj::new();
    /// obj.insert(Cow::from("much key"), vec![]);
    ///
    /// let inner: BTreeMap<_, _> = obj.into_inner();
    /// // Prints:
    /// // {
    /// //     "much key": [],
    /// // }
    /// println!("{:#?}", inner);
    /// ```
    pub fn into_inner(self) -> ObjInner<'text> {
        self.0
    }

    /// Creates an iterator that returns the [`Vdf`]s that compose the object
    ///
    /// This is notably different compared to just iterating over the `BTreeMap`s items because it
    /// will emit a [`Vdf`] for each key-value pair while the actual items are key-values pairs.
    /// This means that empty values will not emit a [`Vdf`] at all, and a pair that has multiple
    /// entries in values will emit a [`Vdf`] for each pairing
    ///
    /// ```
    /// # use keyvalues_parser::{Obj, Value, Vdf};
    /// # use std::borrow::Cow;
    /// let mut obj = Obj::new();
    /// obj.insert(
    ///     Cow::from("no values"),
    ///     vec![]
    /// );
    /// obj.insert(
    ///     Cow::from("multiple values"),
    ///     vec![Value::Str(Cow::from("first")), Value::Str(Cow::from("second"))]
    /// );
    ///
    /// let vdfs: Vec<_> = obj.into_vdfs().collect();
    /// assert_eq!(
    ///     vdfs,
    ///     [
    ///         Vdf {
    ///             key: Cow::from("multiple values"),
    ///             value: Value::Str(Cow::from("first"))
    ///         },
    ///         Vdf {
    ///             key: Cow::from("multiple values"),
    ///             value: Value::Str(Cow::from("second"))
    ///         },
    ///     ]
    /// );
    /// ```
    pub fn into_vdfs(self) -> IntoVdfs<'text> {
        IntoVdfs::new(self)
    }
}

impl<'text> FromIterator<ObjInnerPair<'text>> for Obj<'text> {
    fn from_iter<T: IntoIterator<Item = ObjInnerPair<'text>>>(iter: T) -> Self {
        let mut inner = BTreeMap::new();
        for (key, values) in iter {
            inner.insert(key, values);
        }

        Self(inner)
    }
}

impl<'text> Deref for Obj<'text> {
    type Target = ObjInner<'text>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Obj<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// An iterator over an [`Obj`]'s [`Vdf`] pairs
///
/// Typically created by calling [`Obj::into_vdfs`] on an existing object
pub struct IntoVdfs<'text> {
    // TODO: can this just store an iterator for the values instead of `.collect()`ing
    current_entry: Option<ObjInnerPair<'text>>,
    it: IntoIter<Key<'text>, Vec<Value<'text>>>,
}

impl<'text> IntoVdfs<'text> {
    fn new(obj: Obj<'text>) -> Self {
        Self {
            current_entry: None,
            it: obj.into_inner().into_iter(),
        }
    }
}

impl<'text> Iterator for IntoVdfs<'text> {
    type Item = Vdf<'text>;

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
                _ => {
                    let (key, values) = self.it.next()?;
                    // Store the next entry. Flip the values so that `pop`ing returns correct order
                    self.current_entry = Some((key, values.into_iter().rev().collect()));
                }
            }
        }
    }
}

/// Enum representing all valid VDF values
///
/// VDF is composed of [`Key`]s and their respective [`Value`]s where this represents the latter. A
/// value is either going to be a `Str(Cow<str>)`, or an `Obj(Obj)` that contains a list of keys
/// and values.
///
/// ```
/// # use keyvalues_parser::{Obj, Value};
/// # use std::borrow::Cow;
/// let value_str = Value::Str(Cow::from("some text"));
/// let value_obj = Value::Obj(Obj::new());
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Value<'text> {
    Str(Cow<'text, str>),
    Obj(Obj<'text>),
}

impl<'text> Value<'text> {
    /// Returns if the current value is the `Str` variant
    ///
    /// ```
    /// use std::borrow::Cow;
    /// use keyvalues_parser::{Obj, Value};
    ///
    /// let value_str = Value::Str(Cow::default());
    /// assert!(value_str.is_str());
    /// ```
    pub fn is_str(&self) -> bool {
        self.get_str().is_some()
    }

    /// Returns if the current value is the `Obj` variant
    ///
    /// ```
    /// use keyvalues_parser::{Obj, Value};
    ///
    /// let value_obj = Value::Obj(Obj::default());
    /// assert!(value_obj.is_obj());
    /// ```
    pub fn is_obj(&self) -> bool {
        self.get_obj().is_some()
    }

    /// Gets the inner `&str` if this is a `Value::Str`
    ///
    /// ```
    /// # use keyvalues_parser::Value;
    /// # use std::borrow::Cow;
    /// let value = Value::Str(Cow::from("some text"));
    ///
    /// if let Some(s) = value.get_str() {
    ///     println!("value str: {}", s);
    /// }
    /// ```
    pub fn get_str(&self) -> Option<&str> {
        if let Self::Str(s) = self {
            Some(s)
        } else {
            None
        }
    }

    /// Gets the inner `&Obj` if this value is a `Value::Obj`
    ///
    /// ```
    /// # use keyvalues_parser::{Obj, Value};
    /// let value = Value::Obj(Obj::new());
    ///
    /// if let Some(obj) = value.get_obj() {
    ///     println!("value obj: {:?}", obj);
    /// }
    /// ```
    pub fn get_obj(&self) -> Option<&Obj> {
        if let Self::Obj(obj) = self {
            Some(obj)
        } else {
            None
        }
    }

    /// Gets the inner `&mut str` if this is a `Value::Str`
    ///
    /// ```
    /// # use keyvalues_parser::Value;
    /// # use std::borrow::Cow;
    /// let mut value = Value::Str(Cow::from("some text"));
    /// let mut inner_str = value.get_mut_str().unwrap();
    /// inner_str.to_mut().make_ascii_uppercase();
    ///
    /// assert_eq!(
    ///     value,
    ///     Value::Str(Cow::from("SOME TEXT"))
    /// );
    /// ```
    pub fn get_mut_str(&mut self) -> Option<&mut Cow<'text, str>> {
        if let Self::Str(s) = self {
            Some(s)
        } else {
            None
        }
    }

    /// Gets the inner `&mut Obj` if this is a `Value::Obj`
    ///
    /// ```
    /// # use keyvalues_parser::{Obj, Value};
    /// # use std::borrow::Cow;
    /// let mut value = Value::Obj(Obj::new());
    /// let mut inner_obj = value.get_mut_obj().unwrap();
    /// inner_obj.insert(Cow::from("new key"), vec![]);
    ///
    /// // Prints:
    /// // Value::Obj({
    /// //    "new key": [],
    /// // })
    /// println!("{:?}", value);
    /// ```
    pub fn get_mut_obj(&mut self) -> Option<&mut Obj<'text>> {
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
    pub fn unwrap_str(self) -> Cow<'text, str> {
        self.expect_str("Called `unwrap_str` on a `Value::Obj` variant")
    }

    /// Unwraps the [`Obj`] from the `Value::Obj`
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
    pub fn unwrap_obj(self) -> Obj<'text> {
        self.expect_obj("Called `unwrap_obj` on a `Value::Str` variant")
    }

    /// Refer to [Value::unwrap_str]. Same situation, but with a custom message
    pub fn expect_str(self, msg: &str) -> Cow<'text, str> {
        if let Self::Str(s) = self {
            s
        } else {
            panic!("{}", msg)
        }
    }

    /// Refer to [Value::unwrap_obj]. Same situation, but with a custom message
    pub fn expect_obj(self, msg: &str) -> Obj<'text> {
        if let Self::Obj(obj) = self {
            obj
        } else {
            panic!("{}", msg)
        }
    }
}
