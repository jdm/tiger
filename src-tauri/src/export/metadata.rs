use euclid::default::*;
use pathdiff::diff_paths;
use std::collections::HashMap;
use std::path::PathBuf;
use thiserror::Error;

use crate::export::*;

type TextureLayout = HashMap<PathBuf, PackedFrame>;

#[derive(Error, Debug)]
pub enum MetadataError {
    #[error("Template parser initialization error")]
    ParserInitError,
    #[error("Template parsing error\n\n{0}")]
    TemplateParsingError(liquid::Error),
    #[error("Template rendering error\n\n{0}")]
    TemplateRenderingError(liquid::Error),
    #[error("An animation references a frame which is not part of the sheet")]
    InvalidFrameReference,
    #[error("The sheet contains a frame which was not packed into the texture atlas")]
    FrameWasNotPacked,
    #[error("Error converting an absolute path to a relative path")]
    AbsoluteToRelativePath,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct LiquidHitbox {
    name: String,
    left: i32,
    right: i32,
    top: i32,
    bottom: i32,
    width: i32,
    height: i32,
}

fn liquid_data_from_hitbox(
    hitbox_name: String,
    hitbox: &Hitbox,
) -> Result<LiquidHitbox, MetadataError> {
    Ok(LiquidHitbox {
        name: hitbox_name,
        left: hitbox.position().x,
        right: hitbox.position().x + hitbox.size().x as i32,
        top: hitbox.position().y,
        bottom: hitbox.position().y + hitbox.size().y as i32,
        width: hitbox.size().x as i32,
        height: hitbox.size().y as i32,
    })
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct LiquidFrame {
    source: String,
    index: i32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

fn liquid_data_from_frame(
    sheet: &Sheet,
    frame: &Frame,
    settings: &LiquidExportSettings,
    texture_layout: &TextureLayout,
) -> Result<LiquidFrame, MetadataError> {
    let index = sheet
        .frames_iter()
        .position(|f| std::ptr::eq(f, frame))
        .ok_or(MetadataError::InvalidFrameReference)?;

    let frame_layout = texture_layout
        .get(frame.source())
        .ok_or(MetadataError::FrameWasNotPacked)?;

    let source_path = diff_paths(frame.source(), settings.metadata_paths_root())
        .ok_or(MetadataError::AbsoluteToRelativePath)?
        .to_string_lossy()
        .into_owned();

    Ok(LiquidFrame {
        source: source_path,
        index: index as i32,
        x: frame_layout.position_in_sheet.0 as i32,
        y: frame_layout.position_in_sheet.1 as i32,
        width: frame_layout.size_in_sheet.0 as i32,
        height: frame_layout.size_in_sheet.1 as i32,
    })
}
#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct LiquidKeyframe {
    duration: i32,
    center_offset_x: i32,
    center_offset_y: i32,
    top_left_offset_x: i32,
    top_left_offset_y: i32,
    frame: LiquidFrame,
    hitboxes: Vec<LiquidHitbox>,
}

fn liquid_data_from_keyframe(
    sheet: &Sheet,
    keyframe: &Keyframe,
    settings: &LiquidExportSettings,
    texture_layout: &TextureLayout,
) -> Result<LiquidKeyframe, MetadataError> {
    let packed_frame = texture_layout
        .get(keyframe.frame())
        .ok_or(MetadataError::FrameWasNotPacked)?;

    let frame_size: Vector2D<u32> = packed_frame.size_in_sheet.into();
    let center_offset = keyframe.offset();
    let top_left_offset = center_offset - (frame_size.to_f32() / 2.0).floor().to_i32();

    let frame = sheet
        .frame(keyframe.frame())
        .ok_or(MetadataError::InvalidFrameReference)?;
    let frame_data = liquid_data_from_frame(sheet, frame, settings, texture_layout)?;

    let mut hitboxes = Vec::new();
    for (hitbox_name, hitbox) in keyframe.hitboxes_iter() {
        hitboxes.push(liquid_data_from_hitbox(hitbox_name.clone(), hitbox)?);
    }

    Ok(LiquidKeyframe {
        duration: keyframe.duration_millis() as i32,
        center_offset_x: center_offset.x,
        center_offset_y: center_offset.y,
        top_left_offset_x: top_left_offset.x,
        top_left_offset_y: top_left_offset.y,
        frame: frame_data,
        hitboxes,
    })
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct LiquidSequence {
    direction: Direction,
    keyframes: Vec<LiquidKeyframe>,
}

fn liquid_data_from_sequence(
    sheet: &Sheet,
    direction: Direction,
    sequence: &Sequence,
    settings: &LiquidExportSettings,
    texture_layout: &TextureLayout,
) -> Result<LiquidSequence, MetadataError> {
    let mut keyframes = Vec::new();
    for keyframe in sequence.keyframes_iter() {
        let frame = liquid_data_from_keyframe(sheet, keyframe, settings, texture_layout)?;
        keyframes.push(frame);
    }

    Ok(LiquidSequence {
        direction,
        keyframes,
    })
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct LiquidAnimation {
    name: String,
    is_looping: bool,
    sequences: Vec<LiquidSequence>,
}

fn liquid_data_from_animation(
    sheet: &Sheet,
    animation_name: String,
    animation: &Animation,
    settings: &LiquidExportSettings,
    texture_layout: &TextureLayout,
) -> Result<LiquidAnimation, MetadataError> {
    let mut sequences = Vec::new();
    for (direction, sequence) in animation.sequences_iter() {
        let sequence =
            liquid_data_from_sequence(sheet, *direction, sequence, settings, texture_layout)?;
        sequences.push(sequence);
    }

    Ok(LiquidAnimation {
        name: animation_name,
        is_looping: animation.looping(),
        sequences,
    })
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct LiquidSheet {
    frames: Vec<LiquidFrame>,
    animations: Vec<LiquidAnimation>,
    sheet_image: String,
}

fn liquid_data_from_sheet(
    sheet: &Sheet,
    settings: &LiquidExportSettings,
    texture_layout: &TextureLayout,
) -> Result<LiquidSheet, MetadataError> {
    let frames = {
        let mut frames = Vec::new();
        for frame in sheet.frames_iter() {
            frames.push(liquid_data_from_frame(
                sheet,
                frame,
                settings,
                texture_layout,
            )?);
        }
        frames
    };

    let animations = {
        let mut animations = Vec::new();
        for (animation_name, animation) in sheet.animations_iter() {
            let animation_data = liquid_data_from_animation(
                sheet,
                animation_name.clone(),
                animation,
                settings,
                texture_layout,
            )?;
            animations.push(animation_data);
        }
        animations
    };

    let sheet_image = {
        let relative_to = settings.metadata_paths_root();
        let image_path = diff_paths(&settings.texture_file(), relative_to)
            .ok_or(MetadataError::AbsoluteToRelativePath)?;
        image_path.to_string_lossy().into_owned()
    };

    Ok(LiquidSheet {
        frames,
        animations,
        sheet_image,
    })
}

pub(super) fn generate_sheet_metadata(
    sheet: &Sheet,
    export_settings: &ExportSettings,
    texture_layout: &TextureLayout,
) -> Result<String, MetadataError> {
    match export_settings {
        ExportSettings::Liquid(liquid_settings) => {
            let template = liquid::ParserBuilder::with_stdlib()
                .build()
                .map_err(|_| MetadataError::ParserInitError)?
                .parse_file(&liquid_settings.template_file())
                .map_err(MetadataError::TemplateParsingError)?;
            let globals = liquid_data_from_sheet(sheet, liquid_settings, texture_layout)?;
            let liquid_sheet =
                liquid::to_object(&globals).map_err(MetadataError::TemplateRenderingError)?;
            let output = template
                .render(&liquid_sheet)
                .map_err(MetadataError::TemplateRenderingError)?;
            Ok(output)
        }
    }
}
