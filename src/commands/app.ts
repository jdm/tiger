import { useAppStore, AppState } from "@/stores/app";
import { invoke } from "@tauri-apps/api/tauri";

// TODO Replacing store.$state with command output will trigger reactive
// changes througout the whole UI.
// Consider making commands return a patch structure so that only the
// relevant store fields get modified.
// Possible solution:
// - JS https://github.com/martindale/fast-json-patch
// - Rust https://github.com/idubrov/json-patch

export async function openDocuments(paths: String[]): Promise<void> {
  try {
    const app: AppState = await invoke("open_documents", {
      paths: paths,
    });
    const appStore = useAppStore();
    appStore.$state = app;
  } catch (error) {
    // TODO error UI
    console.log(error);
  }
}
