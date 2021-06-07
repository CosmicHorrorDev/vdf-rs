use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use keyvalues_serde::{from_str, to_string};
use serde::{Deserialize, Serialize};

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
#[derive(Deserialize, Serialize, Debug)]
struct AppInfoAll {
    common: Common,
    config: AppInfoConfig,
    #[serde(rename = "depot")]
    depots: Vec<Depot>,
    branches: Branches,
}

#[derive(Deserialize, Serialize, Debug)]
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

#[derive(Deserialize, Serialize, Debug)]
struct AppInfoConfig {
    installdir: String,
    launch: HashMap<Id, Launch>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Launch {
    executable: String,
    #[serde(rename = "type")]
    launch_type: String,
    config: LaunchConfig,
}

#[derive(Deserialize, Serialize, Debug)]
struct LaunchConfig {
    oslist: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Depot {
    id: Id,
    name: String,
    config: DepotConfig,
    manifests: Manifest,
    maxsize: Option<u64>,
    depotfromapp: Option<Id>,
    encryptedmanifests: Option<EncryptedManifest>,
}

#[derive(Deserialize, Serialize, Debug)]
struct DepotConfig {
    oslist: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Manifest {
    public: u64,
}

#[derive(Deserialize, Serialize, Debug)]
struct EncryptedManifest {
    experimental: HashMap<String, String>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Branches {
    public: Branch,
    experimental: Branch,
    unstable: Branch,
}

#[derive(Deserialize, Serialize, Debug)]
struct Branch {
    buildid: Id,
    description: Option<String>,
    pwdrequired: Option<bool>,
    timeupdated: u64,
}

#[derive(Deserialize, Serialize, Debug)]
struct AppInfoExtract {
    branches: BranchesExtract,
}

#[derive(Deserialize, Serialize, Debug)]
struct BranchesExtract {
    public: BranchExtract,
}

#[derive(Deserialize, Serialize, Debug)]
struct BranchExtract {
    buildid: Id,
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

pub fn de_all_throughput(c: &mut Criterion) {
    let vdf_text = read_app_info().unwrap();

    let mut group = c.benchmark_group("de throughput");
    group.throughput(Throughput::Bytes(vdf_text.len() as u64));
    group.bench_function("all", |b| {
        b.iter(|| from_str_helper::<AppInfoAll>(&vdf_text))
    });
    group.bench_function("single", |b| {
        b.iter(|| from_str_helper::<AppInfoExtract>(&vdf_text))
    });
    group.finish();
}

// It doesn't really make sense to reserialize just the extracted content
pub fn ser_all_throughput(c: &mut Criterion) {
    let vdf_text = read_app_info().unwrap();
    let app_info_all: AppInfoAll = from_str_helper(&vdf_text);
    let ser_len = to_string_helper::<AppInfoAll>(&app_info_all).len();

    let mut group = c.benchmark_group("ser throughput");
    group.throughput(Throughput::Bytes(ser_len as u64));
    group.bench_function("all", |b| {
        b.iter(|| to_string_helper::<AppInfoAll>(&app_info_all))
    });
    group.finish();
}

criterion_group!(throughput, de_all_throughput, ser_all_throughput);
criterion_main!(throughput);
