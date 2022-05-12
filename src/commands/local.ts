import { open as openFileDialog } from "@tauri-apps/api/dialog";
import { openDocuments } from "@/commands/app";

export async function openFiles() {
  const files = await openFileDialog({
    filters: [{ name: "Spritesheet Files", extensions: ["tiger"] }],
    multiple: true,
  });
  if (typeof files === "string") {
    openDocuments([files]);
  } else if (files) {
    openDocuments(files);
  }
}
