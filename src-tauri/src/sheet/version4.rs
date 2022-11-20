use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::io::Read;
use std::path::PathBuf;
use uuid::Uuid;

use crate::sheet::version3 as previous_version;
use crate::sheet::{SheetError, Version};

const THIS_VERSION: Version = Version::Tiger4;

#[derive(Serialize, Deserialize)]
struct VersionedSheet {
    sheet: Sheet,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Sheet {
    pub(in crate::sheet) frames: Vec<Frame>,
    pub(in crate::sheet) animations: BTreeMap<String, Animation>,
    pub(in crate::sheet) export_settings: Option<ExportSettings>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Frame {
    pub(in crate::sheet) source: PathBuf,
}

#[derive(Derivative)]
#[derivative(PartialEq)]
#[derive(Clone, Debug, Eq, Serialize, Deserialize)]
pub struct Animation {
    pub(in crate::sheet) sequences: BTreeMap<Direction, Sequence>,
    pub(in crate::sheet) is_looping: bool,
    #[derivative(PartialEq = "ignore")]
    #[serde(skip, default = "Uuid::new_v4")]
    pub(in crate::sheet) key: Uuid,
}

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    Hash,
    PartialEq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    enum_iterator::Sequence,
)]
pub enum Direction {
    #[default]
    East,
    NorthEast,
    North,
    NorthWest,
    West,
    SouthWest,
    South,
    SouthEast,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, enum_iterator::Sequence)]
pub enum DirectionPreset {
    FourDirections,
    EightDirections,
    LeftRight,
    UpDown,
    Isometric,
    FixedAngle,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Sequence {
    pub(in crate::sheet) keyframes: Vec<Keyframe>,
}

#[derive(Derivative)]
#[derivative(PartialEq)]
#[derive(Clone, Debug, Eq, Serialize, Deserialize)]
pub struct Keyframe {
    pub(in crate::sheet) frame: PathBuf,
    pub(in crate::sheet) hitboxes: BTreeMap<String, Hitbox>,
    pub(in crate::sheet) duration_millis: u64,
    pub(in crate::sheet) offset: (i32, i32),
    #[derivative(PartialEq = "ignore")]
    #[serde(skip, default = "Uuid::new_v4")]
    pub(in crate::sheet) key: Uuid,
}

#[derive(Derivative)]
#[derivative(PartialEq)]
#[derive(Clone, Debug, Eq, Serialize, Deserialize)]
pub struct Hitbox {
    pub(in crate::sheet) geometry: Shape,
    #[derivative(PartialEq = "ignore")]
    #[serde(skip, default = "Uuid::new_v4")]
    pub(in crate::sheet) key: Uuid,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Shape {
    Rectangle(Rectangle),
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ExportSettings {
    Liquid(LiquidExportSettings),
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct LiquidExportSettings {
    pub(in crate::sheet) template_file: PathBuf,
    pub(in crate::sheet) texture_file: PathBuf,
    pub(in crate::sheet) metadata_file: PathBuf,
    pub(in crate::sheet) metadata_paths_root: PathBuf,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Rectangle {
    pub(in crate::sheet) top_left: (i32, i32),
    pub(in crate::sheet) size: (u32, u32),
}

pub(super) fn read_file<R: Read>(version: Version, reader: R) -> Result<Sheet, SheetError> {
    match version {
        THIS_VERSION => {
            let deserialized: VersionedSheet = serde_json::from_reader(reader)?;
            Ok(deserialized.sheet)
        }
        _ => Ok(previous_version::read_file(version, reader)?.into()),
    }
}

impl From<previous_version::Sheet> for Sheet {
    fn from(old: previous_version::Sheet) -> Sheet {
        Sheet {
            frames: old.frames.into_iter().map(|o| o.into()).collect(),
            animations: old
                .animations
                .into_iter()
                .map(|o| (o.name.to_owned(), o.into()))
                .collect(),
            export_settings: old.export_settings.map(|o| o.into()),
        }
    }
}

impl From<previous_version::Animation> for Animation {
    fn from(old: previous_version::Animation) -> Animation {
        Animation {
            sequences: BTreeMap::from([(Direction::East, old.timeline.into())]),
            is_looping: old.is_looping,
            key: Uuid::new_v4(),
        }
    }
}

impl From<Vec<previous_version::Keyframe>> for Sequence {
    fn from(keyframes: Vec<previous_version::Keyframe>) -> Sequence {
        Sequence {
            keyframes: keyframes.into_iter().map(|k| k.into()).collect(),
        }
    }
}

impl From<previous_version::Frame> for Frame {
    fn from(old: previous_version::Frame) -> Frame {
        Frame { source: old.source }
    }
}

impl From<previous_version::Keyframe> for Keyframe {
    fn from(old: previous_version::Keyframe) -> Keyframe {
        Keyframe {
            frame: old.frame,
            duration_millis: old.duration_millis as u64,
            offset: old.offset,
            hitboxes: old
                .hitboxes
                .into_iter()
                .map(|o| (o.name.to_owned(), o.into()))
                .collect(),
            key: Uuid::new_v4(),
        }
    }
}

impl From<previous_version::Hitbox> for Hitbox {
    fn from(old: previous_version::Hitbox) -> Hitbox {
        Hitbox {
            geometry: old.geometry.into(),
            key: Uuid::new_v4(),
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

impl From<previous_version::Rectangle> for Rectangle {
    fn from(old: previous_version::Rectangle) -> Rectangle {
        Rectangle {
            top_left: old.top_left,
            size: old.size,
        }
    }
}

impl From<previous_version::ExportSettings> for ExportSettings {
    fn from(old: previous_version::ExportSettings) -> ExportSettings {
        ExportSettings::Liquid(LiquidExportSettings {
            template_file: match old.format {
                previous_version::ExportFormat::Template(p) => p,
            },
            texture_file: old.texture_destination,
            metadata_file: old.metadata_destination.clone(),
            metadata_paths_root: old.metadata_destination,
        })
    }
}
