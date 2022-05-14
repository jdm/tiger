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
