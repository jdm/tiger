import { ContentTab } from "@/api/dto";
import { useAppStore } from "@/stores/app";
import { invoke } from "@tauri-apps/api";

export async function focusContentTab(contentTab: ContentTab): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("focus_content_tab", { contentTab: contentTab }));
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

export async function pan(delta: [number, number]): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("pan", { delta: delta }));
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

export async function play(): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("play"));
}

export async function pause(): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("pause"));
}
