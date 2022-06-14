use image::ImageError;
use std::{fs::File, io::Write};
use thiserror::Error;

use crate::sheet::*;

mod metadata;
mod texture;

pub use metadata::*;
pub use texture::*;

#[derive(Error, Debug)]
pub enum ExportError {
    #[error("Missing export settings")]
    NoExportSettings,
    #[error("Filesystem error")]
    IoError(#[from] std::io::Error),
    #[error("Metadata generation error")]
    MetadataError(#[from] MetadataError),
    #[error("Texture packing error")]
    TeturePackingError(#[from] PackError),
    #[error("Texture packing error")]
    TetureStorageError(#[from] ImageError),
}

pub fn export_sheet(sheet: &Sheet) -> Result<(), ExportError> {
    let export_settings = sheet
        .export_settings()
        .as_ref()
        .ok_or(ExportError::NoExportSettings)?;

    match export_settings {
        ExportSettings::Liquid(liquid_settings) => {
            // TODO texture export performance is awful
            let packed_sheet = pack_sheet(&sheet)?;
            let metadata =
                generate_sheet_metadata(&sheet, &export_settings, &packed_sheet.get_layout())?;

            {
                let mut file = File::create(liquid_settings.metadata_file())?;
                file.write_all(&metadata.into_bytes())?;
            }
            {
                let mut file = File::create(liquid_settings.texture_file())?;
                packed_sheet
                    .get_texture()
                    .write_to(&mut file, image::ImageFormat::Png)?;
            }
        }
    }

    Ok(())
}
