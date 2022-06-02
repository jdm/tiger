import { ContentTab, Direction, DirectionPreset } from "@/api/dto";
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

export async function beginDragAndDropFrames(frames: string[]): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(
    await invoke("begin_drag_and_drop_frames", { frames: frames })
  );
}

export async function dropFramesOnTimeline(
  direction: Direction,
  index: number
): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(
    await invoke("drop_frames_on_timeline", {
      direction: direction,
      index: index,
    })
  );
}
