import { useAppStore, AppState } from "@/stores/app";
import { invoke } from "@tauri-apps/api/tauri";

export async function openDocument(path: String): Promise<void> {
  const app: AppState = await invoke("open_document", {
    path: path,
  });
  const appStore = useAppStore();
  appStore.$state = app;
  // TODO this also also needs to update the document store
}
