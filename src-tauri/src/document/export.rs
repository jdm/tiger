use std::path::Path;

use crate::document::*;
use crate::export::Template;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExportSettingsValidation {
    Template(TemplateExportSettingsValidation),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExportSettingsError {
    ExpectedAbsolutePath,
    ExpectedDirectory,
    ExpectedFile,
    FileNotFound,
    TemplateError(String),
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TemplateExportSettingsValidation {
    template_file_error: Option<ExportSettingsError>,
    atlas_image_file_error: Option<ExportSettingsError>,
    metadata_file_error: Option<ExportSettingsError>,
    metadata_paths_root_error: Option<ExportSettingsError>,
}

impl Document {
    pub fn export_settings_edit(&self) -> DocumentResult<&ExportSettings<Any>> {
        self.persistent
            .export_settings_edit
            .as_ref()
            .ok_or(DocumentError::NotEditingExportSettings)
    }

    pub(super) fn export_settings_edit_mut(&mut self) -> DocumentResult<&mut ExportSettings<Any>> {
        self.persistent
            .export_settings_edit
            .as_mut()
            .ok_or(DocumentError::NotEditingExportSettings)
    }

    pub(super) fn template_export_settings_mut(
        &mut self,
    ) -> DocumentResult<&mut TemplateExportSettings<Any>> {
        match self.export_settings_edit_mut()? {
            ExportSettings::Template(settings) => Ok(settings),
        }
    }

    pub(super) fn begin_export_as(&mut self) {
        self.persistent.export_settings_edit = self
            .sheet
            .export_settings()
            .as_ref()
            .cloned()
            .map(|s| s.with_any_paths())
            .or_else(|| Some(ExportSettings::<Any>::default()));
    }

    pub(super) fn cancel_export_as(&mut self) {
        self.persistent.export_settings_edit = None;
    }

    pub(super) fn set_export_template_file<T: AsRef<Path>>(
        &mut self,
        file: T,
    ) -> DocumentResult<()> {
        self.template_export_settings_mut()?.set_template_file(file);
        Ok(())
    }

    pub(super) fn set_export_atlas_image_file<T: AsRef<Path>>(
        &mut self,
        file: T,
    ) -> DocumentResult<()> {
        self.template_export_settings_mut()?
            .set_atlas_image_file(file);
        Ok(())
    }

    pub(super) fn set_export_metadata_file<T: AsRef<Path>>(
        &mut self,
        file: T,
    ) -> DocumentResult<()> {
        self.template_export_settings_mut()?.set_metadata_file(file);
        Ok(())
    }

    pub(super) fn set_export_metadata_paths_root<T: AsRef<Path>>(
        &mut self,
        directory: T,
    ) -> DocumentResult<()> {
        self.template_export_settings_mut()?
            .set_metadata_paths_root(directory);
        Ok(())
    }

    pub fn validate_export_settings(&self) -> DocumentResult<ExportSettingsValidation> {
        let validation = match self.export_settings_edit()? {
            ExportSettings::Template(s) => {
                ExportSettingsValidation::Template(self.validate_template_export_settings(s))
            }
        };
        Ok(validation)
    }

    fn validate_template_export_settings(
        &self,
        settings: &TemplateExportSettings<Any>,
    ) -> TemplateExportSettingsValidation {
        TemplateExportSettingsValidation {
            template_file_error: validate_template_path(settings.template_file()),
            atlas_image_file_error: validate_output_file_path(settings.atlas_image_file()),
            metadata_file_error: validate_output_file_path(settings.metadata_file()),
            metadata_paths_root_error: validate_output_directory_path(
                settings.metadata_paths_root(),
            ),
        }
    }

    pub(super) fn end_export_as(&mut self) -> DocumentResult<()> {
        let export_settings = self
            .export_settings_edit_mut()?
            .clone()
            .with_absolute_paths()?;
        self.sheet.set_export_settings(export_settings);
        self.persistent.export_settings_edit = None;
        Ok(())
    }
}

impl TemplateExportSettingsValidation {
    pub fn template_file_error(&self) -> Option<&ExportSettingsError> {
        self.template_file_error.as_ref()
    }

    pub fn atlas_image_file_error(&self) -> Option<&ExportSettingsError> {
        self.atlas_image_file_error.as_ref()
    }

    pub fn metadata_file_error(&self) -> Option<&ExportSettingsError> {
        self.metadata_file_error.as_ref()
    }

    pub fn metadata_paths_root_error(&self) -> Option<&ExportSettingsError> {
        self.metadata_paths_root_error.as_ref()
    }
}

fn validate_template_path(path: &Path) -> Option<ExportSettingsError> {
    if path.is_relative() {
        Some(ExportSettingsError::ExpectedAbsolutePath)
    } else if path.is_dir() {
        Some(ExportSettingsError::ExpectedFile)
    } else if !path.exists() {
        Some(ExportSettingsError::FileNotFound)
    } else {
        Template::new(path)
            .err()
            .map(|e| ExportSettingsError::TemplateError(e.to_string()))
    }
}

fn validate_output_file_path(p: &Path) -> Option<ExportSettingsError> {
    if p.is_relative() {
        Some(ExportSettingsError::ExpectedAbsolutePath)
    } else if p.is_dir() {
        Some(ExportSettingsError::ExpectedFile)
    } else {
        None
    }
}

fn validate_output_directory_path(p: &Path) -> Option<ExportSettingsError> {
    if p.is_relative() {
        Some(ExportSettingsError::ExpectedAbsolutePath)
    } else if p.is_file() {
        Some(ExportSettingsError::ExpectedDirectory)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {

    use sugar_path::SugarPath;

    use super::*;
    use crate::app::{mock::TigerAppMock, TigerApp};

    #[tokio::test]
    async fn export_matches_known_output() {
        let app = TigerAppMock::new();
        app.open_documents(vec!["test-data/samurai.tiger"]).await;

        let ExportSettings::Template(export_settings) = {
            let state_handle = app.state();
            let state = state_handle.lock();
            state
                .current_document()
                .and_then(|d| d.sheet().export_settings().as_ref())
                .unwrap()
                .to_owned()
        };

        std::fs::remove_file(export_settings.atlas_image_file()).ok();
        std::fs::remove_file(export_settings.metadata_file()).ok();
        app.export().await;

        assert_eq!(
            std::fs::read_to_string(export_settings.metadata_file()).unwrap(),
            std::fs::read_to_string("test-data/samurai.export").unwrap()
        );

        assert_eq!(
            std::fs::read(export_settings.atlas_image_file()).unwrap(),
            std::fs::read("test-data/samurai.png").unwrap()
        );
    }

    #[tokio::test]
    async fn can_adjust_export_settings() {
        let template_file = PathBuf::from("test-data/only-frames.template").resolve();
        let metadata_root = PathBuf::from("test-output/root").resolve();
        let atlas_image_file =
            PathBuf::from("test-output/can_adjust_export_settings.png").resolve();
        let metadata_file =
            PathBuf::from("test-output/can_adjust_export_settings.export").resolve();

        let app = TigerAppMock::new();
        app.open_documents(vec!["test-data/samurai.tiger"]).await;
        app.begin_export_as();
        app.set_export_template_file(template_file);
        app.set_export_metadata_paths_root(metadata_root);
        app.set_export_atlas_image_file(&atlas_image_file);
        app.set_export_metadata_file(&metadata_file);
        app.end_export_as().await;

        assert_eq!(
            std::fs::read_to_string(metadata_file).unwrap(),
            std::fs::read_to_string("test-output/can_adjust_export_settings.export").unwrap()
        );

        assert_eq!(
            std::fs::read(atlas_image_file).unwrap(),
            std::fs::read("test-data/samurai.png").unwrap()
        );
    }

    #[tokio::test]
    async fn can_cancel_export_as() {
        let atlas_image_file = PathBuf::from("test-output/can_cancel_export_as.png").resolve();
        let template_file = PathBuf::from("test-output/can_cancel_export_as.export").resolve();

        std::fs::remove_file(&atlas_image_file).ok();
        std::fs::remove_file(&template_file).ok();

        let app = TigerAppMock::new();
        app.open_documents(vec!["test-data/samurai.tiger"]).await;
        app.begin_export_as();
        assert!(app.document().export_settings_being_edited.is_some());
        app.set_export_atlas_image_file(&atlas_image_file);
        app.set_export_metadata_file(&template_file);
        app.cancel_export_as();
        assert!(app.document().export_settings_being_edited.is_none());

        assert!(!atlas_image_file.exists());
        assert!(!template_file.exists());
    }

    #[test]
    fn validates_empty_paths_in_export_settings() {
        let mut d = Document::new("tmp.tiger");
        d.begin_export_as();
        assert_eq!(
            d.validate_export_settings().unwrap(),
            ExportSettingsValidation::Template(TemplateExportSettingsValidation {
                template_file_error: Some(ExportSettingsError::ExpectedAbsolutePath),
                atlas_image_file_error: Some(ExportSettingsError::ExpectedAbsolutePath),
                metadata_file_error: Some(ExportSettingsError::ExpectedAbsolutePath),
                metadata_paths_root_error: Some(ExportSettingsError::ExpectedAbsolutePath)
            })
        );
    }

    #[test]
    fn validates_relative_paths_in_export_settings() {
        let mut d = Document::new("tmp.tiger");
        d.begin_export_as();
        d.set_export_template_file("relative/path.template")
            .unwrap();
        d.set_export_atlas_image_file("relative/path.png").unwrap();
        d.set_export_metadata_file("relative/path.json").unwrap();
        d.set_export_metadata_paths_root("relative/").unwrap();
        assert_eq!(
            d.validate_export_settings().unwrap(),
            ExportSettingsValidation::Template(TemplateExportSettingsValidation {
                template_file_error: Some(ExportSettingsError::ExpectedAbsolutePath),
                atlas_image_file_error: Some(ExportSettingsError::ExpectedAbsolutePath),
                metadata_file_error: Some(ExportSettingsError::ExpectedAbsolutePath),
                metadata_paths_root_error: Some(ExportSettingsError::ExpectedAbsolutePath)
            })
        );
    }

    #[test]
    fn validates_files_vs_dirs_in_export_settings() {
        let mut d = Document::new("tmp.tiger");
        let dir = std::env::current_dir().unwrap();
        let file = PathBuf::from("test-data/samurai.tiger")
            .canonicalize()
            .unwrap();
        d.begin_export_as();
        d.set_export_template_file(&dir).unwrap();
        d.set_export_atlas_image_file(&dir).unwrap();
        d.set_export_metadata_file(&dir).unwrap();
        d.set_export_metadata_paths_root(file).unwrap();
        assert_eq!(
            d.validate_export_settings().unwrap(),
            ExportSettingsValidation::Template(TemplateExportSettingsValidation {
                template_file_error: Some(ExportSettingsError::ExpectedFile),
                atlas_image_file_error: Some(ExportSettingsError::ExpectedFile),
                metadata_file_error: Some(ExportSettingsError::ExpectedFile),
                metadata_paths_root_error: Some(ExportSettingsError::ExpectedDirectory)
            })
        );
    }

    #[test]
    fn validates_template_file() {
        type Test = fn(e: Option<ExportSettingsError>) -> bool;
        let test_table: Vec<(&str, Test)> = vec![
            ("test-data/export.template", |e| e.is_none()),
            ("test-data/samurai-dead-all.png", |e| {
                matches!(e, Some(ExportSettingsError::TemplateError(_)))
            }),
        ];

        let mut d = Document::new("tmp.tiger");
        d.begin_export_as();
        for (path, test) in test_table {
            let absolute_path = PathBuf::from(path).canonicalize().unwrap();
            d.set_export_template_file(absolute_path).unwrap();
            let ExportSettingsValidation::Template(validation) =
                d.validate_export_settings().unwrap();
            assert!(test(validation.template_file_error));
        }
    }
}
