use crate::sheet::*;
use crate::state::*;

#[derive(Clone, Debug, Default)]
pub struct Persistent {
    pub(super) disk_version: i32,
    pub(super) close_requested: bool,
    pub(super) timeline_is_playing: bool,
    pub(super) export_settings_edit: Option<ExportSettings>,
}

impl Document {
    pub fn close_requested(&self) -> bool {
        self.persistent.close_requested
    }

    pub fn is_timeline_playing(&self) -> bool {
        self.persistent.timeline_is_playing
    }
}
