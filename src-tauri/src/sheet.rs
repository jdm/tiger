use core::cmp::Ordering;
use enum_iterator::all;
use euclid::default::*;
use euclid::rect;
#[cfg(test)]
use euclid::vec2;
use pathdiff::diff_paths;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashSet};
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::ops::Range;
use std::path::{Path, PathBuf};
use std::time::Duration;
use thiserror::Error;
use uuid::Uuid;

pub(in crate::sheet) mod version1;
pub(in crate::sheet) mod version2;
pub(in crate::sheet) mod version3;
pub(in crate::sheet) mod version4;

#[derive(Serialize, Deserialize, PartialEq, Eq)]
enum Version {
    Tiger1,
    Tiger2,
    Tiger3,
    Tiger4,
}

const CURRENT_VERSION: Version = Version::Tiger4;
pub use self::version4::*;

#[derive(Error, Debug)]
pub enum SheetError {
    #[error("Filesystem error for file `{0}`: `{1}`")]
    IoError(PathBuf, std::io::Error),
    #[error(transparent)]
    SerdeError(#[from] serde_json::Error),
    #[error("Could not find an animation named `{0}`")]
    AnimationNotFound(String),
    #[error("An animation with the name `{0}` already exists")]
    AnimationNameAlreadyExists(String),
    #[error("Could not find a hitbox named `{0}`")]
    HitboxNotFound(String),
    #[error("A hitbox with the name `{0}` already exists")]
    HitboxNameAlreadyExists(String),
    #[error("Error converting an absolute path to a relative path")]
    AbsoluteToRelativePath,
    #[error("Animation is missing a keyframe at index `{0}`")]
    InvalidFrameIndex(usize),
}

impl From<SheetError> for String {
    fn from(e: SheetError) -> Self {
        e.to_string()
    }
}

impl Sheet {
    pub fn read<T: AsRef<Path>>(path: T) -> Result<Sheet, SheetError> {
        #[derive(Deserialize)]
        struct Versioned {
            version: Version,
        }

        let version = {
            let file = File::open(path.as_ref())
                .map_err(|e| SheetError::IoError(path.as_ref().to_owned(), e))?;
            let versioned: Versioned = serde_json::from_reader(BufReader::new(file))?;
            versioned.version
        };

        let sheet = {
            let file = File::open(path.as_ref())
                .map_err(|e| SheetError::IoError(path.as_ref().to_owned(), e))?;
            let reader = BufReader::new(file);
            read_file(version, reader)?
        };

        let mut directory = path.as_ref().to_owned();
        directory.pop();
        Ok(sheet.with_absolute_paths(directory))
    }

    pub fn write<T: AsRef<Path>>(&self, path: T) -> Result<(), SheetError> {
        #[derive(Serialize)]
        struct VersionedSheet<'a> {
            version: Version,
            sheet: &'a Sheet,
        }
        let mut directory = path.as_ref().to_owned();
        directory.pop();
        let sheet = self.with_relative_paths(directory)?;
        let versioned_sheet = VersionedSheet {
            version: CURRENT_VERSION,
            sheet: &sheet,
        };

        let file = File::create(path.as_ref())
            .map_err(|e| SheetError::IoError(path.as_ref().to_owned(), e))?;
        serde_json::to_writer_pretty(BufWriter::new(file), &versioned_sheet)?;
        Ok(())
    }

    fn with_relative_paths<T: AsRef<Path>>(&self, relative_to: T) -> Result<Sheet, SheetError> {
        let mut sheet = self.clone();
        for frame in sheet.frames_iter_mut() {
            frame.source = diff_paths(&frame.source, relative_to.as_ref())
                .ok_or(SheetError::AbsoluteToRelativePath)?;
        }
        for (_name, animation) in sheet.animations.iter_mut() {
            for (_direction, sequence) in animation.sequences.iter_mut() {
                for keyframe in sequence.keyframes_iter_mut() {
                    keyframe.frame = diff_paths(&keyframe.frame, relative_to.as_ref())
                        .ok_or(SheetError::AbsoluteToRelativePath)?;
                }
            }
        }
        if let Some(e) = sheet.export_settings {
            sheet.export_settings = e.with_relative_paths(relative_to).ok();
        }
        Ok(sheet)
    }

    fn with_absolute_paths<T: AsRef<Path>>(&self, relative_to: T) -> Sheet {
        let mut sheet = self.clone();
        for frame in sheet.frames_iter_mut() {
            frame.source = relative_to.as_ref().join(&frame.source)
        }
        for (_name, animation) in sheet.animations.iter_mut() {
            for (_direction, sequence) in animation.sequences.iter_mut() {
                for keyframe in sequence.keyframes_iter_mut() {
                    keyframe.frame = relative_to.as_ref().join(&&keyframe.frame);
                }
            }
        }
        if let Some(e) = sheet.export_settings {
            sheet.export_settings = Some(e.with_absolute_paths(relative_to));
        }
        sheet
    }

    pub fn frames_iter(&self) -> std::slice::Iter<'_, Frame> {
        self.frames.iter()
    }

    pub fn frames_iter_mut(&mut self) -> std::slice::IterMut<'_, Frame> {
        self.frames.iter_mut()
    }

    pub fn animations_iter(&self) -> impl Iterator<Item = (&String, &Animation)> {
        self.animations.iter()
    }

    pub fn has_frame<T: AsRef<Path>>(&self, path: T) -> bool {
        self.frames.iter().any(|f| f.source == path.as_ref())
    }

    pub fn has_animation<T: AsRef<str>>(&self, name: T) -> bool {
        self.animations.contains_key(name.as_ref())
    }

    pub fn add_frame<T: AsRef<Path>>(&mut self, path: T) {
        if self.has_frame(&path) {
            return;
        }
        let frame = Frame::new(path);
        self.frames.push(frame);
    }

    pub fn add_frames<T: AsRef<Path>>(&mut self, paths: &Vec<T>) {
        for path in paths {
            self.add_frame(path);
        }
    }

    pub fn create_animation(&mut self) -> (String, &mut Animation) {
        let mut name = "New Animation".to_owned();
        let mut index = 2;
        while self.has_animation(&name) {
            name = format!("New Animation {}", index);
            index += 1;
        }
        let animation = Animation::default();
        self.animations.insert(name.clone(), animation);
        (name.clone(), self.animations.get_mut(&name).unwrap())
    }

    pub fn frame<T: AsRef<Path>>(&self, path: T) -> Option<&Frame> {
        self.frames.iter().find(|f| f.source == path.as_ref())
    }

    pub fn animation<T: AsRef<str>>(&self, name: T) -> Option<&Animation> {
        self.animations.get(name.as_ref())
    }

    pub fn animation_mut<T: AsRef<str>>(&mut self, name: T) -> Option<&mut Animation> {
        self.animations.get_mut(name.as_ref())
    }

    pub fn export_settings(&self) -> &Option<ExportSettings> {
        &self.export_settings
    }

    pub fn set_export_settings(&mut self, export_settings: ExportSettings) {
        self.export_settings = Some(export_settings);
    }

    pub fn rename_animation<T: AsRef<str>, U: AsRef<str>>(
        &mut self,
        old_name: T,
        new_name: U,
    ) -> Result<(), SheetError> {
        if old_name.as_ref() == new_name.as_ref() {
            return Ok(());
        }
        if self.has_animation(&new_name) {
            return Err(SheetError::AnimationNameAlreadyExists(
                new_name.as_ref().to_owned(),
            ));
        }
        let animation = self
            .animations
            .remove(old_name.as_ref())
            .ok_or_else(|| SheetError::AnimationNotFound(old_name.as_ref().to_owned()))?;
        self.animations
            .insert(new_name.as_ref().to_owned(), animation);
        Ok(())
    }

    pub fn delete_frame<T: AsRef<Path>>(&mut self, path: T) {
        self.frames.retain(|f| f.source != path.as_ref());
        for (_name, animation) in self.animations.iter_mut() {
            for (_direction, sequence) in animation.sequences.iter_mut() {
                sequence.keyframes.retain(|k| k.frame != path.as_ref())
            }
        }
    }

    pub fn delete_animation<T: AsRef<str>>(&mut self, name: T) {
        self.animations.remove(name.as_ref());
    }
}

impl Frame {
    pub fn new<T: AsRef<Path>>(path: T) -> Frame {
        Frame {
            source: path.as_ref().to_owned(),
        }
    }

    pub fn source(&self) -> &Path {
        &self.source
    }
}

impl Ord for Frame {
    fn cmp(&self, other: &Frame) -> Ordering {
        self.source
            .to_string_lossy()
            .cmp(&other.source.to_string_lossy())
    }
}

impl PartialOrd for Frame {
    fn partial_cmp(&self, other: &Frame) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Animation {
    pub fn looping(&self) -> bool {
        self.is_looping
    }

    pub fn set_looping(&mut self, new_is_looping: bool) {
        self.is_looping = new_is_looping;
    }

    pub fn sequence(&self, direction: Direction) -> Option<&Sequence> {
        self.sequences.get(&direction)
    }

    pub fn sequence_mut(&mut self, direction: Direction) -> Option<&mut Sequence> {
        self.sequences.get_mut(&direction)
    }

    pub fn sequences_iter(&self) -> impl Iterator<Item = (&Direction, &Sequence)> {
        self.sequences.iter()
    }

    pub fn sequences_iter_mut(&mut self) -> impl Iterator<Item = (&Direction, &mut Sequence)> {
        self.sequences.iter_mut()
    }

    pub fn direction_preset(&self) -> Option<DirectionPreset> {
        DirectionPreset::from_directions(self.sequences_iter().map(|(d, _s)| *d))
    }

    pub fn apply_direction_preset(&mut self, preset: DirectionPreset) {
        let directions = preset.directions();
        self.sequences.retain(|d, _s| directions.contains(d));
        for d in directions {
            self.sequences.entry(d).or_default();
        }
    }
}

impl DirectionPreset {
    pub fn from_directions<T: Iterator<Item = Direction>>(directions: T) -> Option<Self> {
        let directions_set: HashSet<Direction> = directions.collect();
        for preset in all::<DirectionPreset>() {
            if directions_set == preset.directions() {
                return Some(preset);
            }
        }
        None
    }

    pub fn directions(&self) -> HashSet<Direction> {
        match self {
            DirectionPreset::FourDirections => HashSet::from([
                Direction::North,
                Direction::South,
                Direction::East,
                Direction::West,
            ]),
            DirectionPreset::EightDirections => HashSet::from([
                Direction::East,
                Direction::NorthEast,
                Direction::North,
                Direction::NorthWest,
                Direction::West,
                Direction::SouthWest,
                Direction::South,
                Direction::SouthEast,
            ]),
            DirectionPreset::LeftRight => HashSet::from([Direction::East, Direction::West]),
            DirectionPreset::UpDown => HashSet::from([Direction::North, Direction::South]),
            DirectionPreset::Isometric => HashSet::from([
                Direction::NorthEast,
                Direction::NorthWest,
                Direction::SouthWest,
                Direction::SouthEast,
            ]),
            DirectionPreset::FixedAngle => HashSet::from([Direction::North]),
        }
    }
}

impl Sequence {
    pub fn num_keyframes(&self) -> usize {
        self.keyframes.len()
    }

    pub fn keyframe(&self, index: usize) -> Option<&Keyframe> {
        if index >= self.keyframes.len() {
            return None;
        }
        Some(&self.keyframes[index])
    }

    pub fn keyframe_mut(&mut self, index: usize) -> Option<&mut Keyframe> {
        if index >= self.keyframes.len() {
            return None;
        }
        Some(&mut self.keyframes[index])
    }

    pub fn keyframe_index_at(&self, time: Duration) -> Option<usize> {
        if self.keyframes.is_empty() {
            return None;
        }
        let mut cursor = Duration::new(0, 0);
        for (index, frame) in self.keyframes.iter().enumerate() {
            cursor += Duration::from_millis(frame.duration_millis);
            if time < cursor {
                return Some(index);
            }
        }
        Some(self.keyframes.len() - 1)
    }

    pub fn keyframe_at(&self, time: Duration) -> Option<(usize, &Keyframe)> {
        let keyframe_index = self.keyframe_index_at(time)?;
        Some((keyframe_index, self.keyframes.get(keyframe_index)?))
    }

    pub fn keyframe_at_mut(&mut self, time: Duration) -> Option<(usize, &mut Keyframe)> {
        let keyframe_index = self.keyframe_index_at(time)?;
        Some((keyframe_index, self.keyframes.get_mut(keyframe_index)?))
    }

    pub fn keyframe_times(&self) -> Vec<u64> {
        self.keyframe_time_ranges()
            .into_iter()
            .map(|r| r.start)
            .collect()
    }

    pub fn keyframe_time_ranges(&self) -> Vec<Range<u64>> {
        let mut cursor = 0;
        self.keyframes_iter()
            .map(|f| {
                let start = cursor;
                cursor += f.duration_millis();
                start..cursor
            })
            .collect()
    }

    pub fn insert_keyframe(&mut self, keyframe: Keyframe, index: usize) -> Result<(), SheetError> {
        if index > self.keyframes.len() {
            return Err(SheetError::InvalidFrameIndex(index));
        }
        self.keyframes.insert(index, keyframe);
        Ok(())
    }

    pub fn delete_keyframe(&mut self, index: usize) -> Result<Keyframe, SheetError> {
        if index >= self.keyframes.len() {
            return Err(SheetError::InvalidFrameIndex(index));
        }
        Ok(self.keyframes.remove(index))
    }

    pub fn keyframes_iter(&self) -> impl Iterator<Item = &Keyframe> {
        self.keyframes.iter()
    }

    pub fn keyframes_iter_mut(&mut self) -> impl Iterator<Item = &mut Keyframe> {
        self.keyframes.iter_mut()
    }

    pub fn duration(&self) -> Option<Duration> {
        self.duration_millis().map(Duration::from_millis)
    }

    pub fn duration_millis(&self) -> Option<u64> {
        if self.keyframes.is_empty() {
            return None;
        }
        Some(self.keyframes.iter().map(|f| f.duration_millis).sum())
    }
}

impl Keyframe {
    pub fn new<T: AsRef<Path>>(frame: T) -> Keyframe {
        Keyframe {
            frame: frame.as_ref().to_owned(),
            duration_millis: 100,
            offset: (0, 0),
            hitboxes: BTreeMap::new(),
            key: Uuid::new_v4(),
        }
    }

    pub fn frame(&self) -> &Path {
        &self.frame
    }

    pub fn duration_millis(&self) -> u64 {
        self.duration_millis
    }

    pub fn offset(&self) -> Vector2D<i32> {
        self.offset.into()
    }

    pub fn key(&self) -> Uuid {
        self.key
    }

    pub fn set_frame<T: AsRef<Path>>(&mut self, new_frame: T) {
        self.frame = new_frame.as_ref().to_owned();
    }

    pub fn set_duration_millis(&mut self, new_duration: u64) {
        self.duration_millis = new_duration;
    }

    pub fn set_offset(&mut self, new_offset: Vector2D<i32>) {
        self.offset = new_offset.to_tuple();
    }

    pub fn hitboxes_iter(&self) -> impl Iterator<Item = (&String, &Hitbox)> {
        self.hitboxes.iter()
    }

    pub fn hitboxes_iter_mut(&mut self) -> impl Iterator<Item = (&String, &mut Hitbox)> {
        self.hitboxes.iter_mut()
    }

    pub fn hitbox<T: AsRef<str>>(&self, name: T) -> Option<&Hitbox> {
        self.hitboxes.get(name.as_ref())
    }

    pub fn hitbox_mut<T: AsRef<str>>(&mut self, name: T) -> Option<&mut Hitbox> {
        self.hitboxes.get_mut(name.as_ref())
    }

    pub fn has_hitbox<T: AsRef<str>>(&self, name: T) -> bool {
        self.hitboxes.contains_key(name.as_ref())
    }

    pub fn create_hitbox(&mut self) -> (String, &mut Hitbox) {
        let mut name = "New Hitbox".to_owned();
        let mut index = 2;
        while self.has_hitbox(&name) {
            name = format!("New Hitbox {}", index);
            index += 1;
        }
        self.hitboxes.insert(name.clone(), Hitbox::new());
        (name.clone(), self.hitboxes.get_mut(&name).unwrap())
    }

    pub fn rename_hitbox<T: AsRef<str>, U: AsRef<str>>(
        &mut self,
        old_name: T,
        new_name: U,
    ) -> Result<(), SheetError> {
        if old_name.as_ref() == new_name.as_ref() {
            return Ok(());
        }
        if self.has_hitbox(&new_name) {
            return Err(SheetError::HitboxNameAlreadyExists(
                new_name.as_ref().to_owned(),
            ));
        }
        let hitbox = self
            .hitboxes
            .remove(old_name.as_ref())
            .ok_or_else(|| SheetError::HitboxNotFound(old_name.as_ref().to_owned()))?;
        self.hitboxes.insert(new_name.as_ref().to_owned(), hitbox);
        Ok(())
    }

    pub fn delete_hitbox<T: AsRef<str>>(&mut self, name: T) {
        self.hitboxes.remove(name.as_ref());
    }
}

impl Hitbox {
    pub fn new() -> Self {
        Hitbox {
            geometry: Shape::Rectangle(Rectangle {
                top_left: (-10, -10),
                size: (20, 20),
            }),
            key: Uuid::new_v4(),
        }
    }

    pub fn rectangle(&self) -> Rect<i32> {
        match &self.geometry {
            Shape::Rectangle(r) => {
                rect(r.top_left.0, r.top_left.1, r.size.0 as i32, r.size.1 as i32)
            }
        }
    }

    pub fn position(&self) -> Vector2D<i32> {
        match &self.geometry {
            Shape::Rectangle(r) => r.top_left.into(),
        }
    }

    pub fn size(&self) -> Vector2D<u32> {
        match &self.geometry {
            Shape::Rectangle(r) => r.size.into(),
        }
    }

    pub fn key(&self) -> Uuid {
        self.key
    }

    pub fn set_position(&mut self, new_position: Vector2D<i32>) {
        match &mut self.geometry {
            Shape::Rectangle(r) => {
                r.top_left = new_position.to_tuple();
            }
        }
    }

    pub fn set_size(&mut self, new_size: Vector2D<u32>) {
        match &mut self.geometry {
            Shape::Rectangle(r) => {
                r.size = new_size.to_tuple();
            }
        }
    }
}

impl ExportSettings {
    pub fn new() -> Self {
        Self::Liquid(LiquidExportSettings::default())
    }

    pub fn with_relative_paths<T: AsRef<Path>>(
        &self,
        relative_to: T,
    ) -> Result<ExportSettings, SheetError> {
        Ok(match self {
            ExportSettings::Liquid(settings) => {
                ExportSettings::Liquid(settings.with_relative_paths(relative_to)?)
            }
        })
    }

    pub fn with_absolute_paths<T: AsRef<Path>>(&self, relative_to: T) -> ExportSettings {
        match self {
            ExportSettings::Liquid(settings) => {
                ExportSettings::Liquid(settings.with_absolute_paths(relative_to))
            }
        }
    }
}

impl LiquidExportSettings {
    pub fn with_relative_paths<T: AsRef<Path>>(
        &self,
        relative_to: T,
    ) -> Result<LiquidExportSettings, SheetError> {
        Ok(LiquidExportSettings {
            template_file: diff_paths(&self.template_file, relative_to.as_ref())
                .ok_or(SheetError::AbsoluteToRelativePath)?,
            texture_file: diff_paths(&self.texture_file, relative_to.as_ref())
                .ok_or(SheetError::AbsoluteToRelativePath)?,
            metadata_file: diff_paths(&self.metadata_file, relative_to.as_ref())
                .ok_or(SheetError::AbsoluteToRelativePath)?,
            metadata_paths_root: diff_paths(&self.metadata_paths_root, relative_to.as_ref())
                .ok_or(SheetError::AbsoluteToRelativePath)?,
        })
    }

    pub fn with_absolute_paths<T: AsRef<Path>>(&self, relative_to: T) -> LiquidExportSettings {
        LiquidExportSettings {
            template_file: relative_to.as_ref().join(&self.template_file),
            texture_file: relative_to.as_ref().join(&self.texture_file),
            metadata_file: relative_to.as_ref().join(&self.metadata_file),
            metadata_paths_root: relative_to.as_ref().join(&self.metadata_paths_root),
        }
    }

    pub fn template_file(&self) -> &Path {
        self.template_file.as_path()
    }

    pub fn texture_file(&self) -> &Path {
        self.texture_file.as_path()
    }

    pub fn metadata_file(&self) -> &Path {
        self.metadata_file.as_path()
    }

    pub fn metadata_paths_root(&self) -> &Path {
        self.metadata_paths_root.as_path()
    }

    pub fn set_template_file<T: AsRef<Path>>(&mut self, path: T) {
        self.template_file = path.as_ref().to_owned();
    }

    pub fn set_texture_file<T: AsRef<Path>>(&mut self, path: T) {
        self.texture_file = path.as_ref().to_owned();
    }

    pub fn set_metadata_file<T: AsRef<Path>>(&mut self, path: T) {
        self.metadata_file = path.as_ref().to_owned();
    }

    pub fn set_metadata_paths_root<T: AsRef<Path>>(&mut self, path: T) {
        self.metadata_paths_root = path.as_ref().to_owned();
    }
}

#[test]
fn can_read_write_sheet_from_disk() {
    let original = Sheet::read("test-data/sample_sheet_1.tiger").unwrap();
    original.write("test-data/sample_sheet_copy.tiger").unwrap();
    let copy = Sheet::read("test-data/sample_sheet_copy.tiger").unwrap();
    std::fs::remove_file("test-data/sample_sheet_copy.tiger").unwrap();
    assert_eq!(original, copy);
}

#[test]
fn can_add_and_remove_sheet_frame() {
    let mut sheet = Sheet::default();
    sheet.add_frame("frame.png");
    assert!(sheet.has_frame("frame.png"));
    assert!(sheet.frame("frame.png").is_some());
    assert_eq!(
        sheet.frame("frame.png").unwrap().source(),
        Path::new("frame.png")
    );
    sheet.delete_frame("frame.png");
    assert!(!sheet.has_frame("frame.png"));
    assert!(sheet.frame("frame.png").is_none());
}

#[test]
fn deleting_frame_remove_its_usage() {
    let mut sheet = Sheet::default();
    sheet.add_frame("frame.png");

    let (animation_name, animation) = sheet.create_animation();
    animation.apply_direction_preset(DirectionPreset::EightDirections);
    animation
        .sequence_mut(Direction::East)
        .unwrap()
        .insert_keyframe(Keyframe::new(Path::new("frame.png")), 0)
        .unwrap();
    sheet.delete_frame("frame.png");

    assert_eq!(
        0,
        sheet
            .animation(animation_name)
            .unwrap()
            .sequence(Direction::East)
            .unwrap()
            .num_keyframes()
    );
}

#[test]
fn cannot_add_duplicate_sheet_frame() {
    let mut sheet = Sheet::default();
    sheet.add_frame("frame.png");
    sheet.add_frame("frame.png");
    assert_eq!(sheet.frames_iter().count(), 1);
}

#[test]
fn can_add_and_remove_sheet_frames() {
    let mut sheet = Sheet::default();
    assert_eq!(sheet.frames_iter().count(), 0);
    sheet.add_frames(&vec![&Path::new("foo.png"), &Path::new("bar.png")]);
    assert_eq!(sheet.frames_iter().count(), 2);
    sheet.delete_frame(&Path::new("foo.png"));
    assert_eq!(sheet.frames_iter().count(), 1);
}

#[test]
fn can_sort_frames() {
    let frame_a = Frame::new(&Path::new("a"));
    let frame_b = Frame::new(&Path::new("b"));
    let frame_c = Frame::new(&Path::new("c"));
    assert!(frame_a < frame_b);
    assert!(frame_a < frame_c);
    assert!(frame_b < frame_c);
}

#[test]
fn can_add_and_remove_sheet_animation() {
    let mut sheet = Sheet::default();
    let (name_1, _animation) = sheet.create_animation();
    assert!(sheet.has_animation(&name_1));
    assert!(sheet.animation(&name_1).is_some());
    assert!(sheet.animation_mut(&name_1).is_some());
    assert_eq!(sheet.animations_iter().count(), 1);

    let (name_2, _animation) = sheet.create_animation();
    assert!(sheet.has_animation(&name_2));

    sheet.delete_animation(&name_1);
    assert!(!sheet.has_animation(&name_1));
    assert!(sheet.animation(&name_1).is_none());
    assert!(sheet.animation_mut(&name_1).is_none());
    assert!(sheet.has_animation(&name_2));
}

#[test]
fn can_rename_sheet_animation() {
    let mut sheet = Sheet::default();
    let (old_name, _animation) = sheet.create_animation();
    sheet.rename_animation(&old_name, "updated name").unwrap();
    assert!(sheet.animation("updated name").is_some());
    assert!(sheet.animation(&old_name).is_none());
}

#[test]
fn can_rename_sheet_animation_to_same_name() {
    let mut sheet = Sheet::default();
    let (old_name, _animation) = sheet.create_animation();
    sheet.rename_animation(&old_name, &old_name).unwrap();
}

#[test]
fn cannot_rename_sheet_animation_to_existing_name() {
    let mut sheet = Sheet::default();
    let (old_name, _animation) = sheet.create_animation();
    sheet.rename_animation(&old_name, "conflict").unwrap();
    let (old_name, _animation) = sheet.create_animation();
    assert!(sheet.rename_animation(&old_name, "conflict").is_err());
}

#[test]
fn can_read_write_animation_looping() {
    let mut animation = Animation::default();
    animation.set_looping(true);
    assert!(animation.looping());
    animation.set_looping(false);
    assert!(!animation.looping());
}

#[test]
fn can_access_animation_sequences() {
    let mut animation = Animation::default();
    animation.apply_direction_preset(DirectionPreset::FourDirections);
    assert!(animation.sequence(Direction::West).is_some());
    assert!(animation.sequence_mut(Direction::West).is_some());
    assert_eq!(animation.sequences_iter().count(), 4);
    assert_eq!(animation.sequences_iter_mut().count(), 4);
}

#[test]
fn can_animation_can_apply_direction_preset() {
    let mut animation = Animation::default();
    assert_eq!(animation.direction_preset(), None);
    for preset in all::<DirectionPreset>() {
        animation.apply_direction_preset(preset);
        assert_eq!(animation.direction_preset(), Some(preset));
    }
}

#[test]
fn animation_can_recognize_direction_preset() {
    let mut animation = Animation::default();
    animation
        .sequences
        .insert(Direction::NorthEast, Sequence::default());
    animation
        .sequences
        .insert(Direction::NorthWest, Sequence::default());
    animation
        .sequences
        .insert(Direction::SouthEast, Sequence::default());
    assert_eq!(animation.direction_preset(), None);
    animation
        .sequences
        .insert(Direction::SouthWest, Sequence::default());
    assert_eq!(
        animation.direction_preset(),
        Some(DirectionPreset::Isometric)
    );
}

#[test]
fn can_add_and_remove_sequence_keyframe() {
    let mut sequence = Sequence::default();
    let keyframe_a = Keyframe::new(&Path::new("a.png"));
    let keyframe_b = Keyframe::new(&Path::new("b.png"));

    sequence.insert_keyframe(keyframe_a, 0).unwrap();
    sequence.insert_keyframe(keyframe_b, 1).unwrap();
    assert_eq!(sequence.num_keyframes(), 2);
    assert_eq!(sequence.keyframe(0).unwrap().frame(), Path::new("a.png"));
    assert_eq!(
        sequence.keyframe_mut(0).unwrap().frame(),
        Path::new("a.png")
    );
    assert_eq!(sequence.keyframe(1).unwrap().frame(), Path::new("b.png"));
    assert_eq!(
        sequence.keyframe_mut(1).unwrap().frame(),
        Path::new("b.png")
    );

    sequence.delete_keyframe(0).unwrap();
    assert_eq!(sequence.num_keyframes(), 1);
    assert_eq!(sequence.keyframe(0).unwrap().frame(), Path::new("b.png"));
    assert!(sequence.keyframe(1).is_none());
}

#[test]
fn cannot_add_sequence_keyframe_at_illegal_index() {
    let mut sequence = Sequence::default();
    let frame_path = &Path::new("a.png");
    sequence
        .insert_keyframe(Keyframe::new(frame_path), 0)
        .unwrap();
    sequence
        .insert_keyframe(Keyframe::new(frame_path), 0)
        .unwrap();
    assert!(sequence
        .insert_keyframe(Keyframe::new(frame_path), 3)
        .is_err());
}

#[test]
fn can_measure_sequence_duration() {
    let mut sequence = Sequence::default();
    assert_eq!(sequence.duration_millis(), None);

    let mut keyframe_a = Keyframe::new(&Path::new("a.png"));
    let mut keyframe_b = Keyframe::new(&Path::new("b.png"));
    keyframe_a.set_duration_millis(150);
    keyframe_b.set_duration_millis(250);
    sequence.insert_keyframe(keyframe_a, 0).unwrap();
    sequence.insert_keyframe(keyframe_b, 1).unwrap();

    assert_eq!(sequence.duration(), Some(Duration::from_millis(400)));
}

#[test]
fn can_query_sequence_by_time_elapsed() {
    let mut sequence = Sequence::default();
    assert!(sequence.keyframe_at(Duration::default()).is_none());

    let mut keyframe_a = Keyframe::new(&Path::new("a.png"));
    keyframe_a.set_duration_millis(200);
    let mut keyframe_b = Keyframe::new(&Path::new("b.png"));
    keyframe_b.set_duration_millis(200);

    sequence.insert_keyframe(keyframe_a, 0).unwrap();
    sequence.insert_keyframe(keyframe_b, 1).unwrap();
    assert_eq!(sequence.keyframe_times(), vec![0, 200]);

    for (time, frame_index) in [(0, 0), (199, 0), (200, 1), (399, 1), (400, 1), (401, 1)] {
        assert_eq!(
            sequence.keyframe_at(Duration::from_millis(time)).unwrap().0,
            frame_index
        );
        assert_eq!(
            sequence
                .keyframe_at_mut(Duration::from_millis(time))
                .unwrap()
                .0,
            frame_index
        );
    }
}

#[test]
fn can_read_write_keyframe_frame() {
    let frame = Path::new("./example/directory/texture.png");
    let other_frame = Path::new("./example/directory/other_texture.png");
    let mut keyframe = Keyframe::new(frame);
    assert_eq!(keyframe.frame(), frame);
    keyframe.set_frame(other_frame);
    assert_eq!(keyframe.frame(), other_frame);
}

#[test]
fn can_read_write_keyframe_duration() {
    let mut keyframe = Keyframe::new(Path::new("./example/directory/texture.png"));
    keyframe.set_duration_millis(200);
    assert_eq!(keyframe.duration_millis(), 200);
}

#[test]
fn can_read_write_keyframe_offset() {
    let mut keyframe = Keyframe::new(Path::new("./example/directory/texture.png"));
    keyframe.set_offset(vec2(30, 20));
    assert_eq!(keyframe.offset(), vec2(30, 20));
}

#[test]
fn can_add_and_remove_keyframe_hitbox() {
    let mut keyframe = Keyframe::new(Path::new("./example/directory/texture.png"));
    let (name, _hitbox) = keyframe.create_hitbox();
    assert!(keyframe.hitbox(&name).is_some());
    assert!(keyframe.hitbox_mut(&name).is_some());
    assert_eq!(keyframe.hitboxes_iter().count(), 1);
    assert_eq!(keyframe.hitboxes_iter_mut().count(), 1);
    keyframe.delete_hitbox(&name);
    assert!(keyframe.hitbox(&name).is_none());
    assert!(keyframe.hitbox_mut(&name).is_none());
    assert_eq!(keyframe.hitboxes_iter().count(), 0);
    assert_eq!(keyframe.hitboxes_iter_mut().count(), 0);
}

#[test]
fn can_rename_keyframe_hitbox() {
    let frame = Path::new("./example/directory/texture.png");
    let mut keyframe = Keyframe::new(frame);
    let (old_name, _hitbox) = keyframe.create_hitbox();
    keyframe.rename_hitbox(&old_name, "updated name").unwrap();
    assert!(keyframe.hitbox("updated name").is_some());
    assert!(keyframe.hitbox(&old_name).is_none());
}

#[test]
fn can_rename_hitbox_to_existing_name() {
    let frame = Path::new("./example/directory/texture.png");
    let mut keyframe = Keyframe::new(frame);
    let (old_name, _hitbox) = keyframe.create_hitbox();
    keyframe.rename_hitbox(&old_name, "conflict").unwrap();

    let (old_name, _hitbox) = keyframe.create_hitbox();
    assert!(keyframe.rename_hitbox(&old_name, "conflict").is_err());
}

#[test]
fn can_read_write_hitbox_position() {
    let mut hitbox = Hitbox::new();
    hitbox.set_position(vec2(100, 100));
    assert_eq!(hitbox.position(), vec2(100, 100));
}

#[test]
fn can_read_write_hitbox_size() {
    let mut hitbox = Hitbox::new();
    hitbox.set_size(vec2(50, 50));
    assert_eq!(hitbox.size(), vec2(50, 50));
}

#[test]
fn moving_hitbox_preserves_size() {
    let mut hitbox = Hitbox::new();
    hitbox.set_size(vec2(50, 50));
    hitbox.set_position(vec2(0, 0));
    hitbox.set_position(vec2(100, 100));
    assert_eq!(hitbox.size(), vec2(50, 50));
}

#[test]
fn resizing_hitbox_preserves_position() {
    let mut hitbox = Hitbox::new();
    hitbox.set_position(vec2(10, 10));
    hitbox.set_size(vec2(50, 50));
    hitbox.set_size(vec2(100, 100));
    assert_eq!(hitbox.position(), vec2(10, 10));
}

#[test]
fn can_convert_hitbox_to_rectangle() {
    let mut hitbox = Hitbox::new();
    hitbox.set_position(vec2(100, 100));
    hitbox.set_size(vec2(50, 50));
    assert_eq!(hitbox.rectangle(), rect(100, 100, 50, 50));
}

#[test]
fn liquid_export_settings_can_convert_relative_and_absolute_paths() {
    let settings = LiquidExportSettings {
        template_file: Path::new("a/b/format.liquid").to_owned(),
        texture_file: Path::new("a/b/c/sheet.png").to_owned(),
        metadata_file: Path::new("a/b/c/sheet.lua").to_owned(),
        metadata_paths_root: Path::new("a/b").to_owned(),
    };

    let relative = settings.with_relative_paths("a/b").unwrap();
    assert_eq!(relative.template_file, Path::new("format.liquid"));
    assert_eq!(&relative.texture_file, Path::new("c/sheet.png"));
    assert_eq!(&relative.metadata_file, Path::new("c/sheet.lua"));
    assert_eq!(&relative.metadata_paths_root, Path::new(""));

    let absolute = relative.with_absolute_paths("a/b");
    assert_eq!(settings, absolute);
}

#[test]
fn liquid_export_settings_can_adjust_paths() {
    let mut settings = LiquidExportSettings::default();

    let path = Path::new("template_file");
    settings.set_template_file(path);
    assert_eq!(settings.template_file(), path);

    let path = Path::new("texture_file");
    settings.set_texture_file(path);
    assert_eq!(settings.texture_file(), path);

    let path = Path::new("metadata_file");
    settings.set_metadata_file(path);
    assert_eq!(settings.metadata_file(), path);

    let path = Path::new("metadata_paths_root");
    settings.set_metadata_paths_root(path);
    assert_eq!(settings.metadata_paths_root(), path);
}
