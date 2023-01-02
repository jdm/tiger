import { defineStore, acceptHMRUpdate } from "pinia";

export const useDevStore = defineStore("dev", {
  state: () => {
    return {
      debugModeEnabled: false,
    };
  },
  actions: {
    toggleDebugModeEnabled() {
      this.debugModeEnabled = !this.debugModeEnabled;
    },
  },
});

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useDevStore, import.meta.hot));
}
