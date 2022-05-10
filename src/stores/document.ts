import { defineStore, acceptHMRUpdate } from "pinia";

interface Sheet {}

interface View {}

export const useDocumentStore = defineStore("document", {
  state: () => ({
    source: "something" as null | String,
    sheet: null as null | Sheet,
    view: null as null | View,
  }),
});

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useDocumentStore, import.meta.hot));
}
