use crate::state::document::KeyframeDurationDrag;

#[derive(Debug, Default)]
pub struct Transient {
    pub(super) keyframe_duration_drag: Option<KeyframeDurationDrag>,
}

impl Transient {
    pub fn is_dragging_keyframe_duration(&self) -> bool {
        self.keyframe_duration_drag.is_some()
    }
}
