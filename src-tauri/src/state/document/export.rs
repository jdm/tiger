use std::path::Path;

use crate::sheet::*;
use crate::state::*;

impl Document {
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

    pub(super) fn end_export_as(&mut self) -> Result<(), DocumentError> {
        let export_settings = self.export_settings_edit_mut()?.clone();
        self.sheet.set_export_settings(export_settings);
        self.persistent.export_settings_edit = None;
        Ok(())
    }
}
