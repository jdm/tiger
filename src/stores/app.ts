import { defineStore, acceptHMRUpdate } from "pinia";

export type AppState = {
  documents: string[];
};

export const useAppStore = defineStore("app", {
  state: () =>
    ({
      documents: [],
    } as AppState),
});

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useAppStore, import.meta.hot));
}
