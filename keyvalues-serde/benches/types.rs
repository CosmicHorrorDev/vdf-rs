use serde::{Deserialize, Serialize};

use std::collections::HashMap;

pub type Id = u64;

// Representation of the app_info file
#[derive(Deserialize, Serialize, Debug)]
pub struct AppInfoBorrow<'a> {
    #[serde(borrow)]
    common: CommonBorrow<'a>,
    #[serde(borrow)]
    config: AppInfoConfigBorrow<'a>,
    #[serde(borrow, rename = "depot")]
    depots: Vec<DepotBorrow<'a>>,
    #[serde(borrow)]
    branches: BranchesBorrow<'a>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CommonBorrow<'a> {
    name: &'a str,
    #[serde(rename = "type")]
    app_type: &'a str,
    parent: Id,
    oslist: &'a str,
    osarch: &'a str,
    icon: &'a str,
    logo: &'a str,
    logo_small: &'a str,
    clienticon: &'a str,
    clienttga: &'a str,
    #[serde(rename = "ReleaseState")]
    release_state: &'a str,
    // Not actually sure what this would map since it's empty
    #[serde(borrow)]
    associations: HashMap<&'a str, &'a str>,
    gameid: Id,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AppInfoConfigBorrow<'a> {
    installdir: &'a str,
    #[serde(borrow)]
    launch: HashMap<Id, LaunchBorrow<'a>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LaunchBorrow<'a> {
    executable: &'a str,
    #[serde(rename = "type")]
    launch_type: &'a str,
    #[serde(borrow)]
    config: LaunchConfigBorrow<'a>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LaunchConfigBorrow<'a> {
    oslist: &'a str,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DepotBorrow<'a> {
    id: Id,
    name: &'a str,
    #[serde(borrow)]
    config: DepotConfigBorrow<'a>,
    manifests: ManifestBorrow,
    maxsize: Option<u64>,
    depotfromapp: Option<Id>,
    #[serde(borrow)]
    encryptedmanifests: Option<EncryptedManifestBorrow<'a>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DepotConfigBorrow<'a> {
    oslist: &'a str,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ManifestBorrow {
    public: u64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct EncryptedManifestBorrow<'a> {
    #[serde(borrow)]
    experimental: HashMap<&'a str, &'a str>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct BranchesBorrow<'a> {
    #[serde(borrow)]
    public: BranchBorrow<'a>,
    #[serde(borrow)]
    experimental: BranchBorrow<'a>,
    #[serde(borrow)]
    unstable: BranchBorrow<'a>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct BranchBorrow<'a> {
    buildid: Id,
    #[serde(borrow)]
    description: Option<&'a str>,
    pwdrequired: Option<bool>,
    timeupdated: u64,
}

// Representation of the app_info file
#[derive(Deserialize, Serialize, Debug)]
pub struct AppInfo {
    common: Common,
    config: AppInfoConfig,
    #[serde(rename = "depot")]
    depots: Vec<Depot>,
    branches: Branches,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Common {
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
pub struct AppInfoConfig {
    installdir: String,
    launch: HashMap<Id, Launch>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Launch {
    executable: String,
    #[serde(rename = "type")]
    launch_type: String,
    config: LaunchConfig,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LaunchConfig {
    oslist: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Depot {
    id: Id,
    name: String,
    config: DepotConfig,
    manifests: Manifest,
    maxsize: Option<u64>,
    depotfromapp: Option<Id>,
    encryptedmanifests: Option<EncryptedManifest>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DepotConfig {
    oslist: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Manifest {
    public: u64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct EncryptedManifest {
    experimental: HashMap<String, String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Branches {
    public: Branch,
    experimental: Branch,
    unstable: Branch,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Branch {
    buildid: Id,
    description: Option<String>,
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
