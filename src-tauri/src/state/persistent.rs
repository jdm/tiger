#[derive(Clone, Debug, Default)]
pub struct Persistent {
    close_requested: bool,
    timeline_is_playing: bool,
    disk_version: i32,
}

impl Persistent {
    pub fn close_requested(&self) -> bool {
        self.close_requested
    }

    pub fn set_close_requested(&mut self, requested: bool) {
        self.close_requested = requested;
    }

    pub fn is_timeline_playing(&self) -> bool {
        self.timeline_is_playing
    }

    pub fn set_timeline_is_playing(&mut self, playing: bool) {
        self.timeline_is_playing = playing;
    }

    pub fn disk_version(&self) -> i32 {
        self.disk_version
    }

    pub fn set_disk_version(&mut self, version: i32) {
        self.disk_version = version;
    }
}
