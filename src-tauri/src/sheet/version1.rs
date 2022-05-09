use anyhow::bail;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use crate::sheet::Version;

const THIS_VERSION: Version = Version::Tiger1;

#[derive(Serialize, Deserialize)]
struct VersionedSheet {
    sheet: Sheet,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Sheet {
    pub(in crate::sheet) frames: Vec<Frame>,
    pub(in crate::sheet) animations: Vec<Animation>,
    pub(in crate::sheet) export_settings: Option<ExportSettings>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Animation {
    pub(in crate::sheet) name: String,
    pub(in crate::sheet) timeline: Vec<Keyframe>,
    pub(in crate::sheet) is_looping: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Frame {
    pub(in crate::sheet) source: PathBuf,
    pub(in crate::sheet) hitboxes: Vec<Hitbox>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Keyframe {
    pub(in crate::sheet) frame: PathBuf,
    pub(in crate::sheet) duration: u32, // in ms
    pub(in crate::sheet) offset: (i32, i32),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Hitbox {
    pub(in crate::sheet) name: String,
    pub(in crate::sheet) geometry: Shape,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Shape {
    Rectangle(Rectangle),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Rectangle {
    pub(in crate::sheet) top_left: (i32, i32),
    pub(in crate::sheet) size: (u32, u32),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExportSettings {
    pub(in crate::sheet) format: ExportFormat,
    pub(in crate::sheet) texture_destination: PathBuf,
    pub(in crate::sheet) metadata_destination: PathBuf,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ExportFormat {
    Template(PathBuf),
}

pub(super) fn read_file<T: AsRef<Path>>(version: Version, path: T) -> anyhow::Result<Sheet> {
    assert!(version == THIS_VERSION);
    match version {
        THIS_VERSION => {
            let deserialized: VersionedSheet =
                serde_json::from_reader(BufReader::new(File::open(path.as_ref())?))?;
            Ok(deserialized.sheet)
        }
        _ => bail!("Unexpected version"),
    }
}
