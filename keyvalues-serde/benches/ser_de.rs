use std::{hint::black_box, time::Duration};

use divan::{bench, counter::BytesCount, Bencher, Divan};
use keyvalues_serde::{from_str, to_string};
use serde::{Deserialize, Serialize};

mod types;

fn main() {
    // Run registered benchmarks.
    Divan::from_args()
        .min_time(Duration::from_millis(200))
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

static APP_INFO: &str = include_str!("../tests/assets/app_info.vdf");

#[bench(
    name = "deserialize as type",
    bytes_count = APP_INFO.len(),
    types = [types::AppInfoFullOwned, types::AppInfoFullBorrowed<'static>, types::AppInfoSingleNested],
)]
pub fn deserialize_as_type<T>() -> T
where
    T: Deserialize<'static>,
{
    expect_str::<T>(black_box(APP_INFO))
}

// It doesn't really make sense to reserialize anything other than the full content
#[bench(name = "serialize as AppInfo")]
pub fn serialize_as_app_info(bencher: Bencher) {
    let app_info_all: types::AppInfoFullOwned = expect_str(APP_INFO);

    let serialized = expect_to_string::<types::AppInfoFullOwned>(&app_info_all);
    let bytes = BytesCount::of_str(&serialized);

    bencher
        .counter(bytes)
        .bench(|| expect_to_string::<types::AppInfoFullOwned>(black_box(&app_info_all)))
}
