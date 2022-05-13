import { defineStore, acceptHMRUpdate } from "pinia";

export type AppState = {
  documents: Document[];
  currentDocumentPath: string | null;
};

type Document = {
  path: string;
  name: string;
  sheet: Sheet;
};

type Sheet = {
  frames: string[];
};

export const useAppStore = defineStore("app", {
  state: () =>
    ({
      documents: [],
      currentDocumentPath: null,
    } as AppState),
  getters: {
    currentDocument(state) {
      for (let document of state.documents) {
        if (document.path == state.currentDocumentPath) {
          return document;
        }
      }
      return null;
    },
  },
});

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useAppStore, import.meta.hot));
}
