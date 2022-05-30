use core::cmp::Ordering;
use euclid::default::*;
use euclid::rect;
#[cfg(test)]
use euclid::vec2;
use pathdiff::diff_paths;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::path::{Path, PathBuf};
use std::time::Duration;
use thiserror::Error;

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
            .ok_or(SheetError::AnimationNotFound(old_name.as_ref().to_owned()))?;
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
            cursor += Duration::from_millis(u64::from(frame.duration_millis));
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
        let mut cursor = 0;
        self.keyframes_iter()
            .map(|f| {
                let t = cursor;
                cursor += u64::from(f.duration_millis());
                t
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
            .ok_or(SheetError::HitboxNotFound(old_name.as_ref().to_owned()))?;
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
            linked: true,
            locked: false,
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

    pub fn linked(&self) -> bool {
        self.linked
    }

    pub fn set_linked(&mut self, linked: bool) {
        self.linked = linked
    }

    pub fn locked(&self) -> bool {
        self.locked
    }

    pub fn set_locked(&mut self, locked: bool) {
        self.locked = locked
    }
}

impl ExportSettings {
    pub fn new() -> ExportSettings {
        ExportSettings {
            format: ExportFormat::Template(PathBuf::new()),
            texture_destination: PathBuf::new(),
            metadata_destination: PathBuf::new(),
            metadata_paths_root: PathBuf::new(),
        }
    }

    pub fn with_relative_paths<T: AsRef<Path>>(
        &self,
        relative_to: T,
    ) -> Result<ExportSettings, SheetError> {
        Ok(ExportSettings {
            format: self.format.with_relative_paths(&relative_to)?,
            texture_destination: diff_paths(&self.texture_destination, relative_to.as_ref())
                .ok_or(SheetError::AbsoluteToRelativePath)?,
            metadata_destination: diff_paths(&self.metadata_destination, relative_to.as_ref())
                .ok_or(SheetError::AbsoluteToRelativePath)?,
            metadata_paths_root: diff_paths(&self.metadata_paths_root, relative_to.as_ref())
                .ok_or(SheetError::AbsoluteToRelativePath)?,
        })
    }

    pub fn with_absolute_paths<T: AsRef<Path>>(&self, relative_to: T) -> ExportSettings {
        ExportSettings {
            format: self.format.with_absolute_paths(&relative_to),
            texture_destination: relative_to.as_ref().join(&self.texture_destination),
            metadata_destination: relative_to.as_ref().join(&self.metadata_destination),
            metadata_paths_root: relative_to.as_ref().join(&self.metadata_paths_root),
        }
    }
}

impl ExportFormat {
    pub fn with_relative_paths<T: AsRef<Path>>(
        &self,
        relative_to: T,
    ) -> Result<ExportFormat, SheetError> {
        match self {
            ExportFormat::Template(p) => Ok(ExportFormat::Template(
                diff_paths(&p, relative_to.as_ref()).ok_or(SheetError::AbsoluteToRelativePath)?,
            )),
        }
    }

    pub fn with_absolute_paths<T: AsRef<Path>>(&self, relative_to: T) -> ExportFormat {
        match self {
            ExportFormat::Template(p) => ExportFormat::Template(relative_to.as_ref().join(&p)),
        }
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
    sheet.delete_frame("frame.png");
    assert!(!sheet.has_frame("frame.png"));
    assert!(sheet.frame("frame.png").is_none());
}

#[test]
fn can_add_and_remove_sheet_animation() {
    let mut sheet = Sheet::default();
    let (name, _animation) = sheet.create_animation();
    assert!(sheet.has_animation(&name));
    assert!(sheet.animation(&name).is_some());
    assert!(sheet.animation_mut(&name).is_some());
    sheet.delete_animation(&name);
    assert!(!sheet.has_animation(&name));
    assert!(sheet.animation(&name).is_none());
    assert!(sheet.animation_mut(&name).is_none());
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
fn can_read_write_animation_looping() {
    let mut animation = Animation::default();
    animation.set_looping(true);
    assert_eq!(animation.looping(), true);
    animation.set_looping(false);
    assert_eq!(animation.looping(), false);
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
fn can_measure_sequence_duration() {
    let mut sequence = Sequence::default();
    assert_eq!(sequence.duration_millis(), None);

    let mut keyframe_a = Keyframe::new(&Path::new("a.png"));
    let mut keyframe_b = Keyframe::new(&Path::new("b.png"));
    keyframe_a.set_duration_millis(150);
    keyframe_b.set_duration_millis(250);
    sequence.insert_keyframe(keyframe_a, 0).unwrap();
    sequence.insert_keyframe(keyframe_b, 1).unwrap();

    assert_eq!(sequence.duration_millis(), Some(400));
}

#[test]
fn can_query_sequence_by_time_elapsed() {
    let mut sequence = Sequence::default();
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
    keyframe.delete_hitbox(&name);
    assert!(keyframe.hitbox(&name).is_none());
    assert!(keyframe.hitbox_mut(&name).is_none());
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
fn can_read_write_hitbox_linked() {
    let mut hitbox = Hitbox::new();
    hitbox.set_linked(true);
    assert_eq!(hitbox.linked(), true);
    hitbox.set_linked(false);
    assert_eq!(hitbox.linked(), false);
}

#[test]
fn can_read_write_hitbox_locked() {
    let mut hitbox = Hitbox::new();
    hitbox.set_locked(true);
    assert_eq!(hitbox.locked(), true);
    hitbox.set_locked(false);
    assert_eq!(hitbox.locked(), false);
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
fn export_settings_can_convert_relative_and_absolute_paths() {
    let settings = ExportSettings {
        texture_destination: Path::new("a/b/c/sheet.png").to_owned(),
        metadata_destination: Path::new("a/b/c/sheet.lua").to_owned(),
        metadata_paths_root: Path::new("a/b").to_owned(),
        format: ExportFormat::Template(Path::new("a/b/format.liquid").to_owned()),
    };

    let relative = settings.with_relative_paths("a/b").unwrap();
    assert_eq!(&relative.texture_destination, Path::new("c/sheet.png"));
    assert_eq!(&relative.metadata_destination, Path::new("c/sheet.lua"));
    assert_eq!(&relative.metadata_paths_root, Path::new(""));
    assert_eq!(
        relative.format,
        ExportFormat::Template(Path::new("format.liquid").to_owned())
    );

    let absolute = relative.with_absolute_paths("a/b");
    assert_eq!(settings, absolute);
}
