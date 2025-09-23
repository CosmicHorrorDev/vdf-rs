use std::{hint::black_box, time::Duration};

use divan::{bench, counter::BytesCount, Bencher, Divan};
use keyvalues_serde::{from_str, to_string};
use serde::{Deserialize, Serialize};

mod types;

fn main() {
    // Run registered benchmarks.
    Divan::default()
        .min_time(Duration::from_millis(200))
        .config_with_args()
        .main();
}

fn expect_str<T>(s: &'static str) -> T
where
    T: Deserialize<'static>,
{
    from_str(s).unwrap()
}

fn expect_to_string<T>(t: &T) -> String
where
    T: Serialize,
{
    to_string(t).unwrap()
}

static VDF_TEXT: &str = include_str!("assets/controller_generic_wasd.vdf");

#[bench(
    bytes_count = VDF_TEXT.len(),
    types = [types::FullStructOwned, types::FullStructBorrowed, types::SingleField],
)]
pub fn deserialize<T>() -> T
where
    T: Deserialize<'static>,
{
    expect_str(black_box(VDF_TEXT))
}

// It doesn't really make sense to reserialize `SingleField`
#[bench(types = [types::FullStructOwned, types::FullStructBorrowed])]
pub fn serialize<T>(bencher: Bencher)
where
    T: Deserialize<'static> + Serialize + Sync,
{
    let app_info_all: T = expect_str(VDF_TEXT);

    let serialized = expect_to_string(&app_info_all);
    let bytes = BytesCount::of_str(&serialized);

    bencher
        .counter(bytes)
        .bench(|| expect_to_string(black_box(&app_info_all)))
}
