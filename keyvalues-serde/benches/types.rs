use serde::{Deserialize, Serialize};

use std::collections::BTreeMap;

pub type Id = u64;

#[derive(Debug, Deserialize, Serialize)]
pub struct SingleField {
    game: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FullStructOwned {
    version: Id,
    game: String,
    title: String,
    description: String,
    controller_type: String,
    localization: BTreeMap<Local, Localization>,
    #[serde(rename = "group")]
    groups: Vec<Group>,
    preset: Preset,
    settings: Settings,
}

// Duplicate definition with fields manually set up to borrow. Generics don't cut it because
// `#[serde(borrow)]` requires some lifetime on the field, so you can't borrow fields that are used
// for owned things. This is also why we sometimes have to give structs some specific lifetime
// annotation even though it's all `'static` ;-;
#[derive(Debug, Deserialize, Serialize)]
pub struct FullStructBorrowed {
    version: Id,
    game: &'static str,
    title: &'static str,
    description: &'static str,
    controller_type: &'static str,
    #[serde(borrow)]
    localization: BTreeMap<Local, LocalizationBorrowed<'static>>,
    #[serde(borrow, rename = "group")]
    groups: Vec<GroupBorrowed<'static>>,
    #[serde(borrow)]
    preset: PresetBorrowed<'static>,
    settings: Settings,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum Local {
    Brazilian,
    Bulgarian,
    Czech,
    Danish,
    Dutch,
    English,
    Finnish,
    French,
    German,
    Greek,
    Hungarian,
    Italian,
    Japanese,
    Koreana,
    Polish,
    Portuguese,
    Romanian,
    Russian,
    SChinese,
    Spanish,
    Swedish,
    Turkish,
    Ukrainian,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Localization {
    title: String,
    description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LocalizationBorrowed<'a> {
    title: &'a str,
    #[serde(borrow)]
    description: Option<&'a str>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Group {
    id: Id,
    mode: Mode,
    inputs: BTreeMap<InputKind, Input>,
    gameactions: Option<Gameactions>,
    settings: Option<GroupSettings>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GroupBorrowed<'a> {
    id: Id,
    mode: Mode,
    #[serde(borrow)]
    inputs: BTreeMap<InputKind, InputBorrowed<'a>>,
    gameactions: Option<Gameactions>,
    settings: Option<GroupSettings>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
enum Mode {
    AbsoluteMouse,
    Dpad,
    Flickstick,
    FourButtons,
    JoystickMouse,
    Switches,
    Trigger,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
enum InputKind {
    ButtonA,
    ButtonB,
    ButtonX,
    ButtonY,
    ButtonBackLeft,
    ButtonBackRight,
    ButtonEscape,
    ButtonMenu,
    Click,
    DpadNorth,
    DpadSouth,
    DpadEast,
    DpadWest,
    Edge,
    LeftBumper,
    RightBumper,
}

#[derive(Debug, Deserialize, Serialize)]
struct Input {
    activators: BTreeMap<ActivatorKind, Activator>,
}

#[derive(Debug, Deserialize, Serialize)]
struct InputBorrowed<'a> {
    #[serde(borrow)]
    activators: BTreeMap<ActivatorKind, ActivatorBorrowed<'a>>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
enum ActivatorKind {
    #[serde(rename = "Double_Press")]
    DoublePress,
    #[serde(rename = "Full_Press")]
    FullPress,
}

#[derive(Debug, Deserialize, Serialize)]
struct Activator {
    bindings: Bindings,
    settings: Option<ActivatorSettings>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ActivatorBorrowed<'a> {
    #[serde(borrow)]
    bindings: BindingsBorrowed<'a>,
    settings: Option<ActivatorSettings>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Bindings {
    binding: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct BindingsBorrowed<'a> {
    binding: &'a str,
}

#[derive(Debug, Deserialize, Serialize)]
struct ActivatorSettings {
    haptic_intensity: Option<u64>,
    repeat_rate: Option<u64>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Gameactions {}

#[derive(Debug, Deserialize, Serialize)]
struct GroupSettings {
    button_dist: Option<u64>,
    button_size: Option<u64>,
    doubletap_max_duration: Option<u64>,
    edge_binding_radius: Option<u64>,
    requires_click: Option<bool>,
    sensitivity: Option<u64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Preset {
    id: Id,
    name: String,
    group_source_bindings: BTreeMap<Id, String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PresetBorrowed<'a> {
    id: Id,
    name: &'a str,
    #[serde(borrow)]
    group_source_bindings: BTreeMap<Id, &'a str>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Settings {
    left_trackpad_mode: u64,
    right_trackpad_mode: u64,
}
