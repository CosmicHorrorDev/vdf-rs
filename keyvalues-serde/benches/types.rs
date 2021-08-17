use serde::{Deserialize, Serialize};

use std::{collections::HashMap, hash};

pub type Id = u64;

// Representation of the app_info file
#[derive(Deserialize, Serialize, Debug)]
pub struct AppInfo<T: Eq + hash::Hash> {
    common: Common<T>,
    config: AppInfoConfig<T>,
    #[serde(rename = "depot")]
    depots: Vec<Depot<T>>,
    branches: Branches<T>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Common<T: Eq + hash::Hash> {
    name: T,
    #[serde(rename = "type")]
    app_type: T,
    parent: Id,
    oslist: T,
    osarch: T,
    icon: T,
    logo: T,
    logo_small: T,
    clienticon: T,
    clienttga: T,
    #[serde(rename = "ReleaseState")]
    release_state: T,
    // Not actually sure what this would map since it's empty
    associations: HashMap<T, T>,
    gameid: Id,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AppInfoConfig<T: Eq + hash::Hash> {
    installdir: T,
    launch: HashMap<Id, Launch<T>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Launch<T: Eq + hash::Hash> {
    executable: T,
    #[serde(rename = "type")]
    launch_type: T,
    config: LaunchConfig<T>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LaunchConfig<T: Eq + hash::Hash> {
    oslist: T,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Depot<T: Eq + hash::Hash> {
    id: Id,
    name: T,
    config: DepotConfig<T>,
    manifests: Manifest,
    maxsize: Option<u64>,
    depotfromapp: Option<Id>,
    encryptedmanifests: Option<EncryptedManifest<T>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DepotConfig<T: Eq + hash::Hash> {
    oslist: T,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Manifest {
    public: u64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct EncryptedManifest<T: Eq + hash::Hash> {
    experimental: HashMap<T, T>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Branches<T: Eq + hash::Hash> {
    public: Branch<T>,
    experimental: Branch<T>,
    unstable: Branch<T>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Branch<T: Eq + hash::Hash> {
    buildid: Id,
    description: Option<T>,
    pwdrequired: Option<bool>,
    timeupdated: u64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AppInfoExtract {
    branches: BranchesExtract,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct BranchesExtract {
    public: BranchExtract,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct BranchExtract {
    buildid: Id,
}
