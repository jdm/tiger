import { useAppStore } from "@/stores/app";
import { invoke } from "@tauri-apps/api";

export async function getState(): Promise<void> {
  const appStore = useAppStore();
  appStore.$state = await invoke("get_state");
}

export async function showErrorMessage(
  title: string,
  summary: string,
  details: string
): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(
    await invoke("show_error_message", {
      title: title,
      summary: summary,
      details: details,
    })
  );
}

export async function acknowledgeError(): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("acknowledge_error"));
}

export async function newDocument(path: string): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("new_document", { path: path }));
}

export async function openDocuments(paths: string[]): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("open_documents", { paths: paths }));
}

export async function saveAll(): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("save_all"));
}

export async function focusDocument(path: string): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("focus_document", { path: path }));
}

export async function closeDocument(path: string): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("close_document", { path: path }));
}

export async function closeCurrentDocument(): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("close_current_document"));
}

export async function closeAllDocuments(): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("close_all_documents"));
}

export async function requestExit(): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("request_exit"));
}

export async function cancelExit(): Promise<void> {
  const appStore = useAppStore();
  appStore.patch(await invoke("cancel_exit"));
}
