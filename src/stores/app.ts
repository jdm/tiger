import { Animation, AppState, Patch, Sequence } from "@/api/dto";
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
    sortedAnimations(): Animation[] | null {
      if (!this.currentDocument?.sheet.animations) {
        return null;
      }
      let animations = Object.values(this.currentDocument.sheet.animations);
      animations.sort((a, b) => {
        const nameA = a.name.toLowerCase();
        const nameB = b.name.toLowerCase();
        if (nameA < nameB) {
          return -1;
        }
        if (nameA > nameB) {
          return 1;
        }
        return 0;
      });
      return animations;
    },
    currentAnimation(): Animation | null {
      if (this.currentDocument?.currentAnimationName) {
        return this.currentDocument.sheet.animations[
          this.currentDocument.currentAnimationName
        ];
      }
      return null;
    },
    currentSequence(): Sequence | null {
      if (this.currentDocument?.currentSequenceDirection) {
        if (this.currentAnimation) {
          return this.currentAnimation.sequences[
            this.currentDocument.currentSequenceDirection
          ];
        }
      }
      return null;
    },
  },
});

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useAppStore, import.meta.hot));
}
