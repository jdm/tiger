import { AppState, ContentTab } from "@/api/dto";
import { useAppStore } from "@/stores/app";
import { invoke } from "@tauri-apps/api";

export async function focusContentTab(contentTab: ContentTab): Promise<void> {
  const app: AppState = await invoke("focus_content_tab", {
    contentTab: contentTab,
  });
  const appStore = useAppStore();
  appStore.$state = app;
}

export async function selectFrame(
  path: string,
  shift: boolean,
  ctrl: boolean
): Promise<void> {
  const app: AppState = await invoke("select_frame", {
    path: path,
    shift: shift,
    ctrl: ctrl,
  });
  const appStore = useAppStore();
  appStore.$state = app;
}

export async function selectAnimation(
  name: string,
  shift: boolean,
  ctrl: boolean
): Promise<void> {
  const app: AppState = await invoke("select_animation", {
    name: name,
    shift: shift,
    ctrl: ctrl,
  });
  const appStore = useAppStore();
  appStore.$state = app;
}

export async function pan(delta: [number, number]): Promise<void> {
  const app: AppState = await invoke("pan", {
    delta: delta,
  });
  const appStore = useAppStore();
  appStore.$state = app;
}
