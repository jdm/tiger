#[derive(Clone, Debug, Default)]
pub struct Persistent {
    pub(in crate::state) close_requested: bool,
    pub(in crate::state) timeline_is_playing: bool,
    pub(in crate::state) disk_version: i32,
}

impl Persistent {
    pub fn close_requested(&self) -> bool {
        self.close_requested
    }

    pub fn is_timeline_playing(&self) -> bool {
        self.timeline_is_playing
    }

    pub fn disk_version(&self) -> i32 {
        self.disk_version
    }
}
