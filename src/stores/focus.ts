import { defineStore, acceptHMRUpdate } from "pinia";

export const useFocusStore = defineStore("focus", {
  state: () => {
    return {
      trapCount: 0,
    };
  },
  actions: {
    trapInput() {
      this.trapCount += 1;
    },
    freeInput() {
      this.trapCount -= 1;
    },
  },
  getters: {
    isInputTrapped: (state) => state.trapCount > 0,
  },
});

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useFocusStore, import.meta.hot));
}
