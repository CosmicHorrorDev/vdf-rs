use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use keyvalues_parser::core::Vdf;

use std::{fs, path::Path};

fn read_app_info() -> Result<String, std::io::Error> {
    let vdf_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("assets")
        .join("app_info.vdf");
    fs::read_to_string(vdf_path)
}

pub fn parse_time(c: &mut Criterion) {
    let vdf_text = read_app_info().unwrap();

    c.bench_function("parse app info", |b| {
        b.iter(|| Vdf::parse(black_box(&vdf_text)))
    });
}

pub fn parse_throughput(c: &mut Criterion) {
    let vdf_text = read_app_info().unwrap();

    let mut group = c.benchmark_group("parse throughput");
    group.throughput(Throughput::Bytes(vdf_text.len() as u64));
    group.bench_function("parse", |b| b.iter(|| Vdf::parse(black_box(&vdf_text))));
    group.finish();
}

criterion_group!(timings, parse_time);
criterion_group!(throughput, parse_throughput);
criterion_main!(timings, throughput);
