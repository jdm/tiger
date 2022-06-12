import { ContentTab, Direction, DirectionPreset, ResizeAxis } from "@/api/dto";
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

export async function undo(): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("undo"));
}

export async function redo(): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("redo"));
}

export async function focusContentTab(contentTab: ContentTab): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("focus_content_tab", { contentTab: contentTab }));
}

export async function importFrames(paths: string[]): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("import_frames", { paths: paths }));
}

export async function deleteSelectedFrames(): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("delete_selected_frames"));
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

export async function createAnimation(): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("create_animation"));
}

export async function editAnimation(name: string): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("edit_animation", { name: name }));
}

export async function renameAnimation(
  oldName: string,
  newName: string
): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(
    await invoke("rename_animation", {
      oldName: oldName,
      newName: newName,
    })
  );
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

export async function zoomInTimeline(): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("zoom_in_timeline"));
}

export async function zoomOutTimeline(): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("zoom_out_timeline"));
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

export async function hideHitboxes(): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("hide_hitboxes"));
}

export async function showHitboxes(): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("show_hitboxes"));
}

export async function renameHitbox(
  oldName: string,
  newName: string
): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(
    await invoke("rename_hitbox", {
      oldName: oldName,
      newName: newName,
    })
  );
}

export async function deleteHitbox(name: string): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("delete_hitbox", { name: name }));
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
