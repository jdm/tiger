use std::path::Path;

use crate::document::*;
use crate::export::parse_template;

#[derive(Clone, Debug)]
pub enum ExportSettingsValidation {
    Liquid(LiquidExportSettingsValidation),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExportSettingsError {
    ExpectedAbsolutePath,
    ExpectedDirectory,
    ExpectedFile,
    FileNotFound,
    TemplateParseError(String),
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct LiquidExportSettingsValidation {
    template_file_error: Option<ExportSettingsError>,
    texture_file_error: Option<ExportSettingsError>,
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

    pub(super) fn liquid_export_settings_mut(
        &mut self,
    ) -> DocumentResult<&mut LiquidExportSettings<Any>> {
        match self.export_settings_edit_mut()? {
            ExportSettings::Liquid(settings) => Ok(settings),
        }
    }

    pub(super) fn begin_export_as(&mut self) {
        self.persistent.export_settings_edit = self
            .sheet
            .export_settings()
            .as_ref()
            .cloned()
            .map(|s| s.with_any_paths())
            .or_else(|| Some(ExportSettings::<Any>::new()));
    }

    pub(super) fn cancel_export_as(&mut self) {
        self.persistent.export_settings_edit = None;
    }

    pub(super) fn set_export_template_file<T: AsRef<Path>>(
        &mut self,
        file: T,
    ) -> DocumentResult<()> {
        self.liquid_export_settings_mut()?.set_template_file(file);
        Ok(())
    }

    pub(super) fn set_export_texture_file<T: AsRef<Path>>(
        &mut self,
        file: T,
    ) -> DocumentResult<()> {
        self.liquid_export_settings_mut()?.set_texture_file(file);
        Ok(())
    }

    pub(super) fn set_export_metadata_file<T: AsRef<Path>>(
        &mut self,
        file: T,
    ) -> DocumentResult<()> {
        self.liquid_export_settings_mut()?.set_metadata_file(file);
        Ok(())
    }

    pub(super) fn set_export_metadata_paths_root<T: AsRef<Path>>(
        &mut self,
        directory: T,
    ) -> DocumentResult<()> {
        self.liquid_export_settings_mut()?
            .set_metadata_paths_root(directory);
        Ok(())
    }

    pub fn validate_export_settings(&self) -> DocumentResult<ExportSettingsValidation> {
        let validation = match self.export_settings_edit()? {
            ExportSettings::Liquid(l) => {
                ExportSettingsValidation::Liquid(self.validate_liquid_export_settings(l))
            }
        };
        Ok(validation)
    }

    fn validate_liquid_export_settings(
        &self,
        settings: &LiquidExportSettings<Any>,
    ) -> LiquidExportSettingsValidation {
        LiquidExportSettingsValidation {
            template_file_error: validate_template_path(settings.template_file()),
            texture_file_error: validate_output_file_path(settings.texture_file()),
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

impl LiquidExportSettingsValidation {
    pub fn template_file_error(&self) -> Option<&ExportSettingsError> {
        self.template_file_error.as_ref()
    }

    pub fn texture_file_error(&self) -> Option<&ExportSettingsError> {
        self.texture_file_error.as_ref()
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
        parse_template(path)
            .err()
            .map(|e| ExportSettingsError::TemplateParseError(e.to_string()))
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
