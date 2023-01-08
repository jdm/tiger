use image::{DynamicImage, GenericImage};
use std::collections::HashMap;
use std::path::PathBuf;
use thiserror::Error;

use crate::features::texture_cache;
use crate::sheet::{Absolute, Sheet};

#[derive(Error, Debug)]
pub enum PackError {
    #[error("Error while reading a frame")]
    FrameRead,
    #[error("Error while packing textures")]
    Packing,
}

pub(super) struct PackedFrame {
    pub position_in_sheet: (u32, u32),
    pub size_in_sheet: (u32, u32),
}

pub(super) struct PackedSheet {
    texture: DynamicImage,
    layout: HashMap<PathBuf, PackedFrame>,
}

impl PackedSheet {
    pub fn texture(&self) -> &DynamicImage {
        &self.texture
    }

    pub fn layout(&self) -> &HashMap<PathBuf, PackedFrame> {
        &self.layout
    }
}

pub(super) fn pack_sheet(
    sheet: &Sheet<Absolute>,
    texture_cache: texture_cache::Handle,
) -> Result<PackedSheet, PackError> {
    let mut bitmaps = HashMap::new();
    {
        let cache = texture_cache.lock();
        for frame in sheet.frames_iter() {
            bitmaps.insert(
                frame.source(),
                cache
                    .get(frame.source())
                    .cloned()
                    .or_else(|| image::open(frame.source()).ok())
                    .ok_or(PackError::FrameRead)?,
            );
        }
    }

    let mut items = bitmaps
        .iter()
        .map(|(path, bitmap)| crunch::Item {
            data: path,
            w: bitmap.width() as usize,
            h: bitmap.height() as usize,
            rot: crunch::Rotation::None,
        })
        .collect::<Vec<_>>();
    items.sort_by_key(|i| i.data);

    let (width, height, layout) =
        crunch::pack_into_po2(8_192, items).map_err(|_| PackError::Packing)?;
    let layout = layout
        .into_iter()
        .map(|(r, p)| {
            (
                p.to_path_buf(),
                PackedFrame {
                    position_in_sheet: (r.x as u32, r.y as u32),
                    size_in_sheet: (r.w as u32, r.h as u32),
                },
            )
        })
        .collect::<HashMap<_, _>>();

    let mut texture = DynamicImage::new_rgba8(width as u32, height as u32);
    layout.iter().for_each(|(path, frame)| {
        let bitmap = bitmaps.get(path.as_path()).unwrap();
        let (x, y) = (frame.position_in_sheet.0, frame.position_in_sheet.1);
        texture.copy_from(bitmap, x, y).unwrap();
    });

    Ok(PackedSheet { texture, layout })
}
