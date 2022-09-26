use image::ImageError;
use std::{
    fs::{create_dir_all, File},
    io::Write,
};
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
    #[error("{0}")]
    IoError(#[from] std::io::Error),
    #[error("{0}")]
    MetadataError(#[from] MetadataError),
    #[error("{0}")]
    TexturePackingError(#[from] PackError),
    #[error("{0}")]
    TextureStorageError(#[from] ImageError),
}

pub fn export_sheet(sheet: &Sheet) -> Result<(), ExportError> {
    let export_settings = sheet
        .export_settings()
        .as_ref()
        .ok_or(ExportError::NoExportSettings)?;

    match export_settings {
        ExportSettings::Liquid(liquid_settings) => {
            // TODO texture export performance is awful
            let packed_sheet = pack_sheet(sheet)?;
            let metadata =
                generate_sheet_metadata(sheet, export_settings, packed_sheet.get_layout())?;

            {
                let path = liquid_settings.metadata_file();
                if let Some(directory) = path.parent() {
                    create_dir_all(directory)?;
                }
                let mut file = File::create(path)?;
                file.write_all(&metadata.into_bytes())?;
            }

            {
                let path = liquid_settings.texture_file();
                if let Some(directory) = path.parent() {
                    create_dir_all(directory)?;
                }
                let mut file = File::create(path)?;
                packed_sheet
                    .get_texture()
                    .write_to(&mut file, image::ImageFormat::Png)?;
            }
        }
    }

    Ok(())
}
