import {
  beginExportAs,
  beginRenameSelection,
  browseSelection,
  browseToEnd,
  browseToStart,
  centerWorkbench,
  closeAllDocuments,
  closeCurrentDocument,
  copy,
  cut,
  deleteSelection,
  doExport,
  newDocument,
  nudgeSelection,
  openDocuments,
  paste,
  pause,
  play,
  redo,
  resetTimelineZoom,
  resetWorkbenchZoom,
  save,
  saveAll,
  saveAs,
  selectAll,
  undo,
  zoomInTimeline,
  zoomInWorkbench,
  zoomOutTimeline,
  zoomOutWorkbench,
} from "@/backend/api";
import { BrowseDirection, NudgeDirection } from "@/backend/dto";
import { useFocusStore } from "@/stores/focus";
import { useStateStore } from "@/stores/state";

function onKeyDown(event: KeyboardEvent) {
  const focus = useFocusStore();
  const state = useStateStore();

  const isInputtingText = document.activeElement?.tagName == "INPUT";

  // Prevent browser shortcuts
  if (
    (event.key == "E" && event.ctrlKey) || // Search
    (event.key == "S" && event.ctrlKey) || // Screenshot
    (event.key == "z" && event.ctrlKey) || //  Undo
    (event.key == "Z" && event.ctrlKey) || //  Redo
    //  Selection
    (!isInputtingText && event.key == "ArrowUp") ||
    (!isInputtingText && event.key == "ArrowDown") ||
    (!isInputtingText && event.key == "ArrowLeft") ||
    (!isInputtingText && event.key == "ArrowRight")
  ) {
    event.preventDefault();
  }

  if (isInputtingText || focus.isInputTrapped) {
    return;
  }

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
      saveAs(state.currentDocumentPath);
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
      if (state.currentDocument?.timelineIsPlaying) {
        pause();
      } else {
        play();
      }
    } else if (event.key == "Delete") {
      deleteSelection();
    } else if (event.key == "ArrowUp") {
      browseSelection(BrowseDirection.Up, event.shiftKey);
    } else if (event.key == "ArrowDown") {
      browseSelection(BrowseDirection.Down, event.shiftKey);
    } else if (event.key == "ArrowLeft") {
      browseSelection(BrowseDirection.Left, event.shiftKey);
    } else if (event.key == "ArrowRight") {
      browseSelection(BrowseDirection.Right, event.shiftKey);
    } else if (event.key == "Home") {
      browseToStart(event.shiftKey);
    } else if (event.key == "End") {
      browseToEnd(event.shiftKey);
    } else if (event.key == "F2") {
      beginRenameSelection();
    }
  }
}

export function registerKeyboardShortcuts() {
  window.addEventListener("keydown", onKeyDown);
}

export function unregisterKeyboardShortcuts() {
  window.removeEventListener("keydown", onKeyDown);
}
