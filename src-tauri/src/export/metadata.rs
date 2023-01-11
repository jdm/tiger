use euclid::default::*;
use handlebars::{handlebars_helper, Handlebars};
use pathdiff::diff_paths;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use thiserror::Error;

use crate::export::PackedFrame;
use crate::sheet::{self, Absolute};

type TextureLayout = HashMap<PathBuf, PackedFrame>;

#[derive(Error, Debug)]
pub enum MetadataError {
    #[error("Template parsing error\n\n{0}")]
    TemplateParsingError(Box<handlebars::TemplateError>),
    #[error("Template rendering error\n\n{0}")]
    TemplateRenderingError(Box<handlebars::RenderError>),
    #[error("An animation references a frame which is not part of the sheet")]
    InvalidFrameReference,
    #[error("The sheet contains a frame which was not packed into the texture atlas")]
    FrameWasNotPacked,
    #[error("Error converting an absolute path to a relative path\nAbsolute path: `{0}`\nRelative path root: `{1}`")]
    AbsoluteToRelativePath(PathBuf, PathBuf),
}

handlebars_helper!(add: |a:f64, b:f64| a + b);
handlebars_helper!(divide: |a:f64, b:f64| if b == 0.0 { 0.0 } else { a / b });
handlebars_helper!(multiply: |a:f64, b:f64| a * b);
handlebars_helper!(subtract: |a:f64, b:f64| a - b);

pub struct Template<'a> {
    handlebars: Handlebars<'a>,
}

impl<'a> Template<'a> {
    const TEMPLATE_NAME: &str = "template";

    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, MetadataError> {
        let mut handlebars = Handlebars::new();
        handlebars_misc_helpers::setup_handlebars(&mut handlebars);
        handlebars.register_helper("add", Box::new(add));
        handlebars.register_helper("divide", Box::new(divide));
        handlebars.register_helper("multiply", Box::new(multiply));
        handlebars.register_helper("subtract", Box::new(subtract));
        handlebars
            .register_template_file(Self::TEMPLATE_NAME, path.as_ref())
            .map_err(|e| MetadataError::TemplateParsingError(Box::new(e)))?;
        Ok(Self { handlebars })
    }

    fn render(&self, sheet: &Sheet) -> Result<String, MetadataError> {
        self.handlebars
            .render(Self::TEMPLATE_NAME, sheet)
            .map_err(|e| MetadataError::TemplateRenderingError(Box::new(e)))
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Hitbox {
    name: String,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl Hitbox {
    fn new(hitbox_name: String, hitbox: &sheet::Hitbox) -> Result<Hitbox, MetadataError> {
        Ok(Self {
            name: hitbox_name,
            x: hitbox.position().x,
            y: hitbox.position().y,
            width: hitbox.size().x as i32,
            height: hitbox.size().y as i32,
        })
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Frame {
    index: i32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl Frame {
    fn new(
        sheet: &sheet::Sheet<Absolute>,
        frame: &sheet::Frame<Absolute>,
        texture_layout: &TextureLayout,
    ) -> Result<Self, MetadataError> {
        let index = sheet
            .sorted_frames()
            .into_iter()
            .position(|f| std::ptr::eq(f, frame))
            .ok_or(MetadataError::InvalidFrameReference)?;

        let frame_layout = texture_layout
            .get(frame.source())
            .ok_or(MetadataError::FrameWasNotPacked)?;

        Ok(Self {
            index: index as i32,
            x: frame_layout.position_in_sheet.0 as i32,
            y: frame_layout.position_in_sheet.1 as i32,
            width: frame_layout.size_in_sheet.0 as i32,
            height: frame_layout.size_in_sheet.1 as i32,
        })
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Keyframe {
    duration: i32,
    x: i32,
    y: i32,
    frame: Frame,
    hitboxes: Vec<Hitbox>,
}

impl Keyframe {
    fn new(
        sheet: &sheet::Sheet<Absolute>,
        keyframe: &sheet::Keyframe<Absolute>,
        texture_layout: &TextureLayout,
    ) -> Result<Self, MetadataError> {
        let packed_frame = texture_layout
            .get(keyframe.frame())
            .ok_or(MetadataError::FrameWasNotPacked)?;

        let frame_size: Vector2D<u32> = packed_frame.size_in_sheet.into();
        let position = keyframe.offset() - (frame_size.to_f32() / 2.0).floor().to_i32();

        let frame = sheet
            .frame(keyframe.frame())
            .ok_or(MetadataError::InvalidFrameReference)?;
        let frame_data = Frame::new(sheet, frame, texture_layout)?;

        let mut hitboxes = Vec::new();
        for (hitbox_name, hitbox) in keyframe.sorted_hitboxes() {
            hitboxes.push(Hitbox::new(hitbox_name.clone(), hitbox)?);
        }

        Ok(Keyframe {
            duration: keyframe.duration_millis() as i32,
            x: position.x,
            y: position.y,
            frame: frame_data,
            hitboxes,
        })
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Sequence {
    direction: sheet::Direction,
    keyframes: Vec<Keyframe>,
}

impl Sequence {
    fn new(
        sheet: &sheet::Sheet<Absolute>,
        direction: sheet::Direction,
        sequence: &sheet::Sequence<Absolute>,
        texture_layout: &TextureLayout,
    ) -> Result<Self, MetadataError> {
        let mut keyframes = Vec::new();
        for keyframe in sequence.keyframes_iter() {
            let frame = Keyframe::new(sheet, keyframe, texture_layout)?;
            keyframes.push(frame);
        }

        Ok(Self {
            direction,
            keyframes,
        })
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Animation {
    name: String,
    is_looping: bool,
    sequences: Vec<Sequence>,
}

impl Animation {
    fn new(
        sheet: &sheet::Sheet<Absolute>,
        animation_name: String,
        animation: &sheet::Animation<Absolute>,
        texture_layout: &TextureLayout,
    ) -> Result<Self, MetadataError> {
        let mut sequences = Vec::new();
        for (direction, sequence) in animation.sequences_iter() {
            let sequence = Sequence::new(sheet, *direction, sequence, texture_layout)?;
            sequences.push(sequence);
        }

        Ok(Self {
            name: animation_name,
            is_looping: animation.looping(),
            sequences,
        })
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Sheet {
    frames: Vec<Frame>,
    animations: Vec<Animation>,
    sheet_image: PathBuf,
}

impl Sheet {
    fn new(
        sheet: &sheet::Sheet<Absolute>,
        settings: &sheet::TemplateExportSettings<Absolute>,
        texture_layout: &TextureLayout,
    ) -> Result<Self, MetadataError> {
        let frames = {
            let mut frames = Vec::new();
            for frame in sheet.sorted_frames() {
                frames.push(Frame::new(sheet, frame, texture_layout)?);
            }
            frames
        };

        let animations = {
            let mut animations = Vec::new();
            for (animation_name, animation) in sheet.sorted_animations() {
                let animation_data =
                    Animation::new(sheet, animation_name.clone(), animation, texture_layout)?;
                animations.push(animation_data);
            }
            animations
        };

        let sheet_image = {
            let relative_to = settings.metadata_paths_root();
            let path = diff_paths(settings.texture_file(), relative_to).ok_or_else(|| {
                MetadataError::AbsoluteToRelativePath(
                    settings.texture_file().to_owned(),
                    relative_to.to_owned(),
                )
            })?;
            path.with_forward_slashes()
        };

        Ok(Self {
            frames,
            animations,
            sheet_image,
        })
    }
}

pub(super) fn generate_sheet_metadata(
    sheet: &sheet::Sheet<Absolute>,
    export_settings: &sheet::ExportSettings<Absolute>,
    texture_layout: &TextureLayout,
) -> Result<String, MetadataError> {
    match export_settings {
        sheet::ExportSettings::Template(template_settings) => {
            let template = Template::new(template_settings.template_file())?;
            let globals = Sheet::new(sheet, template_settings, texture_layout)?;
            template.render(&globals)
        }
    }
}

trait WithForwardSlashes {
    fn with_forward_slashes(&self) -> PathBuf;
}

impl<T: AsRef<Path>> WithForwardSlashes for T {
    fn with_forward_slashes(&self) -> PathBuf {
        self.as_ref().to_string_lossy().replace('\\', "/").into()
    }
}
