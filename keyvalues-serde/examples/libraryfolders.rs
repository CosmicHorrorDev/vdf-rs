use std::{
    borrow::Cow,
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use keyvalues_parser::Vdf;
use keyvalues_serde::from_vdf;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[allow(dead_code)] // We display the fields
struct LibraryFolders {
    #[serde(rename = "contentstatsid")]
    content_stats_id: i128,
    libraries: Vec<Library>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)] // We display the fields
struct Library {
    path: PathBuf,
    label: String,
    #[serde(rename = "contentid")]
    content_id: i128,
    #[serde(rename = "totalsize")]
    total_size: u64,
    update_clean_bytes_tally: u64,
    time_last_update_corruption: u64,
    apps: HashMap<u64, u64>,
}

fn read_asset_file(file_name: &str) -> std::io::Result<String> {
    let asset_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("examples")
        .join(file_name);
    fs::read_to_string(asset_path)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let vdf_text = read_asset_file("libraryfolders.vdf")?;
    let mut vdf = Vdf::parse(&vdf_text)?;
    let obj = vdf.value.get_mut_obj().unwrap();

    // Switch all the entries with keys that are an index (0, 1, ...) to `"libraries"`
    let mut index = 0;
    while let Some(mut library) = obj.remove(index.to_string().as_str()) {
        obj.entry(Cow::from("libraries"))
            .or_insert(Vec::new())
            .push(library.pop().unwrap());

        index += 1;
    }

    let deserialized: LibraryFolders = from_vdf(vdf)?;
    println!("Deserialized output:\n{:#?}", deserialized);

    Ok(())
}
