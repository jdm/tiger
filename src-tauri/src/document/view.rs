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
    pub(super) animations_filter: String,
    pub(super) animations_list_offset: f64,
    pub(super) current_animation: Option<String>,
    pub(super) current_sequence: Option<Direction>,
    pub(super) darken_sprites: bool,
    pub(super) frames_filter: String,
    pub(super) frames_grid_offset: f64,
    pub(super) frames_list_mode: ListMode,
    pub(super) frames_list_offset: f64,
    pub(super) hide_hitboxes: bool,
    pub(super) hide_origin: bool,
    pub(super) hide_sprite: bool,
    pub(super) hitboxes_list_offset: f64,
    pub(super) keyframe_snapping_base_duration: Duration,
    pub(super) lock_hitboxes: bool,
    pub(super) selection: SelectionState,
    pub(super) snap_keyframe_durations: bool,
    pub(super) snap_keyframes_to_multiples_of_duration: bool,
    pub(super) snap_keyframes_to_other_keyframes: bool,
    pub(super) timeline_clock: Duration,
    pub(super) timeline_offset: Duration,
    pub(super) timeline_zoom_amount: f32,
    pub(super) workbench_offset: Vector2D<f32>,
    pub(super) workbench_zoom_factor: u32,
}

impl Default for View {
    fn default() -> View {
        View {
            animations_filter: Default::default(),
            animations_list_offset: 0.0,
            current_animation: None,
            current_sequence: None,
            darken_sprites: true,
            frames_filter: Default::default(),
            frames_grid_offset: 0.0,
            frames_list_mode: ListMode::Grid4xN,
            frames_list_offset: 0.0,
            hide_hitboxes: false,
            hide_origin: false,
            hide_sprite: false,
            hitboxes_list_offset: 0.0,
            keyframe_snapping_base_duration: Duration::from_millis(100),
            lock_hitboxes: false,
            selection: Default::default(),
            snap_keyframe_durations: true,
            snap_keyframes_to_multiples_of_duration: false,
            snap_keyframes_to_other_keyframes: true,
            timeline_clock: Default::default(),
            timeline_offset: Default::default(),
            timeline_zoom_amount: 0.5,
            workbench_offset: Vector2D::<f32>::zero(),
            workbench_zoom_factor: 8,
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
        self.adjust_timeline_zoom_amount(0.2, self.timeline_clock);
    }

    pub(super) fn zoom_out_timeline(&mut self) {
        self.adjust_timeline_zoom_amount(-0.2, self.timeline_clock);
    }

    pub(super) fn zoom_in_timeline_around(&mut self, fixed_point: Duration) {
        self.adjust_timeline_zoom_amount(0.2, fixed_point);
    }

    pub(super) fn zoom_out_timeline_around(&mut self, fixed_point: Duration) {
        self.adjust_timeline_zoom_amount(-0.2, fixed_point);
    }

    fn adjust_timeline_zoom_amount(&mut self, zoom_delta: f32, fixed_point: Duration) {
        let old_zoom = self.timeline_zoom_factor();
        self.set_timeline_zoom_amount(self.timeline_zoom_amount + zoom_delta);
        let new_zoom = self.timeline_zoom_factor();

        // Nudge timeline offset to keep fixed_point visually in the same place
        let clock = fixed_point.as_secs_f32() * 1_000.0;
        let old_offset = self.timeline_offset.as_secs_f32() * 1_000.0;
        let new_offset =
            ((clock * (new_zoom - old_zoom) + old_offset * old_zoom) / new_zoom).max(0.0);
        self.timeline_offset = Duration::from_secs_f32(new_offset / 1_000.0);
    }

    pub(super) fn set_timeline_zoom_amount(&mut self, amount: f32) {
        self.timeline_zoom_amount = amount.clamp(0.0, 1.0);
    }

    pub(super) fn reset_timeline_zoom(&mut self) {
        self.timeline_zoom_amount = 0.5;
    }

    pub(super) fn timeline_zoom_factor(&self) -> f32 {
        const MIN_TIMELINE_ZOOM: f32 = 0.5;
        const MAX_TIMELINE_ZOOM: f32 = 3.0;
        let min_log = MIN_TIMELINE_ZOOM.log2();
        let max_log = MAX_TIMELINE_ZOOM.log2();
        let scale = max_log - min_log;
        let factor = (min_log + scale * self.timeline_zoom_amount).exp2();
        (factor * 10.0).round() / 10.0
    }

    pub(super) fn set_timeline_offset(&mut self, offset: Duration) {
        self.timeline_offset = offset;
    }

    pub(super) fn pan_timeline(&mut self, delta: f32) {
        let delta_seconds = -delta / self.timeline_zoom_factor() / 1_000.0;
        if delta_seconds >= 0.0 {
            self.timeline_offset = self
                .timeline_offset
                .saturating_add(Duration::from_secs_f32(delta_seconds.abs()));
        } else {
            self.timeline_offset = self
                .timeline_offset
                .saturating_sub(Duration::from_secs_f32(delta_seconds.abs()));
        }
    }

    pub(super) fn reset_timeline_offset(&mut self) {
        self.timeline_offset = Duration::ZERO;
    }

    pub(super) fn pan(&mut self, delta: Vector2D<f32>) {
        self.workbench_offset += delta / self.workbench_zoom_factor as f32;
    }

    pub(super) fn skip_to_timeline_start(&mut self) {
        self.timeline_clock = Duration::ZERO;
    }

    pub(super) fn set_frames_list_offset(&mut self, offset: f64) {
        match self.frames_list_mode {
            ListMode::Linear => self.frames_list_offset = offset,
            ListMode::Grid4xN => self.frames_grid_offset = offset,
        }
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

    pub fn frames_list_offset(&self) -> f64 {
        match self.view.frames_list_mode {
            ListMode::Linear => self.view.frames_list_offset,
            ListMode::Grid4xN => self.view.frames_grid_offset,
        }
    }

    pub fn animations_list_offset(&self) -> f64 {
        self.view.animations_list_offset
    }

    pub fn hitboxes_list_offset(&self) -> f64 {
        self.view.hitboxes_list_offset
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

    pub fn timeline_zoom_amount(&self) -> f32 {
        self.view.timeline_zoom_amount
    }

    pub fn timeline_zoom_factor(&self) -> f32 {
        self.view.timeline_zoom_factor()
    }

    pub fn timeline_offset(&self) -> Duration {
        self.view.timeline_offset
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
        let filter = self.view.frames_filter.to_lowercase();
        !filter.split_ascii_whitespace().all(|search_term| {
            frame
                .as_ref()
                .as_os_str()
                .to_string_lossy()
                .to_lowercase()
                .contains(search_term)
        })
    }

    pub fn is_animation_filtered_out<T: AsRef<str>>(&self, animation_name: T) -> bool {
        let filter = self.view.animations_filter.to_lowercase();
        !filter
            .split_ascii_whitespace()
            .all(|search_term| animation_name.as_ref().to_lowercase().contains(search_term))
    }
}

#[cfg(test)]
mod tests {

    use crate::{app::mock::TigerAppMock, dto};

    #[tokio::test]
    async fn can_zoom_workbench_in_out() {
        let app = TigerAppMock::new();
        app.open_documents(vec!["test-data/samurai.tiger"]).await;
        app.set_workbench_zoom_factor(16);
        assert_eq!(app.document().workbench_zoom, 16.0);
        app.reset_workbench_zoom();
        assert_eq!(app.document().workbench_zoom, 1.0);
        app.zoom_in_workbench();
        assert_eq!(app.document().workbench_zoom, 2.0);
        app.zoom_out_workbench();
        assert_eq!(app.document().workbench_zoom, 1.0);
    }

    #[tokio::test]
    async fn can_zoom_workbench_around_fixed_point() {
        let app = TigerAppMock::new();
        app.open_documents(vec!["test-data/samurai.tiger"]).await;
        app.reset_workbench_zoom();
        app.zoom_in_workbench_around((10.0, 20.0));
        assert_eq!(app.document().workbench_offset, (-5.0, -10.0));
        app.zoom_out_workbench_around((10.0, 20.0));
        assert_eq!(app.document().workbench_offset, (0.0, 0.0));
    }

    #[tokio::test]
    async fn can_pan_and_center_workbench() {
        let app = TigerAppMock::new();
        app.open_documents(vec!["test-data/samurai.tiger"]).await;
        app.set_workbench_zoom_factor(2);
        app.pan((16.0, 32.0));
        assert_eq!(app.document().workbench_offset, (8.0, 16.0));
        app.center_workbench();
        assert_eq!(app.document().workbench_offset, (0.0, 0.0));
    }

    #[tokio::test]
    async fn can_toggle_workbench_view_options() {
        let app = TigerAppMock::new();
        app.open_documents(vec!["test-data/samurai.tiger"]).await;

        app.disable_sprite_darkening();
        assert!(!app.document().darken_sprites);
        app.enable_sprite_darkening();
        assert!(app.document().darken_sprites);

        app.hide_hitboxes();
        assert!(app.document().hide_hitboxes);
        app.show_hitboxes();
        assert!(!app.document().hide_hitboxes);

        app.hide_origin();
        assert!(app.document().hide_origin);
        app.show_origin();
        assert!(!app.document().hide_origin);

        app.hide_sprite();
        assert!(app.document().hide_sprite);
        app.show_sprite();
        assert!(!app.document().hide_sprite);
    }

    #[tokio::test]
    async fn can_zoom_timeline_in_out() {
        let app = TigerAppMock::new();
        app.open_documents(vec!["test-data/samurai.tiger"]).await;
        app.set_timeline_zoom_amount(0.1);
        assert_eq!(app.document().timeline_zoom_amount, 0.1);
        app.reset_timeline_zoom();
        assert_eq!(app.document().timeline_zoom_amount, 0.5);
        app.zoom_in_timeline();
        assert_eq!(app.document().timeline_zoom_amount, 0.7);
        app.zoom_out_timeline();
        assert_eq!(app.document().timeline_zoom_amount, 0.5);
    }

    #[tokio::test]
    async fn can_zoom_timeline_around_fixed_point() {
        let app = TigerAppMock::new();
        app.open_documents(vec!["test-data/samurai.tiger"]).await;
        app.reset_timeline_zoom();

        let fixed_point = 1_000.0;
        let visible_position = || {
            (fixed_point - app.document().timeline_offset_millis)
                * app.document().timeline_zoom_factor
        };
        let reference = visible_position();
        app.zoom_in_timeline_around(1_000.0);
        assert!((visible_position() - reference).abs() < 1.0);
        app.zoom_out_timeline_around(1_000.0);
        assert!((visible_position() - reference).abs() < 1.0);
    }

    #[tokio::test]
    async fn can_pan_timeline() {
        let app = TigerAppMock::new();
        app.open_documents(vec!["test-data/samurai.tiger"]).await;
        app.edit_animation("walk");
        app.set_timeline_zoom_amount(1.0);
        app.pan_timeline(-900.0);
        assert_eq!(app.document().timeline_offset_millis, 300.0);
    }

    #[tokio::test]
    async fn can_scrub_timeline() {
        let app = TigerAppMock::new();
        app.open_documents(vec!["test-data/samurai.tiger"]).await;
        app.edit_animation("walk");
        app.set_timeline_offset(1500.0);
        assert_eq!(app.document().timeline_offset_millis, 1500.0);
    }

    #[tokio::test]
    async fn can_filter_frames() {
        let app = TigerAppMock::new();
        app.open_documents(vec!["test-data/samurai.tiger"]).await;
        assert!(!app.document().frame("dead-all").filtered_out);
        assert!(!app.document().frame("attack-west").filtered_out);
        app.filter_frames("a ack");
        assert!(app.document().frame("dead-all").filtered_out);
        assert!(!app.document().frame("attack-west").filtered_out);
    }

    #[tokio::test]
    async fn can_filter_animations() {
        let app = TigerAppMock::new();
        app.open_documents(vec!["test-data/samurai.tiger"]).await;
        assert!(!app.document().animation("idle").filtered_out);
        assert!(!app.document().animation("attack").filtered_out);
        app.filter_animations("dl");
        assert!(!app.document().animation("idle").filtered_out);
        assert!(app.document().animation("attack").filtered_out);
    }

    #[test]
    fn can_store_list_offsets() {
        let app = TigerAppMock::new();
        app.new_document("tmp");

        app.set_frames_list_mode(dto::ListMode::Linear);
        app.set_frames_list_offset(50.0);
        assert_eq!(app.document().frames_list_offset, 50.0);

        app.set_frames_list_mode(dto::ListMode::Grid4xN);
        assert_eq!(app.document().frames_list_offset, 0.0);
        app.set_frames_list_offset(60.0);
        assert_eq!(app.document().frames_list_offset, 60.0);

        app.set_frames_list_mode(dto::ListMode::Linear);
        assert_eq!(app.document().frames_list_offset, 50.0);

        app.set_animations_list_offset(80.0);
        assert_eq!(app.document().animations_list_offset, 80.0);

        app.set_hitboxes_list_offset(90.0);
        assert_eq!(app.document().hitboxes_list_offset, 90.0);
    }

    #[test]
    fn can_lock_and_unlock_hitboxes() {
        let app = TigerAppMock::new();
        app.new_document("tmp");
        assert!(!app.document().lock_hitboxes);
        app.lock_hitboxes();
        assert!(app.document().lock_hitboxes);
        app.unlock_hitboxes();
        assert!(!app.document().lock_hitboxes);
    }
}
