use image::ImageError;
use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::{Path, PathBuf},
};
use thiserror::Error;

use crate::features::texture_cache;
use crate::sheet::*;

mod metadata;
mod texture;

pub use metadata::*;
pub use texture::*;

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
    texture_cache: texture_cache::CacheHandle,
) -> Result<(), ExportError> {
    let export_settings = sheet
        .export_settings()
        .as_ref()
        .ok_or(ExportError::NoExportSettings)?;

    match export_settings {
        ExportSettings::Template(template_settings) => {
            let packed_sheet = pack_sheet(sheet, texture_cache)?;
            let metadata = generate_sheet_metadata(sheet, export_settings, packed_sheet.layout())?;

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
                let path = template_settings.texture_file();
                if let Some(directory) = path.parent() {
                    create_dir(directory)?;
                }
                let mut file = create_file(path)?;
                packed_sheet
                    .texture()
                    .write_to(&mut file, image::ImageFormat::Png)?;
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

#[cfg(test)]
mod test {

    use parking_lot::Mutex;
    use std::collections::HashMap;
    use std::sync::Arc;

    use super::*;
    use crate::{app, document};

    #[test]
    fn export_matches_known_output() {
        let texture_cache = Arc::new(Mutex::new(HashMap::new()));
        let mut app = app::App::default();
        app.open_document(document::Document::open("test-data/samurai.tiger").unwrap());

        let document = app.current_document().unwrap();
        let ExportSettings::Template(export_settings) =
            document.sheet().export_settings().as_ref().unwrap();

        std::fs::remove_file(export_settings.texture_file()).ok();
        std::fs::remove_file(export_settings.metadata_file()).ok();
        export_sheet(document.sheet(), texture_cache).unwrap();

        assert_eq!(
            std::fs::read_to_string(export_settings.metadata_file()).unwrap(),
            std::fs::read_to_string("test-data/samurai.export").unwrap()
        );

        assert_eq!(
            std::fs::read(export_settings.texture_file()).unwrap(),
            std::fs::read("test-data/samurai.png").unwrap()
        );
    }
}
