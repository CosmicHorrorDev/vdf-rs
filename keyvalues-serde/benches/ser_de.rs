use std::{fs, hint::black_box, path::Path};

use divan::{bench, counter::BytesCount, Bencher};
use keyvalues_serde::{from_str, to_string};
use serde::{Deserialize, Serialize};

mod types;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

fn read_app_info() -> Result<String, std::io::Error> {
    let vdf_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("assets")
        .join("app_info.vdf");
    fs::read_to_string(vdf_path)
}

fn from_str_helper<'de, T>(s: &'de str) -> T
where
    T: Deserialize<'de>,
{
    from_str(black_box(s)).unwrap()
}

fn to_string_helper<T>(t: &T) -> String
where
    T: Serialize,
{
    to_string(black_box(t)).unwrap()
}

fn setup_deserialize() -> (String, BytesCount) {
    let vdf_text = read_app_info().unwrap();
    let bytes = BytesCount::of_str(&vdf_text);
    (vdf_text, bytes)
}

#[bench]
pub fn deserialize_all_owned(bencher: Bencher) {
    let (vdf_text, bytes) = setup_deserialize();
    bencher
        .counter(bytes)
        .bench(|| from_str_helper::<types::AppInfo>(black_box(&vdf_text)))
}

#[bench]
pub fn deserialize_all_borrowed(bencher: Bencher) {
    let (vdf_text, bytes) = setup_deserialize();
    bencher
        .counter(bytes)
        .bench(|| from_str_helper::<types::AppInfoBorrow>(black_box(&vdf_text)))
}

#[bench]
pub fn deserialize_extract_single(bencher: Bencher) {
    let (vdf_text, bytes) = setup_deserialize();
    bencher
        .counter(bytes)
        .bench(|| from_str_helper::<types::AppInfoExtract>(black_box(&vdf_text)))
}

// It doesn't really make sense to reserialize anything other than the full content
#[bench]
pub fn serialize(bencher: Bencher) {
    let vdf_text = read_app_info().unwrap();
    let app_info_all: types::AppInfo = from_str_helper(&vdf_text);

    let serialized = to_string_helper::<types::AppInfo>(&app_info_all);
    let bytes = BytesCount::of_str(&serialized);

    bencher
        .counter(bytes)
        .bench(|| to_string_helper::<types::AppInfo>(&app_info_all))
}
