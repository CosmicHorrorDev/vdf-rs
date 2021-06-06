use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use keyvalues_parser::Vdf;

use std::{fs, path::Path};

fn read_app_info() -> Result<String, std::io::Error> {
    let vdf_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("assets")
        .join("app_info.vdf");
    fs::read_to_string(vdf_path)
}

pub fn parse_throughput(c: &mut Criterion) {
    let vdf_text = read_app_info().unwrap();

    let mut group = c.benchmark_group("parse throughput");
    group.throughput(Throughput::Bytes(vdf_text.len() as u64));
    group.bench_function("parse", |b| b.iter(|| Vdf::parse(black_box(&vdf_text))));
    group.finish();
}

pub fn render_throughput(c: &mut Criterion) {
    let vdf_text = read_app_info().unwrap();
    let vdf = Vdf::parse(&vdf_text).unwrap();

    let mut group = c.benchmark_group("render throughput");
    group.throughput(Throughput::Bytes(vdf_text.len() as u64));
    group.bench_function("render", |b| b.iter(|| vdf.to_string()));
    group.finish();
}

criterion_group!(throughput, parse_throughput, render_throughput);
criterion_main!(throughput);
