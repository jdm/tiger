import { newDocument, openDocuments, saveAs } from "@/api/local";
import {
  beginExportAs,
  beginRenameSelection,
  browseSelection,
  browseToEnd,
  browseToStart,
  cancelExportAs,
  centerWorkbench,
  copy,
  cut,
  deleteSelection,
  doExport,
  nudgeSelection,
  paste,
  pause,
  play,
  redo,
  resetTimelineZoom,
  resetWorkbenchZoom,
  save,
  selectAll,
  undo,
  zoomInTimeline,
  zoomInWorkbench,
  zoomOutTimeline,
  zoomOutWorkbench,
} from "@/api/document";
import {
  acknowledgeError,
  cancelExit,
  closeAllDocuments,
  closeCurrentDocument,
  saveAll,
} from "@/api/app";
import { useAppStore } from "@/stores/app";
import { BrowseDirection, NudgeDirection } from "@/api/dto";

function onKeyDown(event: KeyboardEvent) {
  const isActiveElementKeyboardFriendly =
    (document.activeElement as HTMLInputElement).tabIndex != -1;

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
      event.preventDefault();
      beginExportAs();
    } else if (event.key == "w") {
      closeCurrentDocument();
    } else if (event.key == "W") {
      closeAllDocuments();
    } else if (event.key == "z") {
      event.preventDefault();
      undo();
    } else if (event.key == "Z") {
      event.preventDefault();
      redo();
    } else if (event.key == "x") {
      cut();
    } else if (event.key == "c") {
      copy();
    } else if (event.key == "v") {
      paste();
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
    } else if (event.key == "a") {
      selectAll();
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
      if (!isActiveElementKeyboardFriendly) {
        event.preventDefault();
        if (app.currentDocument?.timelineIsPlaying) {
          pause();
        } else {
          play();
        }
      }
    } else if (event.key == "Delete") {
      deleteSelection();
    } else if (event.key == "ArrowUp") {
      event.preventDefault();
      browseSelection(BrowseDirection.Up, event.shiftKey);
    } else if (event.key == "ArrowDown") {
      event.preventDefault();
      browseSelection(BrowseDirection.Down, event.shiftKey);
    } else if (event.key == "ArrowLeft") {
      event.preventDefault();
      browseSelection(BrowseDirection.Left, event.shiftKey);
    } else if (event.key == "ArrowRight") {
      event.preventDefault();
      browseSelection(BrowseDirection.Right, event.shiftKey);
    } else if (event.key == "Home") {
      browseToStart(event.shiftKey);
    } else if (event.key == "End") {
      browseToEnd(event.shiftKey);
    } else if (event.key == "F2") {
      beginRenameSelection();
    } else if (event.key == "Enter") {
      if (!isActiveElementKeyboardFriendly) {
        event.preventDefault();
      }
    } else if (event.key == "Escape") {
      if (app.error) {
        acknowledgeError();
      } else if (app.currentDocument?.wasCloseRequested) {
        cancelExit();
      } else if (app.currentDocument?.exportSettingsBeingEdited) {
        cancelExportAs();
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
