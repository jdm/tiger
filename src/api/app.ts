import { useAppStore } from "@/stores/app";
import { invoke } from "@tauri-apps/api";

export async function newDocument(path: string): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(
    await invoke("new_document", {
      path: path,
    })
  );
}

export async function openDocuments(paths: string[]): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(
    await invoke("open_documents", {
      paths: paths,
    })
  );
}

export async function focusDocument(path: string): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(
    await invoke("focus_document", {
      path: path,
    })
  );
}

export async function closeDocument(path: string): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(
    await invoke("close_document", {
      path: path,
    })
  );
}
