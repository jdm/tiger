import {
  open as openFileDialog,
  save as saveFileDialog,
} from "@tauri-apps/api/dialog";
import {
  newDocument as doNewDocument,
  openDocuments as doOpenDocuments,
} from "@/api/app";

export async function newDocument() {
  const file = await saveFileDialog({
    filters: [{ name: "Spritesheet Files", extensions: ["tiger"] }],
  });
  if (file) {
    doNewDocument(file);
  }
}

export async function openDocuments() {
  const files = await openFileDialog({
    filters: [{ name: "Spritesheet Files", extensions: ["tiger"] }],
    multiple: true,
  });
  if (typeof files === "string") {
    doOpenDocuments([files]);
  } else if (files) {
    doOpenDocuments(files);
  }
}
