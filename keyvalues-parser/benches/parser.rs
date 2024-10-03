use std::{fs, hint::black_box, path::Path};

use divan::{bench, counter::BytesCount, Bencher};
use keyvalues_parser::Vdf;

fn main() {
    // Run registered benchmarks
    divan::main();
}

fn read_app_info() -> Result<String, std::io::Error> {
    let vdf_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("assets")
        .join("app_info.vdf");
    fs::read_to_string(vdf_path)
}

#[bench]
pub fn parse(bencher: Bencher) {
    let vdf_text = read_app_info().unwrap();

    let bytes = BytesCount::of_str(&vdf_text);
    bencher.counter(bytes).bench(|| {
        Vdf::parse(black_box(&vdf_text)).unwrap();
    });
}

#[bench]
pub fn render(bencher: Bencher) {
    let vdf_text = read_app_info().unwrap();
    let vdf = Vdf::parse(&vdf_text).unwrap();

    let bytes = BytesCount::of_str(&vdf_text);
    bencher.counter(bytes).bench(|| vdf.to_string())
}
