use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use keyvalues_serde::{from_str, to_string};
use serde::{Deserialize, Serialize};

use std::{fs, path::Path};

mod types;

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

pub fn de(c: &mut Criterion) {
    let vdf_text = read_app_info().unwrap();

    let mut group = c.benchmark_group("de");
    group.throughput(Throughput::Bytes(vdf_text.len() as u64));
    group.bench_function("all owned", |b| {
        b.iter(|| from_str_helper::<types::AppInfo>(&vdf_text))
    });
    group.bench_function("all borrowed", |b| {
        b.iter(|| from_str_helper::<types::AppInfoBorrow>(&vdf_text))
    });
    group.bench_function("extract single", |b| {
        b.iter(|| from_str_helper::<types::AppInfoExtract>(&vdf_text))
    });
    group.finish();
}

// It doesn't really make sense to reserialize just the extracted content
pub fn ser(c: &mut Criterion) {
    let vdf_text = read_app_info().unwrap();
    let app_info_all: types::AppInfo = from_str_helper(&vdf_text);
    let ser_len = to_string_helper::<types::AppInfo>(&app_info_all).len();

    let mut group = c.benchmark_group("ser");
    group.throughput(Throughput::Bytes(ser_len as u64));
    group.bench_function("all", |b| {
        b.iter(|| to_string_helper::<types::AppInfo>(&app_info_all))
    });
    group.finish();
}

criterion_group!(throughput, de, ser);
criterion_main!(throughput);
