use std::{hint::black_box, time::Duration};

use divan::{bench, counter::BytesCount, Bencher, Divan};
use keyvalues_parser::Vdf;

fn main() {
    // Run registered benchmarks
    Divan::from_args()
        .min_time(Duration::from_millis(200))
        .main();
}

static VDF_TEXT: &str = include_str!("../tests/assets/controller_generic_wasd.vdf");

#[bench(bytes_count = VDF_TEXT.len())]
pub fn parse() {
    Vdf::parse(black_box(VDF_TEXT)).unwrap();
}

#[bench]
pub fn render(bencher: Bencher) {
    let vdf = Vdf::parse(VDF_TEXT).unwrap();
    let rendered = vdf.to_string();
    let bytes = BytesCount::of_str(&rendered);

    bencher.counter(bytes).bench(|| vdf.to_string())
}
