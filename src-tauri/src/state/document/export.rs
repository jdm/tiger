use std::path::Path;

use crate::export::parse_template;
use crate::sheet::*;
use crate::state::*;

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
    pub fn export_settings_edit(&self) -> Result<&ExportSettings, DocumentError> {
        self.persistent
            .export_settings_edit
            .as_ref()
            .ok_or(DocumentError::NotEditingExportSettings)
    }

    pub(super) fn export_settings_edit_mut(
        &mut self,
    ) -> Result<&mut ExportSettings, DocumentError> {
        self.persistent
            .export_settings_edit
            .as_mut()
            .ok_or(DocumentError::NotEditingExportSettings)
    }

    pub(super) fn liquid_export_settings_mut(
        &mut self,
    ) -> Result<&mut LiquidExportSettings, DocumentError> {
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
            .or_else(|| Some(ExportSettings::new()));
    }

    pub(super) fn cancel_export_as(&mut self) {
        self.persistent.export_settings_edit = None;
    }

    pub(super) fn set_export_template_file<T: AsRef<Path>>(
        &mut self,
        file: T,
    ) -> Result<(), DocumentError> {
        self.liquid_export_settings_mut()?.set_template_file(file);
        Ok(())
    }

    pub(super) fn set_export_texture_file<T: AsRef<Path>>(
        &mut self,
        file: T,
    ) -> Result<(), DocumentError> {
        self.liquid_export_settings_mut()?.set_texture_file(file);
        Ok(())
    }

    pub(super) fn set_export_metadata_file<T: AsRef<Path>>(
        &mut self,
        file: T,
    ) -> Result<(), DocumentError> {
        self.liquid_export_settings_mut()?.set_metadata_file(file);
        Ok(())
    }

    pub(super) fn set_export_metadata_paths_root<T: AsRef<Path>>(
        &mut self,
        directory: T,
    ) -> Result<(), DocumentError> {
        self.liquid_export_settings_mut()?
            .set_metadata_paths_root(directory);
        Ok(())
    }

    pub fn validate_export_settings(&self) -> Result<ExportSettingsValidation, DocumentError> {
        let validation = match self.export_settings_edit()? {
            ExportSettings::Liquid(l) => {
                ExportSettingsValidation::Liquid(self.validate_liquid_export_settings(l))
            }
        };
        Ok(validation)
    }

    fn validate_liquid_export_settings(
        &self,
        settings: &LiquidExportSettings,
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

    pub(super) fn end_export_as(&mut self) -> Result<(), DocumentError> {
        let export_settings = self.export_settings_edit_mut()?.clone();
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
