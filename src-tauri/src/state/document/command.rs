use euclid::default::Vector2D;
use std::{path::PathBuf, time::Duration};

use crate::sheet::{Direction, DirectionPreset, Sheet};
use crate::state::*;

#[derive(Clone, Debug)]
pub enum Command {
    Undo,
    Redo,
    FocusContentTab(ContentTab),
    FilterFrames(String),
    FilterAnimations(String),
    ImportFrames(Vec<PathBuf>),
    DeleteSelectedFrames,
    ClearSelection,
    AlterSelection(SelectionInput, bool, bool),
    Pan(Vector2D<f32>),
    CenterWorkbench,
    ZoomInWorkbench,
    ZoomOutWorkbench,
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
    RenameAnimation(String, String),
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
    ZoomInTimeline,
    ZoomOutTimeline,
    SetTimelineZoomAmount(f32),
    ResetTimelineZoom,
    SetAnimationLooping(bool),
    ApplyDirectionPreset(DirectionPreset),
    SelectDirection(Direction),
    BeginDragAndDropFrame(PathBuf),
    DropFrameOnTimeline(Direction, usize),
    EndDragAndDropFrame,
    DeleteSelectedKeyframes,
    BeginDragAndDropKeyframe(Direction, usize),
    DropKeyframeOnTimeline(Direction, usize),
    EndDragAndDropKeyframe,
    BeginDragKeyframeDuration(Direction, usize),
    UpdateDragKeyframeDuration(i64),
    EndDragKeyframeDuration(),
    BeginNudgeKeyframe(Direction, usize),
    UpdateNudgeKeyframe(Vector2D<i32>, bool),
    EndNudgeKeyframe(),
    CreateHitbox(Option<Vector2D<i32>>),
    RenameHitbox(String, String),
    DeleteHitbox(String),
    DeleteSelectedHitboxes,
    LockHitboxes,
    UnlockHitboxes,
    SetHitboxPositionX(i32),
    SetHitboxPositionY(i32),
    SetHitboxWidth(u32),
    SetHitboxHeight(u32),
    BeginNudgeHitbox(String),
    UpdateNudgeHitbox(Vector2D<i32>, bool),
    EndNudgeHitbox,
    BeginResizeHitbox(String, ResizeAxis),
    UpdateResizeHitbox(Vector2D<i32>, bool),
    EndResizeHitbox,
    BeginExportAs,
    SetExportTemplateFile(PathBuf),
    SetExportTextureFile(PathBuf),
    SetExportMetadataFile(PathBuf),
    SetExportMetadataPathsRoot(PathBuf),
    CancelExportAs,
    EndExportAs,
}

#[derive(Debug, Default)]
pub(super) struct HistoryEntry {
    pub(super) last_command: Option<Command>,
    pub(super) sheet: Sheet,
    pub(super) view: View,
    pub(super) version: i32,
}

impl Document {
    pub fn process_command(&mut self, command: Command) -> Result<(), DocumentError> {
        match command {
            Command::Undo => self.undo()?,
            Command::Redo => self.redo()?,
            Command::FocusContentTab(t) => self.view.content_tab = t,
            Command::FilterFrames(ref q) => self.view.frames_filter = q.clone(),
            Command::FilterAnimations(ref q) => self.view.animations_filter = q.clone(),
            Command::ImportFrames(ref p) => self.sheet.add_frames(p),
            Command::DeleteSelectedFrames => self.delete_selected_frames(),
            Command::ClearSelection => self.view.selection.clear(),
            Command::AlterSelection(ref selection, shift, ctrl) => {
                self.alter_selection(selection, shift, ctrl)?
            }
            Command::Pan(delta) => self.view.pan(delta),
            Command::CenterWorkbench => self.view.center_workbench(),
            Command::ZoomInWorkbench => self.view.zoom_in_workbench(),
            Command::ZoomOutWorkbench => self.view.zoom_out_workbench(),
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
            Command::RenameAnimation(ref old_name, ref new_name) => {
                self.rename_animation(old_name, new_name)?
            }
            Command::DeleteAnimation(ref name) => self.delete_animation(name),
            Command::DeleteSelectedAnimations => self.delete_selected_animations(),
            Command::Tick(dt) => self.tick(dt),
            Command::Play => self.play()?,
            Command::Pause => self.pause()?,
            Command::ScrubTimeline(t) => self.scrub_timeline(t)?,
            Command::JumpToAnimationStart => self.jump_to_animation_start()?,
            Command::JumpToAnimationEnd => self.jump_to_animation_end()?,
            Command::JumpToPreviousFrame => self.jump_to_previous_frame()?,
            Command::JumpToNextFrame => self.jump_to_next_frame()?,
            Command::ZoomInTimeline => self.view.zoom_in_timeline(),
            Command::ZoomOutTimeline => self.view.zoom_out_timeline(),
            Command::SetTimelineZoomAmount(a) => self.view.set_timeline_zoom_amount(a),
            Command::ResetTimelineZoom => self.view.reset_timeline_zoom(),
            Command::SetAnimationLooping(l) => self.set_animation_looping(l)?,
            Command::ApplyDirectionPreset(p) => self.apply_direction_preset(p)?,
            Command::SelectDirection(d) => self.select_direction(d)?,
            Command::BeginDragAndDropFrame(ref f) => self.begin_drag_and_drop_frame(f.clone()),
            Command::DropFrameOnTimeline(d, i) => self.drop_frame_on_timeline(d, i)?,
            Command::EndDragAndDropFrame => (),
            Command::DeleteSelectedKeyframes => self.delete_selected_keyframes()?,
            Command::BeginDragAndDropKeyframe(d, i) => self.begin_drag_and_drop_keyframe(d, i)?,
            Command::DropKeyframeOnTimeline(d, i) => self.drop_keyframe_on_timeline(d, i)?,
            Command::EndDragAndDropKeyframe => (),
            Command::BeginDragKeyframeDuration(d, i) => self.begin_drag_keyframe_duration(d, i)?,
            Command::UpdateDragKeyframeDuration(t) => self.update_drag_keyframe_duration(t)?,
            Command::EndDragKeyframeDuration() => (),
            Command::BeginNudgeKeyframe(d, i) => self.begin_nudge_keyframe(d, i)?,
            Command::UpdateNudgeKeyframe(d, b) => self.update_nudge_keyframe(d, b)?,
            Command::EndNudgeKeyframe() => (),
            Command::CreateHitbox(p) => self.create_hitbox(p)?,
            Command::RenameHitbox(ref old_name, ref new_name) => {
                self.rename_hitbox(old_name, new_name)?
            }
            Command::DeleteHitbox(ref name) => self.delete_hitbox(name)?,
            Command::DeleteSelectedHitboxes => self.delete_selected_hitboxes()?,
            Command::LockHitboxes => self.view.lock_hitboxes = true,
            Command::UnlockHitboxes => self.view.lock_hitboxes = false,
            Command::SetHitboxPositionX(x) => self.set_hitbox_position_x(x)?,
            Command::SetHitboxPositionY(y) => self.set_hitbox_position_y(y)?,
            Command::SetHitboxWidth(width) => self.set_hitbox_width(width)?,
            Command::SetHitboxHeight(height) => self.set_hitbox_height(height)?,
            Command::BeginNudgeHitbox(ref n) => self.begin_nudge_hitbox(n)?,
            Command::UpdateNudgeHitbox(d, b) => self.update_nudge_hitbox(d, b)?,
            Command::EndNudgeHitbox => (),
            Command::BeginResizeHitbox(ref n, a) => self.begin_resize_hitbox(n, a)?,
            Command::UpdateResizeHitbox(d, a) => self.update_resize_hitbox(d, a)?,
            Command::EndResizeHitbox => (),
            Command::BeginExportAs => self.begin_export_as(),
            Command::SetExportTemplateFile(ref p) => self.set_export_template_file(p)?,
            Command::SetExportTextureFile(ref p) => self.set_export_texture_file(p)?,
            Command::SetExportMetadataFile(ref p) => self.set_export_metadata_file(p)?,
            Command::SetExportMetadataPathsRoot(ref p) => self.set_export_metadata_paths_root(p)?,
            Command::CancelExportAs => self.cancel_export_as(),
            Command::EndExportAs => self.end_export_as()?,
        }

        self.sanitize_view();

        if !matches!(
            command,
            Command::Tick(_)
                | Command::BeginDragAndDropFrame(_)
                | Command::BeginDragAndDropKeyframe(_, _)
                | Command::BeginDragKeyframeDuration(_, _)
                | Command::UpdateDragKeyframeDuration(_)
                | Command::BeginNudgeKeyframe(_, _)
                | Command::UpdateNudgeKeyframe(_, _)
                | Command::BeginNudgeHitbox(_)
                | Command::UpdateNudgeHitbox(_, _)
                | Command::BeginResizeHitbox(_, _)
                | Command::UpdateResizeHitbox(_, _)
        ) {
            self.transient = Default::default();
            self.record_command(command);
        }

        Ok(())
    }

    pub fn is_saved(&self) -> bool {
        self.persistent.disk_version == self.version()
    }

    pub fn mark_as_saved(&mut self, saved_version: i32) {
        self.persistent.disk_version = saved_version;
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
        let has_sheet_changes = self.history[self.history_index].sheet != self.sheet;

        if has_sheet_changes {
            self.next_version += 1;
        }

        let new_undo_state = HistoryEntry {
            sheet: self.sheet.clone(),
            view: self.view.clone(),
            last_command: Some(command),
            version: self.next_version,
        };

        if has_sheet_changes {
            self.push_undo_state(new_undo_state);
        } else if self.history[self.history_index].view != new_undo_state.view {
            let merge = self.history_index > 0
                && self.history[self.history_index - 1].sheet
                    == self.history[self.history_index].sheet;
            if merge {
                self.history[self.history_index].view = new_undo_state.view;
            } else {
                self.push_undo_state(new_undo_state);
            }
        }
    }

    pub fn undo(&mut self) -> Result<(), DocumentError> {
        if self.history_index > 0 {
            self.history_index -= 1;
            self.sheet = self.history[self.history_index].sheet.clone();
            self.view = self.history[self.history_index].view.clone();
            self.persistent.timeline_is_playing = false;
        }
        Ok(())
    }

    pub fn redo(&mut self) -> Result<(), DocumentError> {
        if self.history_index < self.history.len() - 1 {
            self.history_index += 1;
            self.sheet = self.history[self.history_index].sheet.clone();
            self.view = self.history[self.history_index].view.clone();
            self.persistent.timeline_is_playing = false;
        }
        Ok(())
    }

    pub fn get_undo_command(&self) -> Option<&Command> {
        self.history[self.history_index].last_command.as_ref()
    }

    pub fn get_redo_command(&self) -> Option<&Command> {
        if self.history_index < self.history.len() - 1 {
            self.history[self.history_index + 1].last_command.as_ref()
        } else {
            None
        }
    }
}
