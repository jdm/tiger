use euclid::default::Vector2D;
use log::error;
use std::fmt::Display;
use std::{path::PathBuf, time::Duration};

use crate::document::*;
use crate::sheet::{Direction, DirectionPreset, Sheet};

#[derive(Clone, Debug)]
pub enum Command {
    Undo,
    Redo,
    DetachedNavigation,
    Paste(Clipboard),
    SetFramesListMode(ListMode),
    SetFramesListOffset(f64),
    FilterFrames(String),
    FilterAnimations(String),
    SetAnimationsListOffset(f64),
    SetHitboxesListOffset(f64),
    ImportFrames(Vec<PathBuf>),
    BeginRelocateFrames,
    RelocateFrame {
        from: PathBuf,
        to: PathBuf,
    },
    CancelRelocateFrames,
    EndRelocateFrames,
    DeleteFrame(PathBuf),
    DeleteSelectedFrames,
    DeleteSelection,
    NudgeSelection {
        direction: NudgeDirection,
        large_nudge: bool,
    },
    BrowseSelection {
        direction: BrowseDirection,
        shift: bool,
    },
    BrowseToEnd(bool),
    BrowseToStart(bool),
    ClearSelection,
    SelectAll,
    SelectFrame {
        path: PathBuf,
        shift: bool,
        ctrl: bool,
    },
    SelectAnimation {
        name: String,
        shift: bool,
        ctrl: bool,
    },
    SelectKeyframe {
        direction: Direction,
        index: usize,
        shift: bool,
        ctrl: bool,
    },
    SelectHitbox {
        name: String,
        shift: bool,
        ctrl: bool,
    },
    Pan(Vector2D<f32>),
    CenterWorkbench,
    ZoomInWorkbench,
    ZoomInWorkbenchAround(Vector2D<f32>),
    ZoomOutWorkbench,
    ZoomOutWorkbenchAround(Vector2D<f32>),
    SetWorkbenchZoomFactor(u32),
    ResetWorkbenchZoom,
    EnableSpriteDarkening,
    DisableSpriteDarkening,
    HideSprite,
    ShowSprite,
    HideHitboxes,
    ShowHitboxes,
    HideOrigin,
    ShowOrigin,
    CreateAnimation,
    EditAnimation(String),
    BeginRenameSelection,
    BeginRenameAnimation(String),
    BeginRenameHitbox(String),
    CancelRename,
    EndRenameAnimation(String),
    EndRenameHitbox(String),
    DeleteAnimation(String),
    DeleteSelectedAnimations,
    Tick(Duration),
    Play,
    Pause,
    ScrubTimeline(Duration),
    JumpToAnimationStart,
    JumpToAnimationEnd,
    JumpToPreviousFrame,
    JumpToNextFrame,
    SetSnapKeyframeDurations(bool),
    SetSnapKeyframeToOtherKeyframes(bool),
    SetSnapKeyframeToMultiplesOfDuration(bool),
    SetKeyframeSnappingBaseDuration(Duration),
    ZoomInTimeline,
    ZoomInTimelineAround(Duration),
    ZoomOutTimeline,
    ZoomOutTimelineAround(Duration),
    SetTimelineZoomAmount(f32),
    SetTimelineOffset(Duration),
    PanTimeline(f32),
    ResetTimelineZoom,
    SetAnimationLooping(bool),
    ApplyDirectionPreset(DirectionPreset),
    SelectDirection(Direction),
    BeginDragAndDropFrame(PathBuf),
    DropFrameOnTimeline {
        direction: Direction,
        index: usize,
    },
    EndDragAndDropFrame,
    DeleteSelectedKeyframes,
    SetKeyframeDuration(Duration),
    SetKeyframeOffsetX(i32),
    SetKeyframeOffsetY(i32),
    BeginDragAndDropKeyframe {
        direction: Direction,
        index: usize,
    },
    DropKeyframeOnTimeline {
        direction: Direction,
        index: usize,
    },
    EndDragAndDropKeyframe,
    BeginDragKeyframeDuration {
        direction: Direction,
        index: usize,
    },
    UpdateDragKeyframeDuration(i64),
    EndDragKeyframeDuration,
    BeginNudgeKeyframe {
        direction: Direction,
        index: usize,
    },
    UpdateNudgeKeyframe {
        displacement: Vector2D<i32>,
        both_axis: bool,
    },
    EndNudgeKeyframe,
    CreateHitbox,
    DeleteHitbox(String),
    DeleteSelectedHitboxes,
    LockHitboxes,
    UnlockHitboxes,
    SetHitboxPositionX(i32),
    SetHitboxPositionY(i32),
    SetHitboxWidth(u32),
    SetHitboxHeight(u32),
    TogglePreserveAspectRatio,
    BeginNudgeHitbox(String),
    UpdateNudgeHitbox {
        displacement: Vector2D<i32>,
        both_axis: bool,
    },
    EndNudgeHitbox,
    BeginResizeHitbox {
        name: String,
        axis: ResizeAxis,
    },
    UpdateResizeHitbox {
        displacement: Vector2D<i32>,
        preserve_aspect_ratio: bool,
    },
    EndResizeHitbox,
    BeginExportAs,
    SetExportTemplateFile(PathBuf),
    SetExportAtlasImageFile(PathBuf),
    SetExportMetadataFile(PathBuf),
    SetExportMetadataPathsRoot(PathBuf),
    CancelExportAs,
    EndExportAs,
}

#[derive(Debug, Default)]
pub(super) struct HistoryEntry {
    pub(super) last_command: Option<Command>,
    pub(super) sheet: Sheet<Absolute>,
    pub(super) view: View,
    pub(super) version: i32,
}

impl Document {
    fn process_command_internal(&mut self, command: Command) -> DocumentResult<()> {
        match command {
            Command::Undo => self.undo()?,
            Command::Redo => self.redo()?,
            Command::DetachedNavigation => (),
            Command::Paste(ref c) => self.paste(c.clone())?,
            Command::SetFramesListMode(m) => self.view.frames_list_mode = m,
            Command::FilterFrames(ref q) => self.view.frames_filter = q.clone(),
            Command::FilterAnimations(ref q) => self.view.animations_filter = q.clone(),
            Command::SetAnimationsListOffset(o) => self.view.animations_list_offset = o,
            Command::SetFramesListOffset(o) => self.view.set_frames_list_offset(o),
            Command::SetHitboxesListOffset(o) => self.view.hitboxes_list_offset = o,
            Command::ImportFrames(ref p) => self.import_frames(p),
            Command::BeginRelocateFrames => self.begin_relocate_frames(),
            Command::RelocateFrame { from, to } => self.relocate_frame(from, to)?,
            Command::CancelRelocateFrames => self.cancel_relocate_frames(),
            Command::EndRelocateFrames => self.end_relocate_frames()?,
            Command::DeleteFrame(ref p) => self.sheet.delete_frame(p),
            Command::DeleteSelectedFrames => self.delete_selected_frames(),
            Command::DeleteSelection => self.delete_selection()?,
            Command::NudgeSelection {
                direction,
                large_nudge,
            } => self.nudge_selection(direction, large_nudge)?,
            Command::BrowseSelection { direction, shift } => {
                self.browse_selection(direction, shift)?
            }
            Command::BrowseToEnd(shift) => self.browse_to_end(shift)?,
            Command::BrowseToStart(shift) => self.browse_to_start(shift)?,
            Command::ClearSelection => self.view.selection.clear(),
            Command::SelectAll => self.select_all()?,
            Command::SelectFrame { path, shift, ctrl } => self.select_frame(path, shift, ctrl),
            Command::SelectAnimation { name, shift, ctrl } => {
                self.select_animation(name, shift, ctrl)
            }
            Command::SelectKeyframe {
                direction,
                index,
                shift,
                ctrl,
            } => self.select_keyframe(direction, index, shift, ctrl)?,
            Command::SelectHitbox { name, shift, ctrl } => self.select_hitbox(name, shift, ctrl)?,
            Command::Pan(delta) => self.view.pan(delta),
            Command::CenterWorkbench => self.view.center_workbench(),
            Command::ZoomInWorkbench => self.view.zoom_in_workbench(),
            Command::ZoomInWorkbenchAround(ref p) => self.view.zoom_in_workbench_around(p),
            Command::ZoomOutWorkbench => self.view.zoom_out_workbench(),
            Command::ZoomOutWorkbenchAround(ref p) => self.view.zoom_out_workbench_around(p),
            Command::SetWorkbenchZoomFactor(f) => self.view.set_workbench_zoom_factor(f),
            Command::ResetWorkbenchZoom => self.view.reset_workbench_zoom(),
            Command::EnableSpriteDarkening => self.view.darken_sprites = true,
            Command::DisableSpriteDarkening => self.view.darken_sprites = false,
            Command::HideSprite => self.view.hide_sprite = true,
            Command::ShowSprite => self.view.hide_sprite = false,
            Command::HideHitboxes => self.view.hide_hitboxes = true,
            Command::ShowHitboxes => self.view.hide_hitboxes = false,
            Command::HideOrigin => self.view.hide_origin = true,
            Command::ShowOrigin => self.view.hide_origin = false,
            Command::CreateAnimation => self.create_animation()?,
            Command::EditAnimation(ref name) => self.edit_animation(name)?,
            Command::BeginRenameSelection => self.begin_rename_selection(),
            Command::BeginRenameAnimation(ref n) => self.begin_rename_animation(n.clone()),
            Command::BeginRenameHitbox(ref n) => self.begin_rename_hitbox(n.clone()),
            Command::CancelRename => self.cancel_rename(),
            Command::EndRenameAnimation(ref n) => self.end_rename_animation(n.clone())?,
            Command::EndRenameHitbox(ref n) => self.end_rename_hitbox(n.clone())?,
            Command::DeleteAnimation(ref name) => self.delete_animation(name),
            Command::DeleteSelectedAnimations => self.delete_selected_animations(),
            Command::Tick(dt) => self.advance_timeline(dt),
            Command::Play => self.play()?,
            Command::Pause => self.pause()?,
            Command::ScrubTimeline(t) => self.scrub_timeline(t)?,
            Command::JumpToAnimationStart => self.jump_to_animation_start()?,
            Command::JumpToAnimationEnd => self.jump_to_animation_end()?,
            Command::JumpToPreviousFrame => self.jump_to_previous_frame()?,
            Command::JumpToNextFrame => self.jump_to_next_frame()?,
            Command::SetSnapKeyframeDurations(s) => self.view.snap_keyframe_durations = s,
            Command::SetSnapKeyframeToOtherKeyframes(s) => {
                self.view.snap_keyframes_to_other_keyframes = s
            }
            Command::SetSnapKeyframeToMultiplesOfDuration(s) => {
                self.view.snap_keyframes_to_multiples_of_duration = s
            }
            Command::SetKeyframeSnappingBaseDuration(d) => {
                self.view.keyframe_snapping_base_duration =
                    d.clamp(Duration::from_millis(1), Duration::from_millis(1_000))
            }
            Command::ZoomInTimeline => self.view.zoom_in_timeline(),
            Command::ZoomInTimelineAround(t) => self.view.zoom_in_timeline_around(t),
            Command::ZoomOutTimeline => self.view.zoom_out_timeline(),
            Command::ZoomOutTimelineAround(t) => self.view.zoom_out_timeline_around(t),
            Command::SetTimelineZoomAmount(a) => self.view.set_timeline_zoom_amount(a),
            Command::ResetTimelineZoom => self.view.reset_timeline_zoom(),
            Command::SetTimelineOffset(d) => self.view.set_timeline_offset(d),
            Command::PanTimeline(d) => self.view.pan_timeline(d),
            Command::SetAnimationLooping(l) => self.set_animation_looping(l)?,
            Command::ApplyDirectionPreset(p) => self.apply_direction_preset(p)?,
            Command::SelectDirection(d) => self.select_direction(d)?,
            Command::BeginDragAndDropFrame(ref f) => self.begin_drag_and_drop_frame(f.clone()),
            Command::DropFrameOnTimeline { direction, index } => {
                self.drop_frame_on_timeline(direction, index)?
            }
            Command::EndDragAndDropFrame => self.end_drag_and_drop_frame(),
            Command::DeleteSelectedKeyframes => self.delete_selected_keyframes()?,
            Command::SetKeyframeDuration(d) => self.set_keyframe_duration(d)?,
            Command::SetKeyframeOffsetX(x) => self.set_keyframe_offset_x(x)?,
            Command::SetKeyframeOffsetY(y) => self.set_keyframe_offset_y(y)?,
            Command::BeginDragAndDropKeyframe { direction, index } => {
                self.begin_drag_and_drop_keyframe(direction, index)?
            }
            Command::DropKeyframeOnTimeline { direction, index } => {
                self.drop_keyframe_on_timeline(direction, index)?
            }
            Command::EndDragAndDropKeyframe => self.end_drag_and_drop_keyframe(),
            Command::BeginDragKeyframeDuration { direction, index } => {
                self.begin_drag_keyframe_duration(direction, index)?
            }
            Command::UpdateDragKeyframeDuration(t) => self.update_drag_keyframe_duration(t)?,
            Command::EndDragKeyframeDuration => self.end_drag_keyframe_duration(),
            Command::BeginNudgeKeyframe { direction, index } => {
                self.begin_nudge_keyframe(direction, index)?
            }
            Command::UpdateNudgeKeyframe {
                displacement,
                both_axis,
            } => self.update_nudge_keyframe(displacement, both_axis)?,
            Command::EndNudgeKeyframe => self.end_nudge_keyframe(),
            Command::CreateHitbox => self.create_hitbox()?,
            Command::DeleteHitbox(ref name) => self.delete_hitbox(name)?,
            Command::DeleteSelectedHitboxes => self.delete_selected_hitboxes()?,
            Command::LockHitboxes => self.view.lock_hitboxes = true,
            Command::UnlockHitboxes => self.view.lock_hitboxes = false,
            Command::SetHitboxPositionX(x) => self.set_hitbox_position_x(x)?,
            Command::SetHitboxPositionY(y) => self.set_hitbox_position_y(y)?,
            Command::SetHitboxWidth(width) => self.set_hitbox_width(width)?,
            Command::SetHitboxHeight(height) => self.set_hitbox_height(height)?,
            Command::TogglePreserveAspectRatio => {
                self.persistent.preserve_aspect_ratio = !self.persistent.preserve_aspect_ratio
            }
            Command::BeginNudgeHitbox(ref n) => self.begin_nudge_hitbox(n)?,
            Command::UpdateNudgeHitbox {
                displacement,
                both_axis,
            } => self.update_nudge_hitbox(displacement, both_axis)?,
            Command::EndNudgeHitbox => self.end_nudge_hitbox(),
            Command::BeginResizeHitbox { name, axis } => self.begin_resize_hitbox(name, axis)?,
            Command::UpdateResizeHitbox {
                displacement,
                preserve_aspect_ratio,
            } => self.update_resize_hitbox(displacement, preserve_aspect_ratio)?,
            Command::EndResizeHitbox => self.end_resize_hitbox(),
            Command::BeginExportAs => self.begin_export_as(),
            Command::SetExportTemplateFile(ref p) => self.set_export_template_file(p)?,
            Command::SetExportAtlasImageFile(ref p) => self.set_export_atlas_image_file(p)?,
            Command::SetExportMetadataFile(ref p) => self.set_export_metadata_file(p)?,
            Command::SetExportMetadataPathsRoot(ref p) => self.set_export_metadata_paths_root(p)?,
            Command::CancelExportAs => self.cancel_export_as(),
            Command::EndExportAs => self.end_export_as()?,
        }

        Ok(())
    }

    pub fn process_command(&mut self, command: Command) -> DocumentResult<()> {
        if !command.preserves_transient_state() {
            self.transient = Default::default();
        }

        let result = self.process_command_internal(command.clone());
        if let Err(e) = &result {
            error!("Error while processing document command `{command:?}`: {e}");
        }

        self.sanitize_view();

        if command.is_recordable() {
            self.record_command(command);
        }

        result
    }

    pub fn is_saved(&self) -> bool {
        if self.persistent.disk_version == Some(self.version()) {
            true
        } else {
            // Special case to not mark document dirty after initial navigation change.
            // This mirrors the logic in can_merge_view() which does not allow merging
            // on top of version 0.
            self.version() == 1
                && self.persistent.disk_version == Some(0)
                && self.history_index > 0
                && self.history[self.history_index - 1].sheet
                    == self.history[self.history_index].sheet
        }
    }

    pub fn mark_as_saved(&mut self, saved_version: i32) {
        self.persistent.disk_version = Some(saved_version);
    }

    pub fn version(&self) -> i32 {
        self.history[self.history_index].version
    }

    fn push_undo_state(&mut self, entry: HistoryEntry) {
        self.history.truncate(self.history_index + 1);
        self.history.push(entry);
        self.history_index = self.history.len() - 1;

        while self.history.len() > 100 {
            self.history.remove(0);
            self.history_index -= 1;
        }
    }

    fn record_command(&mut self, command: Command) {
        let current_history_entry = &self.history[self.history_index];
        let has_sheet_changes = current_history_entry.sheet != self.sheet;
        let has_view_changes = current_history_entry.view != self.view;
        let is_at_head = self.history_index == self.history.len() - 1;

        if has_sheet_changes {
            // Record view changes that were done while browsing
            // document at an older point in history
            if !is_at_head {
                match &self.detached_view {
                    Some(detached_view) if detached_view != &current_history_entry.view => {
                        if self.can_merge_view() {
                            self.history[self.history_index].view = detached_view.clone();
                        } else {
                            self.latest_version += 1;
                            self.push_undo_state(HistoryEntry {
                                sheet: current_history_entry.sheet.clone(),
                                view: detached_view.clone(),
                                last_command: Some(Command::DetachedNavigation),
                                version: self.latest_version,
                            });
                        }
                    }
                    _ => (),
                };
            }
            // Record change that was just done
            self.latest_version += 1;
            self.push_undo_state(HistoryEntry {
                sheet: self.sheet.clone(),
                view: self.view.clone(),
                last_command: Some(command),
                version: self.latest_version,
            });
        } else if has_view_changes && is_at_head {
            if self.can_merge_view() {
                self.history[self.history_index].view = self.view.clone();
            } else {
                self.latest_version += 1;
                self.push_undo_state(HistoryEntry {
                    sheet: self.sheet.clone(),
                    view: self.view.clone(),
                    last_command: Some(command),
                    version: self.latest_version,
                });
            }
        }

        if is_at_head {
            self.detached_view = None;
        } else {
            self.detached_view = Some(self.view.clone());
        }
    }

    fn can_merge_view(&self) -> bool {
        let is_at_tail = self.history[self.history_index].version == 0;
        let on_top_of_view_change = self.history_index > 0
            && self.history[self.history_index - 1].sheet == self.history[self.history_index].sheet;
        !is_at_tail && on_top_of_view_change
    }

    pub fn undo(&mut self) -> DocumentResult<()> {
        if self.history_index > 0 {
            self.history_index -= 1;
            self.sheet = self.history[self.history_index].sheet.clone();
            self.view = self.history[self.history_index].view.clone();
            self.persistent.timeline_is_playing = false;
        }
        Ok(())
    }

    pub fn redo(&mut self) -> DocumentResult<()> {
        if self.history_index < self.history.len() - 1 {
            self.history_index += 1;
            self.sheet = self.history[self.history_index].sheet.clone();
            self.view = self.history[self.history_index].view.clone();
            self.persistent.timeline_is_playing = false;
        }
        Ok(())
    }

    fn undo_command(&self) -> Option<&Command> {
        self.history[self.history_index].last_command.as_ref()
    }

    fn redo_command(&self) -> Option<&Command> {
        if self.history_index < self.history.len() - 1 {
            self.history[self.history_index + 1].last_command.as_ref()
        } else {
            None
        }
    }

    pub fn undo_effect(&self) -> Option<String> {
        self.undo_command().map(|c| c.to_string())
    }

    pub fn redo_effect(&self) -> Option<String> {
        self.redo_command().map(|c| c.to_string())
    }
}

impl Command {
    fn is_recordable(&self) -> bool {
        !matches!(
            self,
            Command::BeginDragAndDropFrame(_)
                | Command::BeginDragAndDropKeyframe { .. }
                | Command::BeginDragKeyframeDuration { .. }
                | Command::BeginNudgeHitbox(_)
                | Command::BeginNudgeKeyframe { .. }
                | Command::BeginRenameAnimation(_)
                | Command::BeginRenameHitbox(_)
                | Command::BeginRenameSelection
                | Command::BeginResizeHitbox { .. }
                | Command::CancelRename
                | Command::Redo
                | Command::Undo
                | Command::UpdateDragKeyframeDuration(_)
                | Command::UpdateNudgeHitbox { .. }
                | Command::UpdateNudgeKeyframe { .. }
                | Command::UpdateResizeHitbox { .. }
        )
    }

    fn preserves_transient_state(&self) -> bool {
        matches!(
            self,
            Command::BeginDragAndDropFrame(_)
                | Command::BeginDragAndDropKeyframe { .. }
                | Command::BeginDragKeyframeDuration { .. }
                | Command::BeginNudgeHitbox(_)
                | Command::BeginNudgeKeyframe { .. }
                | Command::BeginRenameAnimation(_)
                | Command::BeginRenameHitbox(_)
                | Command::BeginRenameSelection
                | Command::BeginResizeHitbox { .. }
                | Command::CancelRename
                | Command::EndDragAndDropFrame
                | Command::EndDragAndDropKeyframe
                | Command::EndDragKeyframeDuration
                | Command::EndNudgeHitbox
                | Command::EndNudgeKeyframe
                | Command::EndRenameAnimation(_)
                | Command::EndRenameHitbox(_)
                | Command::EndResizeHitbox
                | Command::UpdateDragKeyframeDuration(_)
                | Command::UpdateNudgeHitbox { .. }
                | Command::UpdateNudgeKeyframe { .. }
                | Command::UpdateResizeHitbox { .. }
                | Command::PanTimeline(_)
                | Command::SetAnimationsListOffset(_)
                | Command::SetHitboxesListOffset(_)
                | Command::SetFramesListOffset(_)
        )
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::DetachedNavigation
            | Command::SetFramesListMode(_)
            | Command::FilterFrames(_)
            | Command::FilterAnimations(_)
            | Command::BrowseSelection { .. }
            | Command::BrowseToEnd(_)
            | Command::BrowseToStart(_)
            | Command::ClearSelection
            | Command::SelectAll
            | Command::SelectFrame { .. }
            | Command::SelectAnimation { .. }
            | Command::SelectKeyframe { .. }
            | Command::SelectHitbox { .. }
            | Command::Pan(_)
            | Command::CenterWorkbench
            | Command::ZoomInWorkbench
            | Command::ZoomInWorkbenchAround(_)
            | Command::ZoomOutWorkbench
            | Command::ZoomOutWorkbenchAround(_)
            | Command::SetWorkbenchZoomFactor(_)
            | Command::ResetWorkbenchZoom
            | Command::EnableSpriteDarkening
            | Command::DisableSpriteDarkening
            | Command::HideSprite
            | Command::ShowSprite
            | Command::HideHitboxes
            | Command::ShowHitboxes
            | Command::HideOrigin
            | Command::ShowOrigin
            | Command::ScrubTimeline(_)
            | Command::JumpToAnimationStart
            | Command::JumpToAnimationEnd
            | Command::JumpToPreviousFrame
            | Command::JumpToNextFrame
            | Command::ZoomInTimeline
            | Command::ZoomInTimelineAround(_)
            | Command::ZoomOutTimeline
            | Command::ZoomOutTimelineAround(_)
            | Command::SetAnimationsListOffset(_)
            | Command::SetFramesListOffset(_)
            | Command::SetHitboxesListOffset(_)
            | Command::SetTimelineZoomAmount(_)
            | Command::SetTimelineOffset(_)
            | Command::PanTimeline(_)
            | Command::Play
            | Command::Pause
            | Command::ResetTimelineZoom => f.write_str("Navigation"),

            Command::BeginExportAs
            | Command::SetExportTemplateFile(_)
            | Command::SetExportAtlasImageFile(_)
            | Command::SetExportMetadataFile(_)
            | Command::SetExportMetadataPathsRoot(_)
            | Command::CancelExportAs
            | Command::EndExportAs => f.write_str("Change Export Settings"),

            Command::Undo => f.write_str("Undo"),
            Command::Redo => f.write_str("Redo"),
            Command::Paste(c) => match c {
                Clipboard::Animations(_) => f.write_str("Paste Animations"),
                Clipboard::Keyframes(_) => f.write_str("Paste Keyframes"),
                Clipboard::Hitboxes(_) => f.write_str("Paste Hitboxes"),
            },
            Command::ImportFrames(_) => f.write_str("Import Frames"),
            Command::DeleteFrame(_) => f.write_str("Delete Frame"),
            Command::DeleteSelectedFrames => f.write_str("Delete Frames"),
            Command::DeleteSelection => f.write_str("Delete"),
            Command::NudgeSelection { .. } => f.write_str("Nudge"),
            Command::CreateAnimation => f.write_str("Create Animation"),
            Command::EditAnimation(_) => f.write_str("Open Animation"),
            Command::DeleteAnimation(_) => f.write_str("Delete Animation"),
            Command::DeleteSelectedAnimations => f.write_str("Delete Animations"),
            Command::Tick(_) => f.write_str("Tick"),
            Command::SetAnimationLooping(_) => f.write_str("Toggle Looping"),
            Command::ApplyDirectionPreset(_) => f.write_str("Set Perspective"),
            Command::SelectDirection(_) => f.write_str("Select Directions"),
            Command::DeleteSelectedKeyframes => f.write_str("Delete Keyframes"),
            Command::SetKeyframeDuration(_) => f.write_str("Set Keyframe Duration"),
            Command::SetKeyframeOffsetX(_) => f.write_str("Start Keyframe X Offset"),
            Command::SetKeyframeOffsetY(_) => f.write_str("Start Keyframe Y Offset"),
            Command::CreateHitbox => f.write_str("Create Hitbox"),
            Command::DeleteHitbox(_) => f.write_str("Delete Hitbox"),
            Command::DeleteSelectedHitboxes => f.write_str("Delete Hitboxes"),
            Command::LockHitboxes => f.write_str("Lock Hitboxes"),
            Command::UnlockHitboxes => f.write_str("Unlock Hitboxes"),
            Command::SetHitboxPositionX(_) => f.write_str("Set Hitbox X Position"),
            Command::SetHitboxPositionY(_) => f.write_str("Set Hitbox Y Position"),
            Command::SetHitboxWidth(_) => f.write_str("Set Hitbox Width"),
            Command::SetHitboxHeight(_) => f.write_str("Set Hitbox Height"),
            Command::TogglePreserveAspectRatio => f.write_str("Toggle Preserve Aspect Ratio"),
            Command::SetSnapKeyframeDurations(true) => f.write_str("Enable Keyframe Snapping"),
            Command::SetSnapKeyframeDurations(false) => f.write_str("Disable Keyframe Snapping"),

            Command::BeginRelocateFrames
            | Command::RelocateFrame { .. }
            | Command::CancelRelocateFrames
            | Command::EndRelocateFrames => f.write_str("Relocate Frames"),

            Command::SetSnapKeyframeToOtherKeyframes(_)
            | Command::SetSnapKeyframeToMultiplesOfDuration(_)
            | Command::SetKeyframeSnappingBaseDuration(_) => {
                f.write_str("Adjust Snapping Settings")
            }

            Command::BeginDragAndDropFrame(_)
            | Command::DropFrameOnTimeline { .. }
            | Command::EndDragAndDropFrame => f.write_str("Create Keyframe"),

            Command::BeginDragAndDropKeyframe { .. }
            | Command::DropKeyframeOnTimeline { .. }
            | Command::EndDragAndDropKeyframe => f.write_str("Reorder Keyframes"),

            Command::BeginDragKeyframeDuration { .. }
            | Command::UpdateDragKeyframeDuration(_)
            | Command::EndDragKeyframeDuration => f.write_str("Adjust Keyframe Duration"),

            Command::BeginNudgeKeyframe { .. }
            | Command::UpdateNudgeKeyframe { .. }
            | Command::EndNudgeKeyframe => f.write_str("Nudge Keyframe"),

            Command::BeginNudgeHitbox(_)
            | Command::UpdateNudgeHitbox { .. }
            | Command::EndNudgeHitbox => f.write_str("Nudge Hitbox"),

            Command::BeginResizeHitbox { .. }
            | Command::UpdateResizeHitbox { .. }
            | Command::EndResizeHitbox => f.write_str("Resize Hitbox"),

            Command::BeginRenameSelection
            | Command::BeginRenameAnimation(_)
            | Command::BeginRenameHitbox(_)
            | Command::CancelRename => f.write_str("Rename"),

            Command::EndRenameAnimation(_) => f.write_str("Rename Animation"),
            Command::EndRenameHitbox(_) => f.write_str("Rename Hitbox"),
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::app::mock::TigerAppMock;

    fn list_frames(app: &TigerAppMock) -> Vec<String> {
        app.document()
            .sheet
            .frames
            .iter()
            .map(|f| f.path.to_string_lossy().into_owned())
            .collect::<Vec<_>>()
    }

    #[test]
    fn can_undo_and_redo() {
        let app = TigerAppMock::new();
        app.new_document("tmp");
        app.import_frames(vec!["frame_1"]);
        app.import_frames(vec!["frame_2"]);
        app.import_frames(vec!["frame_3"]);

        let one = vec![String::from("frame_1")];
        let one_and_two = vec![String::from("frame_1"), String::from("frame_2")];
        let all_three = vec![
            String::from("frame_1"),
            String::from("frame_2"),
            String::from("frame_3"),
        ];

        assert_eq!(list_frames(&app), all_three);
        app.undo();
        assert_eq!(list_frames(&app), one_and_two);
        app.undo();
        assert_eq!(list_frames(&app), one);
        app.redo();
        assert_eq!(list_frames(&app), one_and_two);
        app.redo();
        assert_eq!(list_frames(&app), all_three);
    }

    #[tokio::test]
    async fn view_changes_do_not_dirty_document() {
        let app = TigerAppMock::new();
        app.open_documents(vec!["test-data/samurai.tiger"]).await;
        assert!(!app.document().has_unsaved_changes);
        app.select_animation("dead", false, false);
        assert!(!app.document().has_unsaved_changes);
        app.select_animation("walk", false, false);
        assert!(!app.document().has_unsaved_changes);
    }

    #[tokio::test]
    async fn sheet_changes_do_dirty_document() {
        let app = TigerAppMock::new();
        app.open_documents(vec!["test-data/samurai.tiger"]).await;
        assert!(!app.document().has_unsaved_changes);
        app.import_frames(vec!["some-frame.png"]);
        assert!(app.document().has_unsaved_changes);
    }

    #[test]
    fn can_undo_multiple_view_changes_at_once() {
        let app = TigerAppMock::new();
        app.new_document("tmp");
        app.set_workbench_zoom_factor(1);
        app.import_frames(vec!["frame_1"]);
        app.set_workbench_zoom_factor(2);
        app.set_workbench_zoom_factor(4);
        app.set_workbench_zoom_factor(8);
        assert_eq!(app.document().workbench_zoom, 8.0);
        app.undo();
        assert_eq!(app.document().workbench_zoom, 1.0);
        app.redo();
        assert_eq!(app.document().workbench_zoom, 8.0);
    }

    #[test]
    fn truncates_undo_stack_when_editing_sheet() {
        let app = TigerAppMock::new();
        app.new_document("tmp");
        app.import_frames(vec!["frame_1"]);
        app.import_frames(vec!["frame_2"]);
        app.import_frames(vec!["frame_3"]);
        app.undo();
        app.undo();
        app.import_frames(vec!["frame_4"]);
        app.redo();
        assert_eq!(
            list_frames(&app),
            vec![String::from("frame_1"), String::from("frame_4")]
        );
    }

    #[test]
    fn editing_view_while_browsing_history_does_not_truncate_history() {
        let app = TigerAppMock::new();
        app.new_document("tmp");
        app.import_frames(vec!["frame_1"]);
        app.import_frames(vec!["frame_2"]);
        app.import_frames(vec!["frame_3"]);
        app.undo();
        app.undo();
        app.set_workbench_zoom_factor(2);
        app.set_workbench_zoom_factor(4);
        app.redo();
        app.redo();
        assert_eq!(
            list_frames(&app),
            vec![
                String::from("frame_1"),
                String::from("frame_2"),
                String::from("frame_3")
            ]
        );
    }

    #[test]
    fn editing_sheet_while_browsing_can_insert_navigation_entry() {
        let app = TigerAppMock::new();
        app.new_document("tmp");
        app.set_workbench_zoom_factor(1);
        app.import_frames(vec!["frame_1"]);
        app.import_frames(vec!["frame_2"]);
        app.import_frames(vec!["frame_3"]);
        app.undo();
        app.undo();
        app.set_workbench_zoom_factor(2);
        app.import_frames(vec!["frame_4"]);
        app.undo();
        assert_eq!(list_frames(&app), vec![String::from("frame_1"),]);
        assert_eq!(app.document().workbench_zoom, 2.0);
        app.undo();
        assert_eq!(list_frames(&app), vec![String::from("frame_1"),]);
        assert_eq!(app.document().workbench_zoom, 1.0);
    }

    #[test]
    fn editing_sheet_while_browsing_can_amend_navigation_entry() {
        let app = TigerAppMock::new();
        app.new_document("tmp");
        app.set_workbench_zoom_factor(1);
        app.import_frames(vec!["frame_1"]);
        app.set_workbench_zoom_factor(8);
        app.import_frames(vec!["frame_2"]);
        app.import_frames(vec!["frame_3"]);
        app.undo();
        app.undo();
        app.set_workbench_zoom_factor(2);
        app.import_frames(vec!["frame_4"]);
        app.undo();
        assert_eq!(list_frames(&app), vec![String::from("frame_1"),]);
        assert_eq!(app.document().workbench_zoom, 2.0);
        app.undo();
        assert_eq!(list_frames(&app), vec![String::from("frame_1"),]);
        assert_eq!(app.document().workbench_zoom, 1.0);
    }
}
