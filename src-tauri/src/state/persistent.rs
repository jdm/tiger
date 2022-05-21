#[derive(Clone, Debug, Default)]
pub struct Persistent {
    close_state: Option<CloseState>,
    timeline_is_playing: bool,
    disk_version: i32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CloseState {
    Requested,
    Saving,
    Allowed,
}

impl Persistent {
    pub fn is_timeline_playing(&self) -> bool {
        self.timeline_is_playing
    }

    pub fn set_timeline_is_playing(&mut self, playing: bool) {
        self.timeline_is_playing = playing;
    }

    pub fn set_disk_version(&mut self, version: i32) {
        self.disk_version = version;
    }
}
