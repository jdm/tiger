use image::ImageError;
use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::{Path, PathBuf},
};
use thiserror::Error;

use crate::features::texture_cache;
use crate::sheet::*;

mod atlas;
mod metadata;

pub use atlas::*;
pub use metadata::*;

#[derive(Error, Debug)]
pub enum ExportError {
    #[error("Missing export settings")]
    NoExportSettings,
    #[error("Filesystem error for `{0}`: `{1}`")]
    IoError(PathBuf, std::io::Error),
    #[error("{0}")]
    MetadataError(#[from] MetadataError),
    #[error("{0}")]
    TexturePackingError(#[from] PackError),
    #[error("{0}")]
    TextureStorageError(#[from] ImageError),
}

pub fn export_sheet(
    sheet: &Sheet<Absolute>,
    texture_cache: texture_cache::Handle,
) -> Result<(), ExportError> {
    let export_settings = sheet
        .export_settings()
        .as_ref()
        .ok_or(ExportError::NoExportSettings)?;

    match export_settings {
        ExportSettings::Template(template_settings) => {
            let atlas = pack_sheet(sheet, texture_cache)?;
            let metadata = generate_sheet_metadata(sheet, export_settings, &atlas)?;

            {
                let path = template_settings.metadata_file();
                if let Some(directory) = path.parent() {
                    create_dir(directory)?;
                }
                let mut file = create_file(path)?;
                file.write_all(&metadata.into_bytes())
                    .map_err(|e| ExportError::IoError(path.to_owned(), e))?;
            }

            {
                let path = template_settings.atlas_image_file();
                if let Some(directory) = path.parent() {
                    create_dir(directory)?;
                }
                let mut file = create_file(path)?;
                atlas.image().write_to(&mut file, image::ImageFormat::Png)?;
            }
        }
    }

    Ok(())
}

fn create_file(path: &Path) -> Result<File, ExportError> {
    File::create(path).map_err(|e| ExportError::IoError(path.to_owned(), e))
}

fn create_dir(path: &Path) -> Result<(), ExportError> {
    create_dir_all(path).map_err(|e| ExportError::IoError(path.to_owned(), e))
}
