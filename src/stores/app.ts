import { defineStore, acceptHMRUpdate } from "pinia";

export const useApp = defineStore("app", {
  state: () => ({
    documents: ["a", "b", "c"] as string[],
  }),
});

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useApp, import.meta.hot));
}
