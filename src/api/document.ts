import {
  BrowseDirection,
  Direction,
  DirectionPreset,
  ListMode,
  NudgeDirection,
  ResizeAxis,
} from "@/api/dto";
import { useAppStore } from "@/stores/app";
import { invoke } from "@tauri-apps/api";

export async function closeWithoutSaving(): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("close_without_saving"));
}

export async function save(): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("save"));
}

export async function saveAs(newPath: string): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("save_as", { newPath: newPath }));
}

export async function undo(): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("undo"));
}

export async function redo(): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("redo"));
}

export async function cut(): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("cut"));
}

export async function copy(): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("copy"));
}

export async function paste(): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("paste"));
}

export async function setFramesListMode(listMode: ListMode): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("set_frames_list_mode", { listMode: listMode }));
}

export async function filterFrames(searchQuery: string): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("filter_frames", { searchQuery: searchQuery }));
}

export async function filterAnimations(searchQuery: string): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(
    await invoke("filter_animations", { searchQuery: searchQuery })
  );
}

export async function importFrames(paths: string[]): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("import_frames", { paths: paths }));
}

export async function deleteFrame(path: string): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("delete_frame", { path: path }));
}

export async function deleteSelectedFrames(): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("delete_selected_frames"));
}

export async function deleteSelection(): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("delete_selection"));
}

export async function beginRenameSelection(): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("begin_rename_selection"));
}

export async function beginRenameAnimation(
  animationName: String
): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(
    await invoke("begin_rename_animation", { animationName: animationName })
  );
}

export async function beginRenameHitbox(hitboxName: String): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(
    await invoke("begin_rename_hitbox", { hitboxName: hitboxName })
  );
}

export async function endRenameAnimation(newName: String): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("end_rename_animation", { newName: newName }));
}

export async function endRenameHitbox(newName: String): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("end_rename_hitbox", { newName: newName }));
}

export async function cancelRename(): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("cancel_rename"));
}

export async function nudgeSelection(
  direction: NudgeDirection,
  largeNudge: boolean
): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(
    await invoke("nudge_selection", {
      direction: direction,
      largeNudge: largeNudge,
    })
  );
}

export async function browseSelection(
  direction: BrowseDirection,
  shift: boolean
): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(
    await invoke("browse_selection", {
      direction: direction,
      shift: shift,
    })
  );
}

export async function clearSelection(): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("clear_selection"));
}

export async function selectFrame(
  path: string,
  shift: boolean,
  ctrl: boolean
): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(
    await invoke("select_frame", {
      path: path,
      shift: shift,
      ctrl: ctrl,
    })
  );
}

export async function selectAnimation(
  name: string,
  shift: boolean,
  ctrl: boolean
): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(
    await invoke("select_animation", {
      name: name,
      shift: shift,
      ctrl: ctrl,
    })
  );
}

export async function selectKeyframe(
  direction: Direction,
  index: number,
  shift: boolean,
  ctrl: boolean
): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(
    await invoke("select_keyframe", {
      direction: direction,
      index: index,
      shift: shift,
      ctrl: ctrl,
    })
  );
}

export async function selectHitbox(
  name: string,
  shift: boolean,
  ctrl: boolean
): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(
    await invoke("select_hitbox", {
      name: name,
      shift: shift,
      ctrl: ctrl,
    })
  );
}

export async function pan(delta: [number, number]): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("pan", { delta: delta }));
}

export async function centerWorkbench(): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("center_workbench"));
}

export async function zoomInWorkbench(): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("zoom_in_workbench"));
}

export async function zoomOutWorkbench(): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("zoom_out_workbench"));
}

export async function setWorkbenchZoomFactor(
  zoomFactor: number
): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(
    await invoke("set_workbench_zoom_factor", { zoomFactor: zoomFactor })
  );
}

export async function resetWorkbenchZoom(): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("reset_workbench_zoom"));
}

export async function enableSpriteDarkening(): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("enable_sprite_darkening"));
}

export async function disableSpriteDarkening(): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("disable_sprite_darkening"));
}

export async function hideSprite(): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("hide_sprite"));
}

export async function showSprite(): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("show_sprite"));
}

export async function hideHitboxes(): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("hide_hitboxes"));
}

export async function showHitboxes(): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("show_hitboxes"));
}

export async function hideOrigin(): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("hide_origin"));
}

export async function showOrigin(): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("show_origin"));
}

export async function createAnimation(): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("create_animation"));
}

export async function editAnimation(name: string): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("edit_animation", { name: name }));
}

export async function deleteAnimation(name: string): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("delete_animation", { name: name }));
}

export async function deleteSelectedAnimations(): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("delete_selected_animations"));
}

export async function tick(deltaTimeMillis: number): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("tick", { deltaTimeMillis: deltaTimeMillis }));
}

export async function play(): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("play"));
}

export async function pause(): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("pause"));
}

export async function scrubTimeline(timeMillis: number): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("scrub_timeline", { timeMillis: timeMillis }));
}

export async function jumpToAnimationStart(): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("jump_to_animation_start"));
}

export async function jumpToAnimationEnd(): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("jump_to_animation_end"));
}

export async function jumpToPreviousFrame(): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("jump_to_previous_frame"));
}

export async function jumpToNextFrame(): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("jump_to_next_frame"));
}

export async function setSnapKeyframeDurations(snap: boolean): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("set_snap_keyframe_durations", { snap: snap }));
}

export async function setSnapKeyframesToOtherKeyframes(
  snap: boolean
): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(
    await invoke("set_snap_keyframes_to_other_keyframes", { snap: snap })
  );
}

export async function setSnapKeyframesToMultiplesOfDuration(
  snap: boolean
): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(
    await invoke("set_snap_keyframes_to_multiples_of_duration", { snap: snap })
  );
}

export async function setKeyframeSnappingBaseDuration(
  durationMillis: number
): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(
    await invoke("set_keyframe_snapping_base_duration", {
      durationMillis: durationMillis,
    })
  );
}

export async function zoomInTimeline(): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("zoom_in_timeline"));
}

export async function zoomOutTimeline(): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("zoom_out_timeline"));
}

export async function setTimelineZoomAmount(amount: number): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("set_timeline_zoom_amount", { amount: amount }));
}

export async function resetTimelineZoom(): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("reset_timeline_zoom"));
}

export async function setAnimationLooping(isLooping: boolean): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(
    await invoke("set_animation_looping", { isLooping: isLooping })
  );
}

export async function applyDirectionPreset(
  preset: DirectionPreset
): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("apply_direction_preset", { preset: preset }));
}

export async function selectDirection(direction: Direction): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("select_direction", { direction: direction }));
}

export async function beginDragAndDropFrame(frame: string): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("begin_drag_and_drop_frame", { frame: frame }));
}

export async function dropFrameOnTimeline(
  direction: Direction,
  index: number
): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(
    await invoke("drop_frame_on_timeline", {
      direction: direction,
      index: index,
    })
  );
}

export async function endDragAndDropFrame(): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("end_drag_and_drop_frame"));
}

export async function deleteSelectedKeyframes(): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("delete_selected_keyframes"));
}

export async function setKeyframeDuration(
  durationMillis: number
): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(
    await invoke("set_keyframe_duration", { durationMillis: durationMillis })
  );
}

export async function setKeyframeOffsetX(x: number): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("set_keyframe_offset_x", { x: x }));
}

export async function setKeyframeOffsetY(y: number): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("set_keyframe_offset_y", { y: y }));
}

export async function beginDragAndDropKeyframe(
  direction: Direction,
  index: number
): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(
    await invoke("begin_drag_and_drop_keyframe", {
      direction: direction,
      index: index,
    })
  );
}

export async function dropKeyframeOnTimeline(
  direction: Direction,
  index: number
): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(
    await invoke("drop_keyframe_on_timeline", {
      direction: direction,
      index: index,
    })
  );
}

export async function endDragAndDropKeyframe(): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("end_drag_and_drop_keyframe"));
}

export async function beginDragKeyframeDuration(
  direction: Direction,
  index: number
): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(
    await invoke("begin_drag_keyframe_duration", {
      direction: direction,
      index: index,
    })
  );
}

export async function updateDragKeyframeDuration(
  deltaMillis: number
): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(
    await invoke("update_drag_keyframe_duration", { deltaMillis: deltaMillis })
  );
}

export async function endDragKeyframeDuration(): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("end_drag_keyframe_duration"));
}

export async function beginNudgeKeyframe(
  direction: Direction,
  index: number
): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(
    await invoke("begin_nudge_keyframe", {
      direction: direction,
      index: index,
    })
  );
}

export async function updateNudgeKeyframe(
  displacement: [number, number],
  bothAxis: boolean
): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(
    await invoke("update_nudge_keyframe", {
      displacement: displacement,
      bothAxis: bothAxis,
    })
  );
}

export async function endNudgeKeyframe(): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("end_nudge_keyframe"));
}

export async function createHitbox(
  position: [number, number] | null
): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("create_hitbox", { position: position }));
}

export async function deleteHitbox(name: string): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("delete_hitbox", { name: name }));
}

export async function deleteSelectedHitboxes(): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("delete_selected_hitboxes"));
}

export async function lockHitboxes(): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("lock_hitboxes"));
}

export async function unlockHitboxes(): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("unlock_hitboxes"));
}

export async function setHitboxPositionX(x: number): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("set_hitbox_position_x", { x: x }));
}

export async function setHitboxPositionY(y: number): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("set_hitbox_position_y", { y: y }));
}

export async function setHitboxWidth(width: number): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("set_hitbox_width", { width: width }));
}

export async function setHitboxHeight(height: number): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("set_hitbox_height", { height: height }));
}

export async function togglePreserveAspectRatio(): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("toggle_preserve_aspect_ratio"));
}

export async function beginNudgeHitbox(name: string): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("begin_nudge_hitbox", { name: name }));
}

export async function updateNudgeHitbox(
  displacement: [number, number],
  bothAxis: boolean
): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(
    await invoke("update_nudge_hitbox", {
      displacement: displacement,
      bothAxis: bothAxis,
    })
  );
}

export async function endNudgeHitbox(): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("end_nudge_hitbox"));
}

export async function beginResizeHitbox(
  name: string,
  axis: ResizeAxis
): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(
    await invoke("begin_resize_hitbox", { name: name, axis: axis })
  );
}

export async function updateResizeHitbox(
  displacement: [number, number],
  preserveAspectRatio: boolean
): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(
    await invoke("update_resize_hitbox", {
      displacement: displacement,
      preserveAspectRatio: preserveAspectRatio,
    })
  );
}

export async function endResizeHitbox(): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("end_resize_hitbox"));
}

export async function doExport(): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("export"));
}

export async function beginExportAs(): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("begin_export_as"));
}

export async function setExportTemplateFile(file: string): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("set_export_template_file", { file: file }));
}

export async function setExportTextureFile(file: string): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("set_export_texture_file", { file: file }));
}

export async function setExportMetadataFile(file: string): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("set_export_metadata_file", { file: file }));
}

export async function setExportMetadataPathsRoot(
  directory: string
): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(
    await invoke("set_export_metadata_paths_root", { directory: directory })
  );
}

export async function cancelExportAs(): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("cancel_export_as"));
}

export async function endExportAs(): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("end_export_as"));
}
