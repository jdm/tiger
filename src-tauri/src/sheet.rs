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

#[derive(Serialize, Deserialize, PartialEq, Eq)]
enum Version {
    Tiger1,
    Tiger2,
    Tiger3,
}

const CURRENT_VERSION: Version = Version::Tiger3;
pub use self::version3::*;

#[derive(Error, Debug)]
pub enum SheetError {
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

impl Sheet {
    pub fn read<T: AsRef<Path>>(path: T) -> anyhow::Result<Sheet> {
        #[derive(Deserialize)]
        struct Versioned {
            version: Version,
        }
        let file = File::open(path.as_ref())?;
        let versioned: Versioned = serde_json::from_reader(BufReader::new(file))?;
        let sheet = read_file(versioned.version, &path)?;
        let mut directory = path.as_ref().to_owned();
        directory.pop();
        Ok(sheet.with_absolute_paths(directory))
    }

    pub fn write<T: AsRef<Path>>(&self, path: T) -> anyhow::Result<()> {
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

        let file = BufWriter::new(File::create(path.as_ref())?);
        serde_json::to_writer_pretty(file, &versioned_sheet)?;
        Ok(())
    }

    fn with_relative_paths<T: AsRef<Path>>(&self, relative_to: T) -> Result<Sheet, SheetError> {
        let mut sheet = self.clone();
        for frame in sheet.frames_iter_mut() {
            frame.source = diff_paths(&frame.source, relative_to.as_ref())
                .ok_or(SheetError::AbsoluteToRelativePath)?;
        }
        for (_name, animation) in sheet.animations.iter_mut() {
            for keyframe in animation.keyframes_iter_mut() {
                keyframe.frame = diff_paths(&keyframe.frame, relative_to.as_ref())
                    .ok_or(SheetError::AbsoluteToRelativePath)?;
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
            frame.source = relative_to.as_ref().join(&frame.source);
        }
        for (_name, animation) in sheet.animations.iter_mut() {
            for keyframe in animation.keyframes_iter_mut() {
                keyframe.frame = relative_to.as_ref().join(&&keyframe.frame);
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

    pub fn create_animation(&mut self) -> &mut Animation {
        let mut name = "New Animation".to_owned();
        let mut index = 2;
        while self.has_animation(&name) {
            name = format!("New Animation {}", index);
            index += 1;
        }
        let animation = Animation::default();
        self.animations.insert(name.clone(), animation);
        self.animations.get_mut(&name).unwrap()
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
            animation.timeline.retain(|kf| kf.frame != path.as_ref())
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

    pub fn get_source(&self) -> &Path {
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
    pub fn num_keyframes(&self) -> usize {
        self.timeline.len()
    }

    pub fn looping(&self) -> bool {
        self.is_looping
    }

    pub fn set_looping(&mut self, new_is_looping: bool) {
        self.is_looping = new_is_looping;
    }

    pub fn duration(&self) -> Option<u32> {
        if self.timeline.is_empty() {
            return None;
        }
        Some(self.timeline.iter().map(|f| f.duration).sum())
    }

    pub fn keyframe(&self, index: usize) -> Option<&Keyframe> {
        if index >= self.timeline.len() {
            return None;
        }
        Some(&self.timeline[index])
    }

    pub fn keyframe_mut(&mut self, index: usize) -> Option<&mut Keyframe> {
        if index >= self.timeline.len() {
            return None;
        }
        Some(&mut self.timeline[index])
    }

    pub fn keyframe_index_at(&self, time: Duration) -> Option<usize> {
        let duration = match self.duration() {
            None => return None,
            Some(0) => return None,
            Some(d) => d,
        };
        let time = if self.is_looping {
            Duration::from_millis(time.as_millis() as u64 % u64::from(duration))
        } else {
            time
        };
        let mut cursor = Duration::new(0, 0);
        for (index, frame) in self.timeline.iter().enumerate() {
            cursor += Duration::from_millis(u64::from(frame.duration));
            if time < cursor {
                return Some(index);
            }
        }
        Some(self.timeline.len() - 1)
    }

    pub fn keyframe_at(&self, time: Duration) -> Option<(usize, &Keyframe)> {
        let keyframe_index = self.keyframe_index_at(time)?;
        Some((keyframe_index, self.timeline.get(keyframe_index)?))
    }

    pub fn keyframe_at_mut(&mut self, time: Duration) -> Option<(usize, &mut Keyframe)> {
        let keyframe_index = self.keyframe_index_at(time)?;
        Some((keyframe_index, self.timeline.get_mut(keyframe_index)?))
    }

    pub fn keyframe_times(&self) -> Vec<u64> {
        let mut cursor = 0;
        self.keyframes_iter()
            .map(|f| {
                let t = cursor;
                cursor += u64::from(f.duration());
                t
            })
            .collect()
    }

    pub fn create_keyframe<T: AsRef<Path>>(
        &mut self,
        frame: T,
        index: usize,
    ) -> Result<(), SheetError> {
        if index > self.timeline.len() {
            return Err(SheetError::InvalidFrameIndex(index));
        }
        let keyframe = Keyframe::new(frame);
        self.timeline.insert(index, keyframe);
        Ok(())
    }

    pub fn insert_keyframe(&mut self, keyframe: Keyframe, index: usize) -> Result<(), SheetError> {
        if index > self.timeline.len() {
            return Err(SheetError::InvalidFrameIndex(index));
        }
        self.timeline.insert(index, keyframe);
        Ok(())
    }

    pub fn delete_keyframe(&mut self, index: usize) -> Result<Keyframe, SheetError> {
        if index >= self.timeline.len() {
            return Err(SheetError::InvalidFrameIndex(index));
        }
        Ok(self.timeline.remove(index))
    }

    pub fn keyframes_iter(&self) -> impl Iterator<Item = &Keyframe> {
        self.timeline.iter()
    }

    pub fn keyframes_iter_mut(&mut self) -> impl Iterator<Item = &mut Keyframe> {
        self.timeline.iter_mut()
    }
}

impl Keyframe {
    pub fn new<T: AsRef<Path>>(frame: T) -> Keyframe {
        Keyframe {
            frame: frame.as_ref().to_owned(),
            duration: 100,
            offset: (0, 0),
            hitboxes: BTreeMap::new(),
        }
    }

    pub fn frame(&self) -> &Path {
        &self.frame
    }

    pub fn duration(&self) -> u32 {
        self.duration
    }

    pub fn offset(&self) -> Vector2D<i32> {
        self.offset.into()
    }

    pub fn set_frame<T: AsRef<Path>>(&mut self, new_frame: T) {
        self.frame = new_frame.as_ref().to_owned();
    }

    pub fn set_duration(&mut self, new_duration: u32) {
        self.duration = new_duration;
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
    keyframe.set_duration(200);
    assert_eq!(keyframe.duration(), 200);
}

#[test]
fn can_read_write_keyframe_offset() {
    let mut keyframe = Keyframe::new(Path::new("./example/directory/texture.png"));
    keyframe.set_offset(vec2(30, 20));
    assert_eq!(keyframe.offset(), vec2(30, 20));
}

#[test]
fn can_add_and_remove_keyframe_hitboxes() {
    let mut keyframe = Keyframe::new(Path::new("./example/directory/texture.png"));
    let (name, _hitbox) = keyframe.create_hitbox();
    assert!(keyframe.hitbox(&name).is_some());
    assert!(keyframe.hitbox_mut(&name).is_some());
    keyframe.delete_hitbox(&name);
    assert!(keyframe.hitbox(&name).is_none());
    assert!(keyframe.hitbox_mut(&name).is_none());
}

#[test]
fn can_rename_keyframe_hitboxes() {
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
