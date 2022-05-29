import {
  open as openFileDialog,
  save as saveFileDialog,
} from "@tauri-apps/api/dialog";
import {
  newDocument as doNewDocument,
  openDocuments as doOpenDocuments,
} from "@/api/app";
import { importFrames as doImportFrames } from "@/api/document";

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

export async function importFrames() {
  const files = await openFileDialog({
    filters: [{ name: "Image Files", extensions: ["png", "bmp"] }],
    multiple: true,
  });
  if (typeof files === "string") {
    doImportFrames([files]);
  } else if (files) {
    doImportFrames(files);
  }
}
