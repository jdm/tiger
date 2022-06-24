import { newDocument, openDocuments, saveAs } from "@/api/local";
import {
  beginExportAs,
  centerWorkbench,
  doExport,
  pause,
  play,
  redo,
  resetTimelineZoom,
  resetWorkbenchZoom,
  save,
  undo,
  zoomInTimeline,
  zoomInWorkbench,
  zoomOutTimeline,
  zoomOutWorkbench,
} from "@/api/document";
import { closeAllDocuments, closeCurrentDocument, saveAll } from "@/api/app";
import { useAppStore } from "@/stores/app";

function onKeyDown(event: KeyboardEvent) {
  console.log(document.activeElement?.tagName);
  if (document.activeElement?.tagName == "INPUT") {
    return;
  }

  const app = useAppStore();

  if (event.ctrlKey) {
    if (event.key == "n") {
      newDocument();
    } else if (event.key == "o") {
      openDocuments();
    } else if (event.key == "s") {
      if (event.altKey) {
        saveAll();
      } else {
        save();
      }
    } else if (event.key == "S") {
      saveAs();
    } else if (event.key == "e") {
      doExport();
    } else if (event.key == "E") {
      beginExportAs();
    } else if (event.key == "w") {
      closeCurrentDocument();
    } else if (event.key == "W") {
      closeAllDocuments();
    } else if (event.key == "z") {
      undo();
    } else if (event.key == "Z") {
      redo();
    } else if (event.key == " ") {
      centerWorkbench();
    } else if (event.key == "+" || event.key == "=") {
      if (event.altKey) {
        zoomInTimeline();
      } else {
        zoomInWorkbench();
      }
    } else if (event.key == "-") {
      if (event.altKey) {
        zoomOutTimeline();
      } else {
        zoomOutWorkbench();
      }
    } else if (event.key == "0") {
      if (event.altKey) {
        resetTimelineZoom();
      } else {
        resetWorkbenchZoom();
      }
    }
  } else {
    if (event.key == " ") {
      if (app.currentDocument?.timelineIsPlaying) {
        pause();
      } else {
        play();
      }
    }
  }
}

export function registerKeyboardShortcuts() {
  window.addEventListener("keydown", onKeyDown);
}

export function unregisterKeyboardShortcuts() {
  window.removeEventListener("keydown", onKeyDown);
}
