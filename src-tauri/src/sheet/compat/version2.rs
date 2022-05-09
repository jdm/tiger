use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use crate::sheet::compat::version1 as previous_version;
use crate::sheet::compat::Version;

const THIS_VERSION: Version = Version::Tiger2;

#[derive(Serialize, Deserialize)]
struct VersionedSheet {
    sheet: Sheet,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Sheet {
    pub(in crate::sheet) frames: Vec<Frame>,
    pub(in crate::sheet) animations: Vec<Animation>,
    pub(in crate::sheet) export_settings: Option<ExportSettings>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Frame {
    pub(in crate::sheet) source: PathBuf,
    pub(in crate::sheet) hitboxes: Vec<Hitbox>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Animation {
    pub(in crate::sheet) name: String,
    pub(in crate::sheet) timeline: Vec<Keyframe>,
    pub(in crate::sheet) is_looping: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Keyframe {
    pub(in crate::sheet) frame: PathBuf,
    pub(in crate::sheet) duration: u32, // in ms
    pub(in crate::sheet) offset: (i32, i32),
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Hitbox {
    pub(in crate::sheet) name: String,
    pub(in crate::sheet) geometry: Shape,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Shape {
    Rectangle(Rectangle),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ExportSettings {
    pub(in crate::sheet) format: ExportFormat,
    pub(in crate::sheet) texture_destination: PathBuf,
    pub(in crate::sheet) metadata_destination: PathBuf,
    pub(in crate::sheet) metadata_paths_root: PathBuf,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum ExportFormat {
    Template(PathBuf),
}

pub fn read_file<T: AsRef<Path>>(version: Version, path: T) -> anyhow::Result<Sheet> {
    match version {
        THIS_VERSION => {
            let deserialized: VersionedSheet =
                serde_json::from_reader(BufReader::new(File::open(path.as_ref())?))?;
            Ok(deserialized.sheet)
        }
        _ => Ok(previous_version::read_file(version, path)?.into()),
    }
}

impl From<previous_version::Sheet> for Sheet {
    fn from(old: previous_version::Sheet) -> Sheet {
        Sheet {
            frames: old.frames.into_iter().map(|o| o.into()).collect(),
            animations: old.animations.into_iter().map(|o| o.into()).collect(),
            export_settings: old.export_settings.map(|o| o.into()),
        }
    }
}

impl From<previous_version::Animation> for Animation {
    fn from(old: previous_version::Animation) -> Animation {
        Animation {
            name: old.name,
            timeline: old.timeline.into_iter().map(|o| o.into()).collect(),
            is_looping: old.is_looping,
        }
    }
}

impl From<previous_version::Frame> for Frame {
    fn from(old: previous_version::Frame) -> Frame {
        Frame {
            source: old.source,
            hitboxes: old.hitboxes.into_iter().map(|o| o.into()).collect(),
        }
    }
}

impl From<previous_version::Keyframe> for Keyframe {
    fn from(old: previous_version::Keyframe) -> Keyframe {
        Keyframe {
            frame: old.frame,
            duration: old.duration,
            offset: old.offset,
        }
    }
}

impl From<previous_version::Hitbox> for Hitbox {
    fn from(old: previous_version::Hitbox) -> Hitbox {
        Hitbox {
            name: old.name,
            geometry: old.geometry.into(),
        }
    }
}

impl From<previous_version::Shape> for Shape {
    fn from(old: previous_version::Shape) -> Shape {
        match old {
            previous_version::Shape::Rectangle(r) => Shape::Rectangle(r.into()),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Rectangle {
    pub top_left: (i32, i32),
    pub size: (u32, u32),
}

impl From<previous_version::Rectangle> for Rectangle {
    fn from(old: previous_version::Rectangle) -> Rectangle {
        Rectangle {
            top_left: old.top_left,
            size: old.size,
        }
    }
}

impl From<previous_version::ExportFormat> for ExportFormat {
    fn from(old: previous_version::ExportFormat) -> ExportFormat {
        match old {
            previous_version::ExportFormat::Template(p) => ExportFormat::Template(p),
        }
    }
}

impl From<previous_version::ExportSettings> for ExportSettings {
    fn from(old: previous_version::ExportSettings) -> ExportSettings {
        ExportSettings {
            format: old.format.into(),
            texture_destination: old.texture_destination,
            metadata_destination: old.metadata_destination.clone(),
            metadata_paths_root: old.metadata_destination,
        }
    }
}
