import { defineStore, acceptHMRUpdate } from "pinia";

export type AppState = {
  documents: string[];
  currentDocument: Document | null;
};

type Document = {
  source: string;
};

export const useAppStore = defineStore("app", {
  state: () =>
    ({
      documents: [],
      currentDocument: null,
    } as AppState),
});

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useAppStore, import.meta.hot));
}
