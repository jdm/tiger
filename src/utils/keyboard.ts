import { newDocument, openDocuments, saveAs } from "@/api/local";
import {
  browseSelection,
  beginExportAs,
  centerWorkbench,
  deleteSelection,
  doExport,
  nudgeSelection,
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
import { BrowseSelectionDirection, NudgeDirection } from "@/api/dto";

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
    } else if (event.key == "ArrowUp") {
      nudgeSelection(NudgeDirection.Up, event.shiftKey);
    } else if (event.key == "ArrowDown") {
      nudgeSelection(NudgeDirection.Down, event.shiftKey);
    } else if (event.key == "ArrowLeft") {
      nudgeSelection(NudgeDirection.Left, event.shiftKey);
    } else if (event.key == "ArrowRight") {
      nudgeSelection(NudgeDirection.Right, event.shiftKey);
    }
  } else {
    if (event.key == " ") {
      if (app.currentDocument?.timelineIsPlaying) {
        pause();
      } else {
        play();
      }
    } else if (event.key == "Delete") {
      deleteSelection();
    } else if (event.key == "ArrowUp") {
      browseSelection(BrowseSelectionDirection.Up, event.shiftKey);
    } else if (event.key == "ArrowDown") {
      browseSelection(BrowseSelectionDirection.Down, event.shiftKey);
    } else if (event.key == "ArrowLeft") {
      browseSelection(BrowseSelectionDirection.Left, event.shiftKey);
    } else if (event.key == "ArrowRight") {
      browseSelection(BrowseSelectionDirection.Right, event.shiftKey);
    }
  }
}

export function registerKeyboardShortcuts() {
  window.addEventListener("keydown", onKeyDown);
}

export function unregisterKeyboardShortcuts() {
  window.removeEventListener("keydown", onKeyDown);
}
