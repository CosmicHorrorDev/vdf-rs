use crate::utils::Container;

use keyvalues_serde::{to_string, Error};
use serde::Serialize;

macro_rules! gen_tests {
    ( $( ( $test_name:ident, $to_ser:expr, $err_msg:expr) ),* $(,)? ) => {
        $(
            #[test]
            fn $test_name() {
                let err = to_string(&Container::new($to_ser)).unwrap_err();
                dbg!(&err);
                assert!(matches!(err, Error::Unsupported($err_msg)));
            }
        )*
    };
}

#[derive(Serialize)]
struct Unit;

#[derive(Serialize)]
enum Enum {
    Newtype(bool),
    Tuple(bool, u8),
    Struct { a: bool },
}

gen_tests!(
    (unit, (), "Unit Type"),
    (unit_struct, Unit, "Unit Struct"),
    (enum_newtype, Enum::Newtype(false), "Enum Newtype Variant"),
    (enum_tuple, Enum::Tuple(false, 0), "Enum Tuple Variant"),
    (
        enum_struct,
        Enum::Struct { a: false },
        "Enum Struct Variant"
    ),
);
