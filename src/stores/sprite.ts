import { convertFileSrc } from "@tauri-apps/api/tauri";
import { defineStore, acceptHMRUpdate } from "pinia";

export const useSpriteStore = defineStore("sprite", {
  state: () => {
    return {
      invalidationCounts: new Map<string, number>(),
    };
  },
  actions: {
    invalidate(path: string) {
      const oldValue = this.invalidationCounts.get(path) || 0;
      this.invalidationCounts.set(path, 1 + oldValue);
    },
  },
  getters: {
    getURL: (state) => {
      return (path: string) =>
        convertFileSrc(path) +
        `?invalidation=${state.invalidationCounts.get(path) || 0}`;
    },
  },
});

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useSpriteStore, import.meta.hot));
}
