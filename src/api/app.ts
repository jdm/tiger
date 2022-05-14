import { useAppStore } from "@/stores/app";
import { AppState, ContentTab } from "@/api/dto";
import { invoke } from "@tauri-apps/api";

// TODO Replacing store.$state with command output will trigger reactive
// changes througout the whole UI.
// Consider making commands return a patch structure so that only the
// relevant store fields get modified.
// Possible solution:
// - JS https://github.com/martindale/fast-json-patch
// - Rust https://github.com/idubrov/json-patch

export async function openDocuments(paths: string[]): Promise<void> {
  const app: AppState = await invoke("open_documents", {
    paths: paths,
  });
  const appStore = useAppStore();
  appStore.$state = app;
}

export async function focusDocument(path: string): Promise<void> {
  const app: AppState = await invoke("focus_document", {
    path: path,
  });
  const appStore = useAppStore();
  appStore.$state = app;
}

export async function closeDocument(path: string): Promise<void> {
  const app: AppState = await invoke("close_document", {
    path: path,
  });
  const appStore = useAppStore();
  appStore.$state = app;
}
