use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::io::Read;
use std::path::PathBuf;
use uuid::Uuid;

use crate::sheet::{ordered_map, ordered_slice, portable_path, Any, Paths, SheetError, Version};

const THIS_VERSION: Version = Version::Tiger_0_5_0;

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
    #[allow(dead_code)]
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
    #[allow(dead_code)]
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
    #[allow(dead_code)]
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
        _ => Err(SheetError::UnsupportedVersion(format!("{version:?}"))),
    }
}
