use enum_iterator::all;
use euclid::default::*;
use euclid::rect;
use pathdiff::diff_paths;
use regex::Regex;
use serde::{ser::SerializeMap, Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::ops::Range;
use std::path::{Path, PathBuf};
use std::time::Duration;
use sugar_path::SugarPath;
use thiserror::Error;
use uuid::Uuid;

#[cfg(test)]
use euclid::vec2;

pub(in crate::sheet) mod version1;
pub(in crate::sheet) mod version2;
pub(in crate::sheet) mod version3;
pub(in crate::sheet) mod version4;
pub(in crate::sheet) mod version5;

#[derive(Serialize, Deserialize, PartialEq, Eq)]
enum Version {
    Tiger1,
    Tiger2,
    Tiger3,
    Tiger4,
    Tiger5,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Absolute;
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Relative {
    base: PathBuf,
}
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Any;
pub trait Paths: Eq + PartialEq + Ord + PartialOrd {}

impl Paths for Absolute {}
impl Paths for Relative {}
impl Paths for Any {}

impl<P: AsRef<Path>> From<P> for Relative {
    fn from(path: P) -> Self {
        Self {
            base: path.as_ref().to_owned(),
        }
    }
}

const CURRENT_VERSION: Version = Version::Tiger5;
pub use self::version5::*;

#[derive(Error, Debug)]
pub enum SheetError {
    #[error("Expected an absolute path but got: `{0}`")]
    AbsolutePathExpected(PathBuf),
    #[error("Filesystem error for `{0}`: `{1}`")]
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
    #[error("Error converting an absolute path to a relative path\nAbsolute path: `{0}`\nRelative path root: `{1}`")]
    AbsoluteToRelativePath(PathBuf, PathBuf),
    #[error("Animation is missing a keyframe at index `{0}`")]
    InvalidFrameIndex(usize),
    #[error("Expected a relative path but got: `{0}`")]
    RelativePathExpected(PathBuf),
}

impl From<SheetError> for String {
    fn from(e: SheetError) -> Self {
        e.to_string()
    }
}

impl<P: Paths> Sheet<P> {
    pub fn frames_iter(&self) -> std::slice::Iter<'_, Frame<P>> {
        self.frames.iter()
    }

    pub fn sorted_frames(&self) -> Vec<&Frame<P>> {
        let mut frames = self.frames.iter().collect::<Vec<_>>();
        frames.sort();
        frames
    }

    pub fn animations_iter(&self) -> impl Iterator<Item = (&String, &Animation<P>)> {
        self.animations.iter()
    }

    pub fn sorted_animations(&self) -> Vec<(&String, &Animation<P>)> {
        let mut animations = self.animations.iter().collect::<Vec<_>>();
        animations.sort_by_cached_key(|(n, _)| n.to_lowercase());
        animations
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

    pub fn relocate_frames(&mut self, mapping: &HashMap<PathBuf, PathBuf>) {
        for frame in self.frames.iter_mut() {
            if let Some(moved) = mapping.get(&frame.source) {
                frame.source = moved.clone();
            }
        }
        for (_, animation) in self.animations.iter_mut() {
            for (_, sequence) in animation.sequences.iter_mut() {
                for keyframe in sequence.keyframes.iter_mut() {
                    if let Some(moved) = mapping.get(&keyframe.frame) {
                        keyframe.frame = moved.clone();
                    }
                }
            }
        }
    }

    pub fn create_animation<T: AsRef<str>>(
        &mut self,
        proposed_name: T,
    ) -> (String, &mut Animation<P>) {
        let name = generate_unique_name(proposed_name.as_ref(), |n| !self.has_animation(n));
        self.animations.insert(name.clone(), Animation::new());
        (name.clone(), self.animations.get_mut(&name).unwrap())
    }

    pub fn frame<T: AsRef<Path>>(&self, path: T) -> Option<&Frame<P>> {
        self.frames.iter().find(|f| f.source == path.as_ref())
    }

    pub fn animation<T: AsRef<str>>(&self, name: T) -> Option<&Animation<P>> {
        self.animations.get(name.as_ref())
    }

    pub fn animation_mut<T: AsRef<str>>(&mut self, name: T) -> Option<&mut Animation<P>> {
        self.animations.get_mut(name.as_ref())
    }

    pub fn export_settings(&self) -> &Option<ExportSettings<P>> {
        &self.export_settings
    }

    pub fn set_export_settings(&mut self, export_settings: ExportSettings<P>) {
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

impl Sheet<Relative> {
    pub fn with_absolute_paths(self) -> Sheet<Absolute> {
        Sheet {
            frames: self
                .frames
                .into_iter()
                .map(|f| f.with_absolute_paths(&self.paths.base))
                .collect(),
            animations: self
                .animations
                .into_iter()
                .map(|(n, a)| (n, a.with_absolute_paths(&self.paths.base)))
                .collect(),
            export_settings: self
                .export_settings
                .map(|s| s.with_absolute_paths(&self.paths.base)),
            paths: Default::default(),
        }
    }
}

impl Sheet<Any> {
    pub fn read<T: AsRef<Path>>(path: T) -> Result<Self, SheetError> {
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

        let sheet: Self = {
            let file = File::open(path.as_ref())
                .map_err(|e| SheetError::IoError(path.as_ref().to_owned(), e))?;
            let reader = BufReader::new(file);
            read_file(version, reader)?
        };

        Ok(sheet)
    }

    pub fn with_relative_paths<P: AsRef<Path>>(
        self,
        relative_to: P,
    ) -> Result<Sheet<Relative>, SheetError> {
        let export_settings = match self.export_settings {
            Some(s) => Some(s.with_relative_paths()?),
            None => None,
        };
        Ok(Sheet {
            frames: self
                .frames
                .into_iter()
                .map(|f| f.with_relative_paths())
                .collect::<Result<_, _>>()?,
            animations: self
                .animations
                .into_iter()
                .map(|(n, a)| a.with_relative_paths().map(|a| (n, a)))
                .collect::<Result<_, _>>()?,
            export_settings,
            paths: relative_to.as_ref().resolve().into(),
        })
    }
}

impl Sheet<Absolute> {
    pub fn write<T: AsRef<Path>>(self, destination: T) -> Result<(), SheetError> {
        #[derive(Serialize)]
        struct VersionedSheet {
            version: Version,
            sheet: Sheet<Relative>,
        }

        let destination = destination.as_ref().resolve();
        let mut directory = destination.clone();
        directory.pop();

        let versioned_sheet = VersionedSheet {
            version: CURRENT_VERSION,
            sheet: self.with_relative_paths(directory)?,
        };

        let file = File::create(&destination).map_err(|e| SheetError::IoError(destination, e))?;
        serde_json::to_writer_pretty(BufWriter::new(file), &versioned_sheet)?;
        Ok(())
    }

    pub fn with_relative_paths<T: AsRef<Path>>(
        self,
        relative_to: T,
    ) -> Result<Sheet<Relative>, SheetError> {
        let export_settings = match self.export_settings {
            Some(s) => Some(s.with_relative_paths(&relative_to)?),
            None => None,
        };
        Ok(Sheet {
            frames: self
                .frames
                .into_iter()
                .map(|f| f.with_relative_paths(&relative_to))
                .collect::<Result<_, _>>()?,
            animations: self
                .animations
                .into_iter()
                .map(|(n, a)| a.with_relative_paths(&relative_to).map(|a| (n, a)))
                .collect::<Result<_, _>>()?,
            export_settings,
            paths: relative_to.into(),
        })
    }
}

impl<P: Paths> Frame<P> {
    pub fn new<T: AsRef<Path>>(path: T) -> Self {
        Self {
            source: path.as_ref().to_owned(),
            paths: std::marker::PhantomData,
        }
    }

    pub fn source(&self) -> &Path {
        &self.source
    }
}

impl Frame<Relative> {
    pub fn with_absolute_paths<T: AsRef<Path>>(self, relative_to: T) -> Frame<Absolute> {
        Frame {
            source: relative_to.as_ref().join(self.source).resolve(),
            paths: std::marker::PhantomData,
        }
    }
}

impl Frame<Absolute> {
    pub fn with_relative_paths<T: AsRef<Path>>(
        self,
        relative_to: T,
    ) -> Result<Frame<Relative>, SheetError> {
        Ok(Frame {
            source: absolute_to_relative(self.source, relative_to)?,
            paths: std::marker::PhantomData,
        })
    }
}

impl Frame<Any> {
    pub fn with_relative_paths(self) -> Result<Frame<Relative>, SheetError> {
        Ok(Frame {
            source: relative_or_err(self.source)?,
            paths: std::marker::PhantomData,
        })
    }
}

impl<P: Paths> Animation<P> {
    fn new() -> Self {
        Self {
            sequences: Default::default(),
            is_looping: Default::default(),
            key: Uuid::new_v4(),
        }
    }

    pub fn duplicate(&self) -> Animation<P> {
        Animation {
            sequences: self
                .sequences
                .iter()
                .map(|(d, s)| (*d, s.duplicate()))
                .collect(),
            is_looping: self.is_looping,
            key: Uuid::new_v4(),
        }
    }

    pub fn key(&self) -> Uuid {
        self.key
    }

    pub fn looping(&self) -> bool {
        self.is_looping
    }

    pub fn set_looping(&mut self, new_is_looping: bool) {
        self.is_looping = new_is_looping;
    }

    pub fn sequence(&self, direction: Direction) -> Option<&Sequence<P>> {
        self.sequences.get(&direction)
    }

    pub fn sequence_mut(&mut self, direction: Direction) -> Option<&mut Sequence<P>> {
        self.sequences.get_mut(&direction)
    }

    pub fn sequences_iter(&self) -> impl Iterator<Item = (&Direction, &Sequence<P>)> {
        self.sequences.iter()
    }

    pub fn sequences_iter_mut(&mut self) -> impl Iterator<Item = (&Direction, &mut Sequence<P>)> {
        self.sequences.iter_mut()
    }

    pub fn direction_preset(&self) -> Option<DirectionPreset> {
        DirectionPreset::from_directions(self.sequences_iter().map(|(d, _s)| *d))
    }
}

impl<P: Paths + Default> Animation<P> {
    pub fn apply_direction_preset(&mut self, preset: DirectionPreset) {
        let directions = preset.directions();
        self.sequences.retain(|d, _s| directions.contains(d));
        for d in directions {
            self.sequences.entry(d).or_default();
        }
    }
}

impl Animation<Relative> {
    pub fn with_absolute_paths<T: AsRef<Path>>(self, relative_to: T) -> Animation<Absolute> {
        Animation {
            sequences: self
                .sequences
                .into_iter()
                .map(|(d, s)| (d, s.with_absolute_paths(&relative_to)))
                .collect(),
            is_looping: self.is_looping,
            key: self.key,
        }
    }
}

impl Animation<Absolute> {
    pub fn with_relative_paths<T: AsRef<Path>>(
        self,
        relative_to: T,
    ) -> Result<Animation<Relative>, SheetError> {
        Ok(Animation {
            sequences: self
                .sequences
                .into_iter()
                .map(|(d, s)| s.with_relative_paths(&relative_to).map(|s| (d, s)))
                .collect::<Result<_, _>>()?,
            is_looping: self.is_looping,
            key: self.key,
        })
    }
}

impl Animation<Any> {
    pub fn with_relative_paths(self) -> Result<Animation<Relative>, SheetError> {
        Ok(Animation {
            sequences: self
                .sequences
                .into_iter()
                .map(|(d, s)| s.with_relative_paths().map(|s| (d, s)))
                .collect::<Result<_, _>>()?,
            is_looping: self.is_looping,
            key: self.key,
        })
    }
}

impl DirectionPreset {
    pub fn from_directions<T: Iterator<Item = Direction>>(directions: T) -> Option<Self> {
        let directions_set: HashSet<Direction> = directions.collect();
        all::<DirectionPreset>().find(|&preset| directions_set == preset.directions())
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

impl<P: Paths> Sequence<P> {
    pub fn duplicate(&self) -> Sequence<P> {
        Sequence {
            keyframes: self.keyframes.iter().map(Keyframe::duplicate).collect(),
        }
    }

    pub fn num_keyframes(&self) -> usize {
        self.keyframes.len()
    }

    pub fn keyframe(&self, index: usize) -> Option<&Keyframe<P>> {
        if index >= self.keyframes.len() {
            return None;
        }
        Some(&self.keyframes[index])
    }

    pub fn keyframe_mut(&mut self, index: usize) -> Option<&mut Keyframe<P>> {
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

    pub fn keyframe_at(&self, time: Duration) -> Option<(usize, &Keyframe<P>)> {
        let keyframe_index = self.keyframe_index_at(time)?;
        Some((keyframe_index, self.keyframes.get(keyframe_index)?))
    }

    pub fn keyframe_at_mut(&mut self, time: Duration) -> Option<(usize, &mut Keyframe<P>)> {
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

    pub fn insert_keyframe(
        &mut self,
        keyframe: Keyframe<P>,
        index: usize,
    ) -> Result<(), SheetError> {
        if index > self.keyframes.len() {
            return Err(SheetError::InvalidFrameIndex(index));
        }
        self.keyframes.insert(index, keyframe);
        Ok(())
    }

    pub fn delete_keyframe(&mut self, index: usize) -> Result<Keyframe<P>, SheetError> {
        if index >= self.keyframes.len() {
            return Err(SheetError::InvalidFrameIndex(index));
        }
        Ok(self.keyframes.remove(index))
    }

    pub fn keyframes_iter(&self) -> impl Iterator<Item = &Keyframe<P>> {
        self.keyframes.iter()
    }

    pub fn keyframes_iter_mut(&mut self) -> impl Iterator<Item = &mut Keyframe<P>> {
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

impl Sequence<Relative> {
    pub fn with_absolute_paths<T: AsRef<Path>>(self, relative_to: T) -> Sequence<Absolute> {
        Sequence {
            keyframes: self
                .keyframes
                .into_iter()
                .map(|k| k.with_absolute_paths(&relative_to))
                .collect(),
        }
    }
}

impl Sequence<Absolute> {
    pub fn with_relative_paths<T: AsRef<Path>>(
        self,
        relative_to: T,
    ) -> Result<Sequence<Relative>, SheetError> {
        Ok(Sequence {
            keyframes: self
                .keyframes
                .into_iter()
                .map(|k| k.with_relative_paths(&relative_to))
                .collect::<Result<_, _>>()?,
        })
    }
}

impl Sequence<Any> {
    pub fn with_relative_paths(self) -> Result<Sequence<Relative>, SheetError> {
        Ok(Sequence {
            keyframes: self
                .keyframes
                .into_iter()
                .map(|k| k.with_relative_paths())
                .collect::<Result<_, _>>()?,
        })
    }
}

impl<P: Paths> Keyframe<P> {
    pub fn new<T: AsRef<Path>>(frame: T) -> Self {
        Self {
            frame: frame.as_ref().to_owned(),
            duration_millis: 100,
            offset: (0, 0),
            hitboxes: HashMap::new(),
            key: Uuid::new_v4(),
            paths: std::marker::PhantomData,
        }
    }

    pub fn duplicate(&self) -> Self {
        Self {
            frame: self.frame.clone(),
            hitboxes: self
                .hitboxes
                .iter()
                .map(|(n, h)| (n.clone(), h.duplicate()))
                .collect(),
            duration_millis: self.duration_millis,
            offset: self.offset,
            key: Uuid::new_v4(),
            paths: std::marker::PhantomData,
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

    pub fn sorted_hitboxes(&self) -> Vec<(&String, &Hitbox)> {
        let mut hitboxes = self.hitboxes.iter().collect::<Vec<_>>();
        hitboxes.sort_by_cached_key(|(n, _)| n.to_lowercase());
        hitboxes
    }

    pub fn has_hitbox<T: AsRef<str>>(&self, name: T) -> bool {
        self.hitboxes.contains_key(name.as_ref())
    }

    pub fn create_hitbox<T: AsRef<str>>(&mut self, proposed_name: T) -> (String, &mut Hitbox) {
        let name = generate_unique_name(proposed_name.as_ref(), |n| !self.has_hitbox(n));
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

impl Keyframe<Relative> {
    pub fn with_absolute_paths<T: AsRef<Path>>(self, relative_to: T) -> Keyframe<Absolute> {
        Keyframe {
            frame: relative_to.as_ref().join(&self.frame).resolve(),
            hitboxes: self.hitboxes,
            duration_millis: self.duration_millis,
            offset: self.offset,
            key: self.key,
            paths: std::marker::PhantomData,
        }
    }
}

impl Keyframe<Absolute> {
    pub fn with_relative_paths<T: AsRef<Path>>(
        self,
        relative_to: T,
    ) -> Result<Keyframe<Relative>, SheetError> {
        Ok(Keyframe {
            frame: absolute_to_relative(self.frame, relative_to)?,
            hitboxes: self.hitboxes,
            duration_millis: self.duration_millis,
            offset: self.offset,
            key: self.key,
            paths: std::marker::PhantomData,
        })
    }
}

impl Keyframe<Any> {
    pub fn with_relative_paths(self) -> Result<Keyframe<Relative>, SheetError> {
        Ok(Keyframe {
            frame: relative_or_err(self.frame)?,
            hitboxes: self.hitboxes,
            duration_millis: self.duration_millis,
            offset: self.offset,
            key: self.key,
            paths: std::marker::PhantomData,
        })
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

    pub fn duplicate(&self) -> Hitbox {
        Hitbox {
            geometry: self.geometry.clone(),
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

impl Default for Hitbox {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for ExportSettings<Any> {
    fn default() -> Self {
        Self::Template(TemplateExportSettings::<Any>::default())
    }
}

impl ExportSettings<Relative> {
    pub fn with_absolute_paths<T: AsRef<Path>>(self, relative_to: T) -> ExportSettings<Absolute> {
        match self {
            ExportSettings::Template(settings) => {
                ExportSettings::Template(settings.with_absolute_paths(relative_to))
            }
        }
    }
}

impl ExportSettings<Absolute> {
    pub fn with_relative_paths<T: AsRef<Path>>(
        self,
        relative_to: T,
    ) -> Result<ExportSettings<Relative>, SheetError> {
        Ok(match self {
            ExportSettings::Template(settings) => {
                ExportSettings::Template(settings.with_relative_paths(relative_to)?)
            }
        })
    }

    pub fn with_any_paths(self) -> ExportSettings<Any> {
        match self {
            ExportSettings::Template(settings) => {
                ExportSettings::Template(settings.with_any_paths())
            }
        }
    }
}

impl ExportSettings<Any> {
    pub fn with_absolute_paths(self) -> Result<ExportSettings<Absolute>, SheetError> {
        Ok(match self {
            ExportSettings::Template(settings) => {
                ExportSettings::Template(settings.with_absolute_paths()?)
            }
        })
    }

    pub fn with_relative_paths(self) -> Result<ExportSettings<Relative>, SheetError> {
        Ok(match self {
            ExportSettings::Template(settings) => {
                ExportSettings::Template(settings.with_relative_paths()?)
            }
        })
    }
}

impl<P: Paths> TemplateExportSettings<P> {
    pub fn template_file(&self) -> &Path {
        self.template_file.as_path()
    }

    pub fn atlas_image_file(&self) -> &Path {
        self.atlas_image_file.as_path()
    }

    pub fn metadata_file(&self) -> &Path {
        self.metadata_file.as_path()
    }

    pub fn metadata_paths_root(&self) -> &Path {
        self.metadata_paths_root.as_path()
    }
}

impl TemplateExportSettings<Absolute> {
    pub fn with_relative_paths<T: AsRef<Path>>(
        self,
        relative_to: T,
    ) -> Result<TemplateExportSettings<Relative>, SheetError> {
        Ok(TemplateExportSettings {
            template_file: absolute_to_relative(self.template_file, &relative_to)?,
            atlas_image_file: absolute_to_relative(self.atlas_image_file, &relative_to)?,
            metadata_file: absolute_to_relative(self.metadata_file, &relative_to)?,
            metadata_paths_root: absolute_to_relative(self.metadata_paths_root, &relative_to)?,
            paths: std::marker::PhantomData,
        })
    }

    pub fn with_any_paths(self) -> TemplateExportSettings<Any> {
        TemplateExportSettings {
            template_file: self.template_file,
            atlas_image_file: self.atlas_image_file,
            metadata_file: self.metadata_file,
            metadata_paths_root: self.metadata_paths_root,
            paths: std::marker::PhantomData,
        }
    }
}

impl TemplateExportSettings<Relative> {
    pub fn with_absolute_paths<T: AsRef<Path>>(
        &self,
        relative_to: T,
    ) -> TemplateExportSettings<Absolute> {
        TemplateExportSettings {
            template_file: relative_to.as_ref().join(&self.template_file).resolve(),
            atlas_image_file: relative_to.as_ref().join(&self.atlas_image_file).resolve(),
            metadata_file: relative_to.as_ref().join(&self.metadata_file).resolve(),
            metadata_paths_root: relative_to
                .as_ref()
                .join(&self.metadata_paths_root)
                .resolve(),
            paths: std::marker::PhantomData,
        }
    }
}

impl TemplateExportSettings<Any> {
    pub fn set_template_file<T: AsRef<Path>>(&mut self, path: T) {
        self.template_file = path.as_ref().to_owned();
    }

    pub fn set_atlas_image_file<T: AsRef<Path>>(&mut self, path: T) {
        self.atlas_image_file = path.as_ref().to_owned();
    }

    pub fn set_metadata_file<T: AsRef<Path>>(&mut self, path: T) {
        self.metadata_file = path.as_ref().to_owned();
    }

    pub fn set_metadata_paths_root<T: AsRef<Path>>(&mut self, path: T) {
        self.metadata_paths_root = path.as_ref().to_owned();
    }

    pub fn with_absolute_paths(self) -> Result<TemplateExportSettings<Absolute>, SheetError> {
        Ok(TemplateExportSettings {
            template_file: absolute_or_err(self.template_file)?,
            atlas_image_file: absolute_or_err(self.atlas_image_file)?,
            metadata_file: absolute_or_err(self.metadata_file)?,
            metadata_paths_root: absolute_or_err(self.metadata_paths_root)?,
            paths: std::marker::PhantomData,
        })
    }

    pub fn with_relative_paths(self) -> Result<TemplateExportSettings<Relative>, SheetError> {
        Ok(TemplateExportSettings {
            template_file: relative_or_err(self.template_file)?,
            atlas_image_file: relative_or_err(self.atlas_image_file)?,
            metadata_file: relative_or_err(self.metadata_file)?,
            metadata_paths_root: relative_or_err(self.metadata_paths_root)?,
            paths: std::marker::PhantomData,
        })
    }
}

fn generate_unique_name<F: Fn(&str) -> bool>(proposed_name: &str, validate: F) -> String {
    let name_regex = Regex::new(r"(?P<base>.*?)(?P<suffix>\d+)$").unwrap();
    if validate(proposed_name) {
        return proposed_name.to_owned();
    }

    let (base, mut suffix): (String, usize) = name_regex
        .captures(proposed_name)
        .map(|c| {
            (
                c.name("base").unwrap().as_str().to_owned(),
                c.name("suffix").unwrap().as_str().parse().unwrap_or(0),
            )
        })
        .unwrap_or_else(|| (proposed_name.to_owned() + " ", 2));

    loop {
        let name = format!("{base}{suffix}");
        suffix += 1;
        if validate(&name) {
            return name;
        }
    }
}

fn relative_or_err(path: PathBuf) -> Result<PathBuf, SheetError> {
    if path.is_relative() {
        Ok(path)
    } else {
        Err(SheetError::RelativePathExpected(path))
    }
}

fn absolute_or_err(path: PathBuf) -> Result<PathBuf, SheetError> {
    if path.is_absolute() {
        Ok(path)
    } else {
        Err(SheetError::AbsolutePathExpected(path))
    }
}

fn absolute_to_relative<P: AsRef<Path>, B: AsRef<Path>>(
    path: P,
    base: B,
) -> Result<PathBuf, SheetError> {
    diff_paths(&path, &base).ok_or_else(|| {
        SheetError::AbsoluteToRelativePath(path.as_ref().to_owned(), base.as_ref().to_owned())
    })
}

fn ordered_map<V: Serialize, S: serde::Serializer>(
    value: &HashMap<String, V>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let mut sorted = value.iter().collect::<Vec<_>>();
    sorted.sort_by_cached_key(|(k, _)| k.to_lowercase());

    let mut map = serializer.serialize_map(Some(sorted.len()))?;
    for (k, v) in sorted {
        map.serialize_entry(k, v)?;
    }
    map.end()
}

fn ordered_slice<V: Serialize + Ord, S: serde::Serializer>(
    value: &[V],
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let mut sorted = value.iter().collect::<Vec<_>>();
    sorted.sort_unstable();
    sorted.serialize(serializer)
}

fn portable_path<S: serde::Serializer>(value: &Path, serializer: S) -> Result<S::Ok, S::Error> {
    let cleaned = value.to_string_lossy().replace('\\', "/");
    cleaned.serialize(serializer)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn can_read_write_sheet_from_disk() {
        let original = Sheet::<Any>::read("test-data/samurai.tiger")
            .and_then(|s| s.with_relative_paths("test-data"))
            .unwrap()
            .with_absolute_paths();
        original.clone().write("test-data/copy.tiger").unwrap();
        let copy = Sheet::<Any>::read("test-data/copy.tiger")
            .and_then(|s| s.with_relative_paths("test-data"))
            .unwrap()
            .with_absolute_paths();
        std::fs::remove_file("test-data/copy.tiger").unwrap();
        assert_eq!(original, copy);
    }

    #[test]
    fn can_add_and_remove_sheet_frame() {
        let mut sheet = Sheet::<Any>::default();
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
        let mut sheet = Sheet::<Any>::default();
        sheet.add_frame("frame.png");
        sheet.create_animation("Animation");
        sheet
            .animation_mut("Animation")
            .unwrap()
            .apply_direction_preset(DirectionPreset::FourDirections);
        sheet
            .animation_mut("Animation")
            .and_then(|a| a.sequence_mut(Direction::North))
            .unwrap()
            .insert_keyframe(Keyframe::new("frame.png"), 0)
            .unwrap();

        sheet.delete_frame("frame.png");

        assert_eq!(
            0,
            sheet
                .animation("Animation")
                .and_then(|a| a.sequence(Direction::North))
                .map(|s| s.num_keyframes())
                .unwrap()
        );
    }

    #[test]
    fn cannot_add_duplicate_sheet_frame() {
        let mut sheet = Sheet::<Any>::default();
        sheet.add_frame("frame.png");
        sheet.add_frame("frame.png");
        assert_eq!(sheet.frames_iter().count(), 1);
    }

    #[test]
    fn can_add_and_remove_sheet_frames() {
        let mut sheet = Sheet::<Any>::default();
        assert_eq!(sheet.frames_iter().count(), 0);
        sheet.add_frames(&vec![&Path::new("foo.png"), &Path::new("bar.png")]);
        assert_eq!(sheet.frames_iter().count(), 2);
        sheet.delete_frame(Path::new("foo.png"));
        assert_eq!(sheet.frames_iter().count(), 1);
    }

    #[test]
    fn can_sort_frames() {
        let frame_a = Frame::<Any>::new(Path::new("a"));
        let frame_b = Frame::<Any>::new(Path::new("b"));
        let frame_c = Frame::<Any>::new(Path::new("c"));
        assert!(frame_a < frame_b);
        assert!(frame_a < frame_c);
        assert!(frame_b < frame_c);
    }

    #[test]
    fn can_relocate_frame() {
        let mut sheet = Sheet::<Any>::default();
        sheet.add_frame("old.png");
        assert!(sheet.has_frame("old.png"));
        sheet.relocate_frames(&HashMap::from([("old.png".into(), "new.png".into())]));
        assert!(!sheet.has_frame("old.png"));
        assert!(sheet.has_frame("new.png"));
    }

    #[test]
    fn relocating_a_frame_updates_its_usage() {
        let mut sheet = Sheet::<Any>::default();
        sheet.add_frame("old.png");
        sheet.create_animation("Animation");
        sheet
            .animation_mut("Animation")
            .unwrap()
            .apply_direction_preset(DirectionPreset::FourDirections);
        sheet
            .animation_mut("Animation")
            .and_then(|a| a.sequence_mut(Direction::North))
            .unwrap()
            .insert_keyframe(Keyframe::new("old.png"), 0)
            .unwrap();

        sheet.relocate_frames(&HashMap::from([("old.png".into(), "new.png".into())]));

        let keyframe = sheet
            .animation("Animation")
            .and_then(|a| a.sequence(Direction::North))
            .and_then(|s| s.keyframe(0))
            .unwrap();

        assert_eq!(PathBuf::from("new.png"), keyframe.frame);
    }

    #[test]
    fn can_add_and_remove_sheet_animation() {
        let mut sheet = Sheet::<Any>::default();
        let (name_1, _animation) = sheet.create_animation("Animation");
        assert!(sheet.has_animation(&name_1));
        assert!(sheet.animation(&name_1).is_some());
        assert!(sheet.animation_mut(&name_1).is_some());
        assert_eq!(sheet.animations_iter().count(), 1);

        let (name_2, _animation) = sheet.create_animation("Animation");
        assert!(sheet.has_animation(&name_2));

        sheet.delete_animation(&name_1);
        assert!(!sheet.has_animation(&name_1));
        assert!(sheet.animation(&name_1).is_none());
        assert!(sheet.animation_mut(&name_1).is_none());
        assert!(sheet.has_animation(&name_2));
    }

    #[test]
    fn can_rename_sheet_animation() {
        let mut sheet = Sheet::<Any>::default();
        let (old_name, _animation) = sheet.create_animation("Animation");
        sheet.rename_animation(&old_name, "updated name").unwrap();
        assert!(sheet.animation("updated name").is_some());
        assert!(sheet.animation(&old_name).is_none());
    }

    #[test]
    fn can_rename_sheet_animation_to_same_name() {
        let mut sheet = Sheet::<Any>::default();
        let (old_name, _animation) = sheet.create_animation("Animation");
        sheet.rename_animation(&old_name, &old_name).unwrap();
    }

    #[test]
    fn cannot_rename_sheet_animation_to_existing_name() {
        let mut sheet = Sheet::<Any>::default();
        let (old_name, _animation) = sheet.create_animation("Animation");
        sheet.rename_animation(&old_name, "conflict").unwrap();
        let (old_name, _animation) = sheet.create_animation("Animation");
        assert!(sheet.rename_animation(&old_name, "conflict").is_err());
    }

    #[test]
    fn can_read_write_animation_looping() {
        let mut animation = Animation::<Any>::new();
        animation.set_looping(true);
        assert!(animation.looping());
        animation.set_looping(false);
        assert!(!animation.looping());
    }

    #[test]
    fn can_access_animation_sequences() {
        let mut animation = Animation::<Any>::new();
        animation.apply_direction_preset(DirectionPreset::FourDirections);
        assert!(animation.sequence(Direction::West).is_some());
        assert!(animation.sequence_mut(Direction::West).is_some());
        assert_eq!(animation.sequences_iter().count(), 4);
        assert_eq!(animation.sequences_iter_mut().count(), 4);
    }

    #[test]
    fn can_animation_can_apply_direction_preset() {
        let mut animation = Animation::<Any>::new();
        assert_eq!(animation.direction_preset(), None);
        for preset in all::<DirectionPreset>() {
            animation.apply_direction_preset(preset);
            assert_eq!(animation.direction_preset(), Some(preset));
        }
    }

    #[test]
    fn animation_can_recognize_direction_preset() {
        let mut animation = Animation::<Any>::new();
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
        let mut sequence = Sequence::<Any>::default();
        let keyframe_a = Keyframe::new(Path::new("a.png"));
        let keyframe_b = Keyframe::new(Path::new("b.png"));

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
        let mut sequence = Sequence::<Any>::default();
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
        let mut sequence = Sequence::<Any>::default();
        assert_eq!(sequence.duration_millis(), None);

        let mut keyframe_a = Keyframe::new(Path::new("a.png"));
        let mut keyframe_b = Keyframe::new(Path::new("b.png"));
        keyframe_a.set_duration_millis(150);
        keyframe_b.set_duration_millis(250);
        sequence.insert_keyframe(keyframe_a, 0).unwrap();
        sequence.insert_keyframe(keyframe_b, 1).unwrap();

        assert_eq!(sequence.duration(), Some(Duration::from_millis(400)));
    }

    #[test]
    fn can_query_sequence_by_time_elapsed() {
        let mut sequence = Sequence::<Any>::default();
        assert!(sequence.keyframe_at(Duration::default()).is_none());

        let mut keyframe_a = Keyframe::<Any>::new(Path::new("a.png"));
        keyframe_a.set_duration_millis(200);
        let mut keyframe_b = Keyframe::<Any>::new(Path::new("b.png"));
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
    fn can_read_keyframe_frame() {
        let frame = Path::new("./example/directory/texture.png");
        let keyframe = Keyframe::<Relative>::new(frame);
        assert_eq!(keyframe.frame(), frame);
    }

    #[test]
    fn can_read_write_keyframe_duration() {
        let mut keyframe = Keyframe::<Relative>::new(Path::new("./example/directory/texture.png"));
        keyframe.set_duration_millis(200);
        assert_eq!(keyframe.duration_millis(), 200);
    }

    #[test]
    fn can_read_write_keyframe_offset() {
        let mut keyframe = Keyframe::<Relative>::new(Path::new("./example/directory/texture.png"));
        keyframe.set_offset(vec2(30, 20));
        assert_eq!(keyframe.offset(), vec2(30, 20));
    }

    #[test]
    fn can_add_and_remove_keyframe_hitbox() {
        let mut keyframe = Keyframe::<Relative>::new(Path::new("./example/directory/texture.png"));
        let (name, _hitbox) = keyframe.create_hitbox("Hitbox");
        assert!(keyframe.has_hitbox(&name));
        assert_eq!(keyframe.hitboxes_iter().count(), 1);
        assert_eq!(keyframe.hitboxes_iter_mut().count(), 1);
        keyframe.delete_hitbox(&name);
        assert!(!keyframe.has_hitbox(&name));
        assert_eq!(keyframe.hitboxes_iter().count(), 0);
        assert_eq!(keyframe.hitboxes_iter_mut().count(), 0);
    }

    #[test]
    fn can_rename_keyframe_hitbox() {
        let frame = Path::new("./example/directory/texture.png");
        let mut keyframe = Keyframe::<Relative>::new(frame);
        let (old_name, _hitbox) = keyframe.create_hitbox("Hitbox");
        keyframe.rename_hitbox(&old_name, "updated name").unwrap();
        assert!(keyframe.has_hitbox("updated name"));
        assert!(!keyframe.has_hitbox(&old_name));
    }

    #[test]
    fn can_rename_hitbox_to_existing_name() {
        let frame = Path::new("./example/directory/texture.png");
        let mut keyframe = Keyframe::<Relative>::new(frame);
        let (old_name, _hitbox) = keyframe.create_hitbox("Hitbox");
        keyframe.rename_hitbox(&old_name, "conflict").unwrap();

        let (old_name, _hitbox) = keyframe.create_hitbox("Hitbox");
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
    fn template_export_settings_can_convert_relative_and_absolute_paths() {
        let absolute = TemplateExportSettings::<Any> {
            template_file: PathBuf::from("a/b/format.template").resolve(),
            atlas_image_file: PathBuf::from("a/b/c/sheet.png").resolve(),
            metadata_file: PathBuf::from("a/b/c/sheet.lua").resolve(),
            metadata_paths_root: PathBuf::from("a/b").resolve(),
            paths: std::marker::PhantomData,
        }
        .with_absolute_paths()
        .unwrap();

        let relative = absolute
            .clone()
            .with_relative_paths(PathBuf::from("a/b").resolve())
            .unwrap();
        assert_eq!(relative.template_file, Path::new("format.template"));
        assert_eq!(&relative.atlas_image_file, Path::new("c/sheet.png"));
        assert_eq!(&relative.metadata_file, Path::new("c/sheet.lua"));
        assert_eq!(&relative.metadata_paths_root, Path::new(""));

        let roundtrip = relative.with_absolute_paths("a/b");
        assert_eq!(roundtrip, absolute);
    }

    #[test]
    fn template_export_settings_can_adjust_paths() {
        let mut settings = TemplateExportSettings::<Any>::default();

        let path = Path::new("template_file");
        settings.set_template_file(path);
        assert_eq!(settings.template_file(), path);

        let path = Path::new("atlas_image_file");
        settings.set_atlas_image_file(path);
        assert_eq!(settings.atlas_image_file(), path);

        let path = Path::new("metadata_file");
        settings.set_metadata_file(path);
        assert_eq!(settings.metadata_file(), path);

        let path = Path::new("metadata_paths_root");
        settings.set_metadata_paths_root(path);
        assert_eq!(settings.metadata_paths_root(), path);
    }

    #[test]
    fn generate_unique_name_respects_suggestions() {
        assert_eq!("oink", generate_unique_name("oink", |_| true));
    }

    #[test]
    fn generate_unique_name_finds_workarounds() {
        assert_eq!("oink 2", generate_unique_name("oink", |n| n != "oink"));
        assert_eq!(
            "oink 3",
            generate_unique_name("oink", |n| n != "oink" && n != "oink 2")
        );
    }

    #[test]
    fn generate_unique_name_detects_existing_numbers() {
        assert_eq!("oink 3", generate_unique_name("oink 2", |n| n != "oink 2"));
    }
}
