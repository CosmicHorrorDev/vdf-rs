use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use keyvalues_serde::from_str;
use serde::Deserialize;

use std::{collections::HashMap, fs, path::Path};

fn read_app_info() -> Result<String, std::io::Error> {
    let vdf_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("assets")
        .join("app_info.vdf");
    fs::read_to_string(vdf_path)
}

type Id = u64;

// Representation of the app_info file
#[derive(Deserialize, Debug)]
struct AppInfoAll {
    common: Common,
    config: AppInfoConfig,
    #[serde(rename = "depot")]
    depots: Vec<Depot>,
    branches: Branches,
}

#[derive(Deserialize, Debug)]
struct Common {
    name: String,
    #[serde(rename = "type")]
    app_type: String,
    parent: Id,
    oslist: String,
    osarch: String,
    icon: String,
    logo: String,
    logo_small: String,
    clienticon: String,
    clienttga: String,
    #[serde(rename = "ReleaseState")]
    release_state: String,
    // Not actually sure what this would map since it's empty
    associations: HashMap<String, String>,
    gameid: Id,
}

#[derive(Deserialize, Debug)]
struct AppInfoConfig {
    installdir: String,
    launch: HashMap<Id, Launch>,
}

#[derive(Deserialize, Debug)]
struct Launch {
    executable: String,
    #[serde(rename = "type")]
    launch_type: String,
    config: LaunchConfig,
}

#[derive(Deserialize, Debug)]
struct LaunchConfig {
    oslist: String,
}

#[derive(Deserialize, Debug)]
struct Depot {
    id: Id,
    name: String,
    config: DepotConfig,
    manifests: Manifest,
    maxsize: Option<u64>,
    depotfromapp: Option<Id>,
    encryptedmanifests: Option<EncryptedManifest>,
}

#[derive(Deserialize, Debug)]
struct DepotConfig {
    oslist: String,
}

#[derive(Deserialize, Debug)]
struct Manifest {
    public: u64,
}

#[derive(Deserialize, Debug)]
struct EncryptedManifest {
    experimental: HashMap<String, String>,
}

#[derive(Deserialize, Debug)]
struct Branches {
    public: Branch,
    experimental: Branch,
    unstable: Branch,
}

#[derive(Deserialize, Debug)]
struct Branch {
    buildid: Id,
    description: Option<String>,
    pwdrequired: Option<bool>,
    timeupdated: u64,
}

#[derive(Deserialize)]
struct AppInfoExtract;

pub fn de_all_timing(c: &mut Criterion) {
    let vdf_text = read_app_info().unwrap();

    c.bench_function("de all timing", |b| {
        b.iter(|| {
            let _: AppInfoAll = from_str(black_box(&vdf_text)).unwrap();
        })
    });
}

pub fn de_all_throughput(c: &mut Criterion) {
    let vdf_text = read_app_info().unwrap();

    let mut group = c.benchmark_group("de all throughput");
    group.throughput(Throughput::Bytes(vdf_text.len() as u64));
    group.bench_function("de", |b| {
        b.iter(|| {
            let _: AppInfoAll = from_str(black_box(&vdf_text)).unwrap();
        })
    });
    group.finish();
}

// pub fn de_extract_deeply_nested_timing(c: &mut Criterion) {
//     let vdf_text = read_app_info().unwrap();

//     c.bench_function("de all timing", |b| {
//         b.iter(|| {
//             let _: AppInfoExtract = from_str(&vdf_text).unwrap();
//         })
//     });
// }

// pub fn render_time(c: &mut Criterion) {
//     let vdf_text = read_app_info().unwrap();
//     let vdf = Vdf::parse(&vdf_text).unwrap();

//     c.bench_function("render timing", |b| {
//         b.iter(|| vdf.to_string());
//     });
// }

// pub fn render_throughput(c: &mut Criterion) {
//     let vdf_text = read_app_info().unwrap();
//     let vdf = Vdf::parse(&vdf_text).unwrap();

//     let mut group = c.benchmark_group("render throughput");
//     group.throughput(Throughput::Bytes(vdf_text.len() as u64));
//     group.bench_function("render", |b| b.iter(|| vdf.to_string()));
//     group.finish();
// }

criterion_group!(timings, de_all_timing);
criterion_group!(throughput, de_all_throughput);
criterion_main!(timings, throughput);
