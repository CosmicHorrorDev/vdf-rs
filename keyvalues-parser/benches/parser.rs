use std::{hint::black_box, time::Duration};

use divan::{bench, counter::BytesCount, Bencher, Divan};
use keyvalues_parser::Vdf;

fn main() {
    // Run registered benchmarks
    Divan::from_args()
        .min_time(Duration::from_millis(200))
        .main();
}

static APP_INFO: &str = include_str!("../tests/assets/app_info.vdf");

#[bench(bytes_count = APP_INFO.len())]
pub fn parse() {
    Vdf::parse(black_box(APP_INFO)).unwrap();
}

#[bench]
pub fn render(bencher: Bencher) {
    let vdf = Vdf::parse(APP_INFO).unwrap();
    let rendered = vdf.to_string();
    let bytes = BytesCount::of_str(&rendered);

    bencher.counter(bytes).bench(|| vdf.to_string())
}
