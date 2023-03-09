use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::io::Read;
use std::path::PathBuf;
use uuid::Uuid;

use crate::sheet::version_0_5_0 as previous_version;
use crate::sheet::{ordered_map, ordered_slice, portable_path, Any, Paths, SheetError, Version};

const THIS_VERSION: Version = Version::Tiger_1_0_0;

#[derive(Serialize, Deserialize)]
struct VersionedSheet {
    sheet: Sheet<Any>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Sheet<P: Paths> {
    #[serde(serialize_with = "ordered_slice")]
    pub(in crate::sheet) frames: Vec<Frame<P>>,
    #[serde(serialize_with = "ordered_map")]
    pub(in crate::sheet) animations: HashMap<String, Animation<P>>,
    pub(in crate::sheet) export_settings: Option<ExportSettings<P>>,
    #[serde(skip)]
    pub(in crate::sheet) paths: P,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct Frame<P: Paths> {
    #[serde(serialize_with = "portable_path")]
    pub(in crate::sheet) source: PathBuf,
    #[serde(skip)]
    pub(in crate::sheet) paths: std::marker::PhantomData<P>,
}

#[derive(Derivative)]
#[derivative(PartialEq)]
#[derive(Clone, Debug, Eq, Serialize, Deserialize)]
pub struct Animation<P: Paths> {
    pub(in crate::sheet) sequences: BTreeMap<Direction, Sequence<P>>,
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
pub struct Sequence<P: Paths> {
    pub(in crate::sheet) keyframes: Vec<Keyframe<P>>,
}

#[derive(Derivative)]
#[derivative(PartialEq)]
#[derive(Clone, Debug, Eq, Serialize, Deserialize)]
pub struct Keyframe<P: Paths> {
    #[serde(serialize_with = "portable_path")]
    pub(in crate::sheet) frame: PathBuf,
    #[serde(serialize_with = "ordered_map")]
    pub(in crate::sheet) hitboxes: HashMap<String, Hitbox>,
    pub(in crate::sheet) duration_millis: u64,
    pub(in crate::sheet) offset: (i32, i32),
    #[derivative(PartialEq = "ignore")]
    #[serde(skip, default = "Uuid::new_v4")]
    pub(in crate::sheet) key: Uuid,
    #[serde(skip)]
    pub(in crate::sheet) paths: std::marker::PhantomData<P>,
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
pub enum ExportSettings<P: Paths> {
    Template(TemplateExportSettings<P>),
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct TemplateExportSettings<P: Paths> {
    #[serde(serialize_with = "portable_path")]
    pub(in crate::sheet) template_file: PathBuf,
    #[serde(serialize_with = "portable_path")]
    pub(in crate::sheet) atlas_image_file: PathBuf,
    #[serde(serialize_with = "portable_path")]
    pub(in crate::sheet) metadata_file: PathBuf,
    #[serde(serialize_with = "portable_path")]
    pub(in crate::sheet) metadata_paths_root: PathBuf,
    #[serde(skip)]
    pub(in crate::sheet) paths: std::marker::PhantomData<P>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Rectangle {
    pub(in crate::sheet) top_left: (i32, i32),
    pub(in crate::sheet) size: (u32, u32),
}

pub(super) fn read_file<R: Read>(version: Version, reader: R) -> Result<Sheet<Any>, SheetError> {
    match version {
        THIS_VERSION => {
            let deserialized: VersionedSheet = serde_json::from_reader(reader)?;
            Ok(deserialized.sheet)
        }
        _ => Ok(previous_version::read_file(version, reader)?.into()),
    }
}

impl From<previous_version::Sheet<Any>> for Sheet<Any> {
    fn from(old: previous_version::Sheet<Any>) -> Sheet<Any> {
        Sheet {
            frames: old.frames.into_iter().map(|o| o.into()).collect(),
            animations: old
                .animations
                .into_iter()
                .map(|(n, a)| (n, a.into()))
                .collect(),
            export_settings: old.export_settings.map(|o| o.into()),
            paths: Default::default(),
        }
    }
}

impl From<previous_version::Animation<Any>> for Animation<Any> {
    fn from(old: previous_version::Animation<Any>) -> Animation<Any> {
        Self {
            sequences: old
                .sequences
                .into_iter()
                .map(|(d, s)| (d.into(), s.into()))
                .collect(),
            is_looping: old.is_looping,
            key: Uuid::new_v4(),
        }
    }
}

impl From<previous_version::Direction> for Direction {
    fn from(old: previous_version::Direction) -> Self {
        match old {
            previous_version::Direction::East => Direction::East,
            previous_version::Direction::NorthEast => Direction::NorthEast,
            previous_version::Direction::North => Direction::North,
            previous_version::Direction::NorthWest => Direction::NorthWest,
            previous_version::Direction::West => Direction::West,
            previous_version::Direction::SouthWest => Direction::SouthWest,
            previous_version::Direction::South => Direction::South,
            previous_version::Direction::SouthEast => Direction::SouthEast,
        }
    }
}

impl From<previous_version::Sequence<Any>> for Sequence<Any> {
    fn from(old: previous_version::Sequence<Any>) -> Sequence<Any> {
        Self {
            keyframes: old.keyframes.into_iter().map(|k| k.into()).collect(),
        }
    }
}

impl From<previous_version::Frame<Any>> for Frame<Any> {
    fn from(old: previous_version::Frame<Any>) -> Self {
        Self {
            source: old.source,
            paths: std::marker::PhantomData,
        }
    }
}

impl From<previous_version::Keyframe<Any>> for Keyframe<Any> {
    fn from(old: previous_version::Keyframe<Any>) -> Keyframe<Any> {
        Self {
            frame: old.frame,
            duration_millis: old.duration_millis,
            offset: old.offset,
            hitboxes: old
                .hitboxes
                .into_iter()
                .map(|(n, h)| (n, h.into()))
                .collect(),
            key: Uuid::new_v4(),
            paths: std::marker::PhantomData,
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

impl From<previous_version::ExportSettings<Any>> for ExportSettings<Any> {
    fn from(old: previous_version::ExportSettings<Any>) -> ExportSettings<Any> {
        let previous_version::ExportSettings::Template(old_template_export_settings) = old;
        ExportSettings::Template(TemplateExportSettings {
            template_file: old_template_export_settings.template_file,
            atlas_image_file: old_template_export_settings.atlas_image_file,
            metadata_file: old_template_export_settings.metadata_file.clone(),
            metadata_paths_root: old_template_export_settings.metadata_paths_root,
            paths: std::marker::PhantomData,
        })
    }
}
