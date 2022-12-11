use euclid::default::*;
use std::path::Path;
use std::time::Duration;

use crate::document::*;
use crate::sheet::Direction;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ListMode {
    Linear,
    Grid4xN,
}

#[derive(Clone, Debug, PartialEq)]
pub struct View {
    pub(super) frames_list_mode: ListMode,
    pub(super) selection: SelectionState,
    pub(super) frames_filter: String,
    pub(super) animations_filter: String,
    pub(super) current_animation: Option<String>,
    pub(super) current_sequence: Option<Direction>,
    pub(super) workbench_offset: Vector2D<f32>,
    pub(super) timeline_clock: Duration,
    pub(super) workbench_zoom_factor: u32,
    pub(super) timeline_zoom_amount: f32,
    pub(super) darken_sprites: bool,
    pub(super) hide_sprite: bool,
    pub(super) hide_hitboxes: bool,
    pub(super) hide_origin: bool,
    pub(super) lock_hitboxes: bool,
    pub(super) snap_keyframe_durations: bool,
    pub(super) snap_keyframes_to_other_keyframes: bool,
    pub(super) snap_keyframes_to_multiples_of_duration: bool,
    pub(super) keyframe_snapping_base_duration: Duration,
}

impl Default for View {
    fn default() -> View {
        View {
            frames_list_mode: ListMode::Grid4xN,
            selection: Default::default(),
            frames_filter: Default::default(),
            animations_filter: Default::default(),
            current_animation: None,
            current_sequence: None,
            workbench_offset: Vector2D::<f32>::zero(), // Should this be an integer?
            workbench_zoom_factor: 8,
            timeline_zoom_amount: 0.5,
            timeline_clock: Default::default(),
            darken_sprites: true,
            hide_sprite: false,
            hide_hitboxes: false,
            hide_origin: false,
            lock_hitboxes: false,
            snap_keyframe_durations: true,
            snap_keyframes_to_other_keyframes: true,
            snap_keyframes_to_multiples_of_duration: false,
            keyframe_snapping_base_duration: Duration::from_millis(100),
        }
    }
}

impl View {
    pub(super) fn zoom_in_workbench(&mut self) {
        self.set_workbench_zoom_factor(self.workbench_zoom_factor * 2);
    }

    pub(super) fn zoom_out_workbench(&mut self) {
        self.set_workbench_zoom_factor(self.workbench_zoom_factor / 2);
    }

    pub(super) fn zoom_in_workbench_around(&mut self, fixed_point: &Vector2D<f32>) {
        let old_zoom_factor = self.workbench_zoom_factor as f32;
        self.zoom_in_workbench();
        let new_zoom_factor = self.workbench_zoom_factor as f32;
        self.workbench_offset += ((*fixed_point + self.workbench_offset)
            * (old_zoom_factor - new_zoom_factor))
            / new_zoom_factor;
    }

    pub(super) fn zoom_out_workbench_around(&mut self, fixed_point: &Vector2D<f32>) {
        let old_zoom_factor = self.workbench_zoom_factor as f32;
        self.zoom_out_workbench();
        let new_zoom_factor = self.workbench_zoom_factor as f32;
        self.workbench_offset += (*fixed_point + self.workbench_offset)
            * (old_zoom_factor - new_zoom_factor)
            / new_zoom_factor;
    }

    pub(super) fn set_workbench_zoom_factor(&mut self, zoom_factor: u32) {
        const MIN_WORKBENCH_ZOOM: u32 = 1;
        const MAX_WORKBENCH_ZOOM: u32 = 32;
        self.workbench_zoom_factor = zoom_factor.clamp(MIN_WORKBENCH_ZOOM, MAX_WORKBENCH_ZOOM);
    }

    pub(super) fn reset_workbench_zoom(&mut self) {
        self.workbench_zoom_factor = 1;
    }

    pub(super) fn center_workbench(&mut self) {
        self.workbench_offset = Vector2D::zero();
    }

    pub(super) fn zoom_in_timeline(&mut self) {
        self.timeline_zoom_amount = (self.timeline_zoom_amount + 0.2).min(1.0);
    }

    pub(super) fn zoom_out_timeline(&mut self) {
        self.timeline_zoom_amount = (self.timeline_zoom_amount - 0.2).max(0.0);
    }

    pub(super) fn set_timeline_zoom_amount(&mut self, amount: f32) {
        self.timeline_zoom_amount = amount.clamp(0.0, 1.0);
    }

    pub(super) fn reset_timeline_zoom(&mut self) {
        self.timeline_zoom_amount = 0.5;
    }

    pub(super) fn pan(&mut self, delta: Vector2D<f32>) {
        self.workbench_offset += delta / self.workbench_zoom_factor as f32;
    }

    pub(super) fn skip_to_timeline_start(&mut self) {
        self.timeline_clock = Duration::ZERO;
    }
}

impl Document {
    pub fn frames_list_mode(&self) -> ListMode {
        self.view.frames_list_mode
    }

    pub fn frames_filter(&self) -> &String {
        &self.view.frames_filter
    }

    pub fn animations_filter(&self) -> &String {
        &self.view.animations_filter
    }

    pub fn selection(&self) -> &SelectionState {
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
        self.view.workbench_zoom_factor as f32
    }

    pub fn timeline_zoom_factor(&self) -> f32 {
        const MIN_TIMELINE_ZOOM: f32 = 0.5;
        const MAX_TIMELINE_ZOOM: f32 = 3.0;
        let min_log = MIN_TIMELINE_ZOOM.log2();
        let max_log = MAX_TIMELINE_ZOOM.log2();
        let scale = max_log - min_log;
        (min_log + scale * self.view.timeline_zoom_amount).exp2()
    }

    pub fn timeline_zoom_amount(&self) -> f32 {
        self.view.timeline_zoom_amount
    }

    pub fn timeline_clock(&self) -> Duration {
        self.view.timeline_clock
    }

    pub fn should_snap_keyframe_durations(&self) -> bool {
        self.view.snap_keyframe_durations
    }

    pub fn should_snap_keyframes_to_other_keyframes(&self) -> bool {
        self.view.snap_keyframes_to_other_keyframes
    }

    pub fn should_snap_keyframes_to_multiples_of_duration(&self) -> bool {
        self.view.snap_keyframes_to_multiples_of_duration
    }

    pub fn keyframe_snapping_base_duration(&self) -> Duration {
        self.view.keyframe_snapping_base_duration
    }

    pub fn should_darken_sprites(&self) -> bool {
        self.view.darken_sprites
    }

    pub fn is_hiding_sprite(&self) -> bool {
        self.view.hide_sprite
    }

    pub fn is_hiding_hitboxes(&self) -> bool {
        self.view.hide_hitboxes
    }

    pub fn is_hiding_origin(&self) -> bool {
        self.view.hide_origin
    }

    pub fn are_hitboxes_locked(&self) -> bool {
        self.view.lock_hitboxes
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
