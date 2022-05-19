import { Animation, AppState, Patch } from "@/api/dto";
import { applyPatch } from "fast-json-patch";
import { defineStore, acceptHMRUpdate } from "pinia";

export const useAppStore = defineStore("app", {
  state: () =>
    ({
      documents: [],
      currentDocumentPath: null,
    } as AppState),
  actions: {
    patch(patch: Patch) {
      applyPatch(this.$state, patch, false);
    },
  },
  getters: {
    currentDocument(state) {
      for (let document of state.documents) {
        if (document.path == state.currentDocumentPath) {
          return document;
        }
      }
      return null;
    },
    currentAnimation(): Animation | null {
      if (this.currentDocument) {
        for (let animation of this.currentDocument.sheet.animations) {
          if (
            animation.name == this.currentDocument.view.currentAnimationName
          ) {
            return animation;
          }
        }
      }
      return null;
    },
  },
});

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useAppStore, import.meta.hot));
}
