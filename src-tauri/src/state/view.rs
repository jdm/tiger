use euclid::default::*;
use std::time::Duration;

use crate::sheet::Direction;
use crate::state::MultiSelection;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ContentTab {
    Frames,
    Animations,
}

#[derive(Clone, Debug, PartialEq)]
pub struct View {
    content_tab: ContentTab,
    selection: MultiSelection,
    current_animation: Option<String>,
    current_sequence: Option<Direction>,
    workbench_offset: Vector2D<f32>,
    timeline_clock: Duration,
    workbench_zoom_level: i32,
    timeline_zoom_level: i32,
}

impl Default for View {
    fn default() -> View {
        View {
            content_tab: ContentTab::Frames,
            selection: Default::default(),
            current_animation: None,
            current_sequence: None,
            workbench_offset: Vector2D::<f32>::zero(), // Should this be an integer?
            workbench_zoom_level: 1,
            timeline_zoom_level: 1,
            timeline_clock: Default::default(),
        }
    }
}

impl View {
    pub fn content_tab(&self) -> ContentTab {
        self.content_tab
    }

    pub fn set_content_tab(&mut self, content_tab: ContentTab) {
        self.content_tab = content_tab;
    }

    pub fn selection(&self) -> &MultiSelection {
        &self.selection
    }

    pub fn selection_mut(&mut self) -> &mut MultiSelection {
        &mut self.selection
    }

    pub fn current_animation(&self) -> &Option<String> {
        &self.current_animation
    }

    pub fn clear_current_animation(&mut self) {
        self.current_animation = None;
    }

    pub fn set_current_animation<T: AsRef<str>>(&mut self, name: T) {
        self.current_animation = Some(name.as_ref().to_owned());
    }

    pub fn current_sequence(&self) -> &Option<Direction> {
        &self.current_sequence
    }

    pub fn clear_current_sequence(&mut self) {
        self.current_sequence = None;
    }

    pub fn set_current_sequence(&mut self, direction: Direction) {
        self.current_sequence = Some(direction);
    }

    pub fn workbench_zoom(&self) -> f32 {
        if self.workbench_zoom_level >= 0 {
            self.workbench_zoom_level as f32
        } else {
            -1.0 / self.workbench_zoom_level as f32
        }
    }

    pub fn zoom_in_workbench(&mut self) {
        if self.workbench_zoom_level >= 1 {
            self.workbench_zoom_level *= 2;
        } else if self.workbench_zoom_level == -2 {
            self.workbench_zoom_level = 1;
        } else {
            self.workbench_zoom_level /= 2;
        }
        self.workbench_zoom_level = std::cmp::min(self.workbench_zoom_level, 32);
    }

    pub fn zoom_out_workbench(&mut self) {
        if self.workbench_zoom_level > 1 {
            self.workbench_zoom_level /= 2;
        } else if self.workbench_zoom_level == 1 {
            self.workbench_zoom_level = -2;
        } else {
            self.workbench_zoom_level *= 2;
        }
        self.workbench_zoom_level = std::cmp::max(self.workbench_zoom_level, -4);
    }

    pub fn reset_workbench_zoom(&mut self) {
        self.workbench_zoom_level = 1;
    }

    pub fn workbench_offset(&self) -> Vector2D<f32> {
        self.workbench_offset
    }

    pub fn center_workbench(&mut self) {
        self.workbench_offset = Default::default();
    }

    pub fn zoom_in_timeline(&mut self) {
        if self.timeline_zoom_level >= 1 {
            self.timeline_zoom_level *= 2;
        } else if self.timeline_zoom_level == -2 {
            self.timeline_zoom_level = 1;
        } else {
            self.timeline_zoom_level /= 2;
        }
        self.timeline_zoom_level = std::cmp::min(self.timeline_zoom_level, 4);
    }

    pub fn zoom_out_timeline(&mut self) {
        if self.timeline_zoom_level > 1 {
            self.timeline_zoom_level /= 2;
        } else if self.timeline_zoom_level == 1 {
            self.timeline_zoom_level = -2;
        } else {
            self.timeline_zoom_level *= 2;
        }
        self.timeline_zoom_level = std::cmp::max(self.timeline_zoom_level, -4);
    }

    pub fn reset_timeline_zoom(&mut self) {
        self.timeline_zoom_level = 1;
    }

    pub fn timeline_zoom(&self) -> f32 {
        if self.timeline_zoom_level >= 0 {
            self.timeline_zoom_level as f32
        } else {
            -1.0 / self.timeline_zoom_level as f32
        }
    }

    pub fn pan(&mut self, delta: Vector2D<f32>) {
        self.workbench_offset += delta
    }

    pub fn timeline_clock(&self) -> Duration {
        self.timeline_clock
    }

    pub fn skip_to_timeline_start(&mut self) {
        self.timeline_clock = Duration::ZERO;
    }
}
