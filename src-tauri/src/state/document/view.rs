use euclid::default::*;
use std::path::Path;
use std::time::Duration;

use crate::sheet::Direction;
use crate::state::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ContentTab {
    Frames,
    Animations,
}

#[derive(Clone, Debug, PartialEq)]
pub struct View {
    pub(super) content_tab: ContentTab,
    pub(super) selection: MultiSelection,
    pub(super) frames_filter: String,
    pub(super) animations_filter: String,
    pub(super) current_animation: Option<String>,
    pub(super) current_sequence: Option<Direction>,
    pub(super) workbench_offset: Vector2D<f32>,
    pub(super) timeline_clock: Duration,
    pub(super) workbench_zoom_level: u32,
    pub(super) timeline_zoom_level: i32,
    pub(super) hide_hitboxes: bool,
    pub(super) darken_sprites: bool,
}

impl Default for View {
    fn default() -> View {
        View {
            content_tab: ContentTab::Frames,
            selection: Default::default(),
            frames_filter: Default::default(),
            animations_filter: Default::default(),
            current_animation: None,
            current_sequence: None,
            workbench_offset: Vector2D::<f32>::zero(), // Should this be an integer?
            workbench_zoom_level: 8,
            timeline_zoom_level: 1,
            timeline_clock: Default::default(),
            hide_hitboxes: false,
            darken_sprites: true,
        }
    }
}

impl View {
    pub(super) fn zoom_in_workbench(&mut self) {
        self.workbench_zoom_level = 32.min(self.workbench_zoom_level * 2);
    }

    pub(super) fn zoom_out_workbench(&mut self) {
        self.workbench_zoom_level = 1.max(self.workbench_zoom_level / 2);
    }

    pub(super) fn reset_workbench_zoom(&mut self) {
        self.workbench_zoom_level = 1;
    }

    pub(super) fn center_workbench(&mut self) {
        self.workbench_offset = Vector2D::zero();
    }

    pub(super) fn zoom_in_timeline(&mut self) {
        if self.timeline_zoom_level >= 1 {
            self.timeline_zoom_level *= 2;
        } else if self.timeline_zoom_level == -2 {
            self.timeline_zoom_level = 1;
        } else {
            self.timeline_zoom_level /= 2;
        }
        self.timeline_zoom_level = std::cmp::min(self.timeline_zoom_level, 4);
    }

    pub(super) fn zoom_out_timeline(&mut self) {
        if self.timeline_zoom_level > 1 {
            self.timeline_zoom_level /= 2;
        } else if self.timeline_zoom_level == 1 {
            self.timeline_zoom_level = -2;
        } else {
            self.timeline_zoom_level *= 2;
        }
        self.timeline_zoom_level = std::cmp::max(self.timeline_zoom_level, -4);
    }

    pub(super) fn reset_timeline_zoom(&mut self) {
        self.timeline_zoom_level = 1;
    }

    pub(super) fn pan(&mut self, delta: Vector2D<f32>) {
        self.workbench_offset += delta
    }

    pub(super) fn skip_to_timeline_start(&mut self) {
        self.timeline_clock = Duration::ZERO;
    }
}

impl Document {
    pub fn content_tab(&self) -> ContentTab {
        self.view.content_tab
    }

    pub fn frames_filter(&self) -> &String {
        &self.view.frames_filter
    }

    pub fn animations_filter(&self) -> &String {
        &self.view.animations_filter
    }

    pub fn selection(&self) -> &MultiSelection {
        &self.view.selection
    }

    pub fn current_animation(&self) -> &Option<String> {
        &self.view.current_animation
    }

    pub fn current_sequence(&self) -> &Option<Direction> {
        &self.view.current_sequence
    }

    pub fn workbench_offset(&self) -> Vector2D<f32> {
        self.view.workbench_offset
    }

    pub fn workbench_zoom(&self) -> f32 {
        self.view.workbench_zoom_level as f32
    }

    pub fn timeline_zoom(&self) -> f32 {
        if self.view.timeline_zoom_level >= 0 {
            self.view.timeline_zoom_level as f32
        } else {
            -1.0 / self.view.timeline_zoom_level as f32
        }
    }

    pub fn timeline_clock(&self) -> Duration {
        self.view.timeline_clock
    }

    pub fn should_darken_sprites(&self) -> bool {
        self.view.darken_sprites
    }

    pub fn is_hiding_hitboxes(&self) -> bool {
        self.view.hide_hitboxes
    }

    pub fn is_frame_filtered_out<T: AsRef<Path>>(&self, frame: T) -> bool {
        !self
            .view
            .frames_filter
            .split_ascii_whitespace()
            .all(|search_term| {
                frame
                    .as_ref()
                    .as_os_str()
                    .to_string_lossy()
                    .contains(search_term)
            })
    }

    pub fn is_animation_filtered_out<T: AsRef<str>>(&self, animation_name: T) -> bool {
        !self
            .view
            .animations_filter
            .split_ascii_whitespace()
            .all(|search_term| animation_name.as_ref().contains(search_term))
    }
}
