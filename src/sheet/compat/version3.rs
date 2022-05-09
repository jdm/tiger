use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use crate::sheet::compat::version2 as previous_version;
use crate::sheet::compat::Version;

const THIS_VERSION: Version = Version::Tiger3;

#[derive(Serialize, Deserialize)]
pub struct VersionedSheet {
    pub sheet: Sheet,
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

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Sheet {
    pub frames: Vec<Frame>,
    pub animations: Vec<Animation>,
    pub export_settings: Option<ExportSettings>,
}

impl From<previous_version::Sheet> for Sheet {
    fn from(old: previous_version::Sheet) -> Sheet {
        let mut new_animations: Vec<Animation> =
            old.animations.into_iter().map(|o| o.into()).collect();

        // Migrate hitbox data from frames to keyframes
        for frame in &old.frames {
            for hitbox in &frame.hitboxes {
                for animation in &mut new_animations {
                    for keyframe in &mut animation.timeline {
                        if keyframe.frame == frame.source {
                            let mut new_hitbox: Hitbox = hitbox.clone().into();
                            let Shape::Rectangle(r) = &mut new_hitbox.geometry;
                            r.top_left.0 += keyframe.offset.0;
                            r.top_left.1 += keyframe.offset.1;
                            keyframe.hitboxes.push(new_hitbox);
                        }
                    }
                }
            }
        }
        let new_frames = old.frames.into_iter().map(|o| o.into()).collect();
        Sheet {
            frames: new_frames,
            animations: new_animations,
            export_settings: old.export_settings.map(|o| o.into()),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Animation {
    pub name: String,
    pub timeline: Vec<Keyframe>,
    pub is_looping: bool,
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

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Frame {
    pub source: PathBuf,
}

impl From<previous_version::Frame> for Frame {
    fn from(old: previous_version::Frame) -> Frame {
        Frame { source: old.source }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Keyframe {
    pub frame: PathBuf,
    pub hitboxes: Vec<Hitbox>,
    pub duration: u32, // in ms
    pub offset: (i32, i32),
}

impl From<previous_version::Keyframe> for Keyframe {
    fn from(old: previous_version::Keyframe) -> Keyframe {
        Keyframe {
            frame: old.frame,
            duration: old.duration,
            offset: old.offset,
            hitboxes: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Hitbox {
    pub name: String,
    pub geometry: Shape,
    pub linked: bool,
    pub locked: bool,
}

impl From<previous_version::Hitbox> for Hitbox {
    fn from(old: previous_version::Hitbox) -> Hitbox {
        Hitbox {
            name: old.name,
            geometry: old.geometry.into(),
            linked: true,
            locked: false,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Shape {
    Rectangle(Rectangle),
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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum ExportFormat {
    Template(PathBuf),
}

impl From<previous_version::ExportFormat> for ExportFormat {
    fn from(old: previous_version::ExportFormat) -> ExportFormat {
        match old {
            previous_version::ExportFormat::Template(p) => ExportFormat::Template(p),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ExportSettings {
    pub format: ExportFormat,
    pub texture_destination: PathBuf,
    pub metadata_destination: PathBuf,
    pub metadata_paths_root: PathBuf,
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
